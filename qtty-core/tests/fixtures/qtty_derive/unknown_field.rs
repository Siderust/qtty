#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, qtty_derive::Unit)]
#[unit(
    crate = qtty_core,
    symbol = "bad",
    dimension = qtty_core::Length,
    ratio = 1.0,
    unknown = "field"
)]
pub struct UnknownField;

fn main() {}
