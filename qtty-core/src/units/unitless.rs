// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

// The `Unitless` type has been removed.
// Same-unit division (`Quantity<U, S> / Quantity<U, S>`) now returns `S` directly.
// Use `quantity.erase_unit_raw()` to strip the unit tag and get the raw scalar.
