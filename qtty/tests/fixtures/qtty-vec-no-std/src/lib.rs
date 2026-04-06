#![no_std]

use qtty::Second;

pub fn build_samples() {
    let _ = qtty::qtty_vec!(vec Second; 1.0, 2.0, 3.0);
}
