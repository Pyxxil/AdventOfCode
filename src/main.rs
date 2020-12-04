#[macro_use]
mod day;
use day::{one::One, three::Three, two::Two, Day};

fn main() {
    run!(One, Two, Three);
}
