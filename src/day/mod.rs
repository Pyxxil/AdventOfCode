pub mod one;
pub mod three;
pub mod two;

pub trait Day {
    type Input;
    type Output;

    fn part_one(input: &Self::Input) -> Self::Output;
    fn part_two(input: &Self::Input) -> Self::Output;
    fn get_input() -> Self::Input;
    fn print_results(one: Self::Output, two: Self::Output);
}

macro_rules! run {
    ($( $t:ty ),*) => {
        $(
            {
                println!("\nDay {}\n--------------------", stringify!($t));
                let input = <$t>::get_input();

                let (p1, p2) = (<$t>::part_one(&input), <$t>::part_two(&input));
                <$t>::print_results(p1, p2);
                println!("--------------------");
            }
        )*
    };
}
