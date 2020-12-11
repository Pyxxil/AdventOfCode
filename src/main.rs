#![feature(str_split_once)]
#![feature(iterator_fold_self)]
use structopt::StructOpt;

#[macro_use]
mod day;
use day::{
    eight::Eight, eleven::Eleven, five::Five, four::Four, nine::Nine, one::One, seven::Seven,
    six::Six, ten::Ten, three::Three, two::Two, Day,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "calendar", about = "An Advent of Code Calendar")]
struct Opt {
    /// The Days to display
    #[structopt(short, long = "day")]
    days: Vec<u64>,
}

fn main() {
    let mut opt = Opt::from_args();

    if opt.days.is_empty() {
        opt.days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    }

    opt.days.into_iter().for_each(|day| match day {
        1 => run!(One),
        2 => run!(Two),
        3 => run!(Three),
        4 => run!(Four),
        5 => run!(Five),
        6 => run!(Six),
        7 => run!(Seven),
        8 => run!(Eight),
        9 => run!(Nine),
        10 => run!(Ten),
        11 => run!(Eleven),
        _ => eprintln!("That day doesn't exist in the calendar or hasn't passed yet!"),
    });
}
