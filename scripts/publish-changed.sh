#!/usr/bin/env bash
# publish-changed.sh — publish only changed, publishable workspace packages.
#
# Usage:
#   ./scripts/publish-changed.sh [--confirm-ffi] [--dry-run]
#
# Options:
#   --confirm-ffi   Allow publishing FFI crates (those named *-ffi or in an ffi/
#                   directory). Absent by default; required for any FFI publish.
#   --dry-run       Print the cargo publish commands without executing them.
#
# Environment:
#   PUBLISH_BASE_TAG  Git tag to diff against. Defaults to the most recent
#                     reachable release tag (v[0-9]*) that is an ancestor of HEAD.
#                     On an exact-tag build, uses the tag *before* the current one.
#   CARGO_REGISTRY_TOKEN  Required for actual publishing (cargo publish reads it).
#
# Exit codes:
#   0  All publishable changed packages published (or dry-run complete).
#   1  A soundness TODO was found in an FFI crate being published.
#   2  --confirm-ffi was not passed but an FFI crate would be published.

set -euo pipefail

CONFIRM_FFI=false
DRY_RUN=false
for arg in "$@"; do
    case "$arg" in
        --confirm-ffi) CONFIRM_FFI=true ;;
        --dry-run)     DRY_RUN=true ;;
    esac
done

# ── Determine base tag ───────────────────────────────────────────────────────
CURRENT_TAG=$(git describe --exact-match --tags HEAD 2>/dev/null || true)

if [[ -n "${PUBLISH_BASE_TAG:-}" ]]; then
    BASE_TAG="$PUBLISH_BASE_TAG"
elif [[ -n "$CURRENT_TAG" ]]; then
    # Exact-tag build: use the tag before this one
    BASE_TAG=$(git tag --sort=-version:refname | grep -E '^v[0-9]' | grep -v "^${CURRENT_TAG}$" | head -1)
else
    BASE_TAG=$(git describe --abbrev=0 --tags --match 'v[0-9]*' 2>/dev/null || git rev-list --max-parents=0 HEAD)
fi

echo "Diffing against base: ${BASE_TAG}"

# ── Collect changed files since base tag ────────────────────────────────────
CHANGED_FILES=$(git diff --name-only "${BASE_TAG}"..HEAD 2>/dev/null || true)
if [[ -z "$CHANGED_FILES" ]]; then
    echo "No changed files since ${BASE_TAG}. Nothing to publish."
    exit 0
fi

# ── Map changed paths to workspace packages ─────────────────────────────────
MANIFEST_JSON=$(cargo metadata --no-deps --format-version 1)
ORDERED_PACKAGES=$(CHANGED_FILES="$CHANGED_FILES" MANIFEST_JSON="$MANIFEST_JSON" python3 - <<'PY'
import json
import os
from pathlib import Path

data = json.loads(os.environ["MANIFEST_JSON"])
changed_files = set(os.environ["CHANGED_FILES"].splitlines())
workspace_root = Path.cwd().resolve()
root_manifest_changed = "Cargo.toml" in changed_files

packages = data["packages"]
by_name = {pkg["name"]: pkg for pkg in packages}
workspace_names = set(by_name)

deps = {pkg["name"]: set() for pkg in packages}
for pkg in packages:
    for dep in pkg.get("dependencies", []):
        # Publishing only needs normal/build workspace dependencies ordered
        # first. Dev-dependencies are excluded to avoid false cycles in crates
        # that test against another workspace member.
        if dep.get("kind") == "dev":
            continue
        if dep.get("path") and dep["name"] in workspace_names:
            deps[pkg["name"]].add(dep["name"])

ordered = []
state = {}


def visit(name):
    status = state.get(name)
    if status == "visiting":
        raise SystemExit(f"workspace dependency cycle involving {name}")
    if status == "visited":
        return
    state[name] = "visiting"
    for dep_name in sorted(deps[name]):
        visit(dep_name)
    state[name] = "visited"
    ordered.append(name)


for pkg in packages:
    visit(pkg["name"])

for name in ordered:
    pkg = by_name[name]
    manifest = Path(pkg["manifest_path"]).resolve()
    pkg_dir = manifest.parent
    pkg_rel = str(pkg_dir.relative_to(workspace_root))
    changed = root_manifest_changed or any(
        path == pkg_rel or path.startswith(pkg_rel + "/") for path in changed_files
    )
    print(
        json.dumps(
            {
                "name": pkg["name"],
                "manifest_path": pkg["manifest_path"],
                "publish": pkg.get("publish"),
                "changed": changed,
            }
        )
    )
PY
)

is_ffi_crate() {
    local name="$1" manifest="$2"
    [[ "$name" == *-ffi ]] || [[ "$manifest" == */ffi/* ]]
}

has_soundness_todo() {
    local manifest="$1"
    local src_dir
    src_dir=$(dirname "$manifest")/src
    if [[ -d "$src_dir" ]]; then
        grep -rq 'TODO: justify soundness' "$src_dir" 2>/dev/null && return 0
    fi
    return 1
}

PUBLISHED=0
SKIPPED=0

while IFS= read -r pkg_json; do
    name=$(echo "$pkg_json" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d['name'])")
    manifest=$(echo "$pkg_json" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d['manifest_path'])")
    publish=$(echo "$pkg_json" | python3 -c "
import sys, json
d = json.load(sys.stdin)
p = d.get('publish')
# publish: null means publishable; publish: [] means publish=false
print('false' if p is not None and len(p) == 0 else 'true')
")
    changed=$(echo "$pkg_json" | python3 -c "import sys,json; d=json.load(sys.stdin); print('true' if d['changed'] else 'false')")

    # Skip non-publishable crates
    if [[ "$publish" == "false" ]]; then
        echo "SKIP (publish=false): $name"
        ((SKIPPED++)) || true
        continue
    fi

    # Check if the package changed. Root Cargo.toml changes count for every
    # package because workspace-inherited fields such as package version can
    # change without touching the package directory.
    if [[ "$changed" != "true" ]]; then
        echo "SKIP (unchanged): $name"
        ((SKIPPED++)) || true
        continue
    fi

    # FFI gate
    if is_ffi_crate "$name" "$manifest"; then
        if [[ "$CONFIRM_FFI" != "true" ]]; then
            echo "ERROR: $name is an FFI crate. Pass --confirm-ffi to allow publishing FFI crates." >&2
            exit 2
        fi
        if has_soundness_todo "$manifest"; then
            echo "ERROR: $name has unsound unsafe blocks (TODO: justify soundness). Resolve before publishing." >&2
            exit 1
        fi
    fi

    cmd="cargo publish -p $name --no-verify"
    if [[ "$DRY_RUN" == "true" ]]; then
        echo "DRY-RUN: $cmd"
    else
        echo "Publishing: $name"
        eval "$cmd"
        sleep 30
    fi
    ((PUBLISHED++)) || true

done < <(echo "$ORDERED_PACKAGES")

echo ""
echo "Done. Published: ${PUBLISHED}, Skipped: ${SKIPPED}."
