mod day;

use day::Day;
use day::{one::One, three::Three, two::Two};

macro_rules! run {
    ($t:ty) => {
        println!("\nDay {}\n--------------------", stringify!($t));
        let input = <$t>::get_input();

        let (p1, p2) = (<$t>::part_one(&input), <$t>::part_two(&input));
        <$t>::print_results(p1, p2);
        println!("--------------------");
    };
}

fn main() {
    run!(One);
    run!(Two);
    run!(Three);
}
