use structopt::StructOpt;

#[macro_use]
mod day;
use day::{four::Four, one::One, three::Three, two::Two, Day};

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
        opt.days = vec![1, 2, 3, 4];
    }

    opt.days.into_iter().for_each(|day| match day {
        1 => run!(One),
        2 => run!(Two),
        3 => run!(Three),
        4 => run!(Four),
        _ => eprintln!("That day doesn't exist in the calendar or hasn't passed yet!"),
    });
}
