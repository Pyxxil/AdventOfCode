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
