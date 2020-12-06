pub mod five;
pub mod four;
pub mod one;
pub mod six;
pub mod three;
pub mod two;

pub trait Day {
    type Input;
    type Output;

    fn part_one(input: &Self::Input) -> Self::Output;
    fn part_two(input: &Self::Input) -> Self::Output;
    fn get_input() -> Self::Input;
}

macro_rules! time {
    ($e:expr) => {{
        let start = std::time::Instant::now();
        let res = $e;
        let elapsed = start.elapsed().as_nanos();
        (res, elapsed)
    }};
}

macro_rules! run {
    ($( $t:ty ),*) => {
        $(
            {
                let input = <$t>::get_input();

                println!("\nDay {}\n--------------------", stringify!($t));

                let (results, elapsed) = time!(<$t>::part_one(&input));
                println!("Results for Part One: {:>10} (time: {:>6}ns)", results, elapsed);

                let (results, elapsed) = time!(<$t>::part_two(&input));
                println!("Results for Part Two: {:>10} (time: {:>6}ns)", results, elapsed);

                println!("--------------------");
            }
        )*
    };
}
