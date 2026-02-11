use qtty::Seconds;

pub fn build_samples() -> Vec<Seconds> {
    qtty::qtty_vec!(vec Seconds; 1.0, 2.0, 3.0)
}
