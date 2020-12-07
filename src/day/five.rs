use crate::day::Day;

pub struct Five {}

impl Day for Five {
    type Output = usize;
    type Input = Vec<String>;

    ///
    /// Task: The first 7 characters will either be F or B; these specify exactly
    ///       one of the 128 rows on the plane (numbered 0 through 127). Each
    ///       letter tells you which half of a region the given seat is in. Start
    ///       with the whole list of rows; the first letter indicates whether the
    ///       seat is in the front (0 through 63) or the back (64 through 127).
    ///       The next letter indicates which half of that region the seat is in,
    ///       and so on until you're left with exactly one row. The last three
    ///       characters will be either L or R; these specify exactly one of the 8
    ///       columns of seats on the plane (numbered 0 through 7). The same
    ///       process as above proceeds again, this time with only three steps. L
    ///       means to keep the lower half, while R means to keep the upper half.
    ///
    ///       We need to find the seat with the highest ID.
    ///
    fn part_one(passes: &Self::Input) -> Self::Output {
        let (row, column) = passes.iter().fold((0, ""), |(max, st), pass| {
            let row = pass.chars().take(7).enumerate().fold(0, |id, (idx, ch)| {
                if ch == 'B' {
                    id + (1 << (9 - idx))
                } else {
                    id
                }
            });

            if row > max {
                (row, &pass)
            } else {
                (max, st)
            }
        });

        let column = column.chars().skip(7).enumerate().fold(0, |id, (idx, ch)| {
            if ch == 'R' {
                id + (1 << (3 - idx))
            } else {
                id
            }
        });

        row + column
    }

    ///
    /// Task: It's a completely full flight, so your seat should be the only missing
    ///       boarding pass in your list. However, there's a catch: some of the seats
    ///       at the very front and back of the plane don't exist on this aircraft,
    ///       so they'll be missing from your list as well.
    ///
    ///       We need to find our seat, given that the seat with an ID +-1 from us does
    ///       exist.
    ///
    fn part_two(passes: &Self::Input) -> Self::Output {
        let mut ids = passes
            .iter()
            .fold(Vec::with_capacity(passes.len()), |mut ids, pass| {
                ids.push(pass.chars().enumerate().fold(0, |id, (idx, ch)| {
                    if ch == 'B' || ch == 'R' {
                        id + (1 << (9 - idx))
                    } else {
                        id
                    }
                }));
                ids
            });

        ids.sort_unstable();

        *ids.iter()
            .enumerate()
            .skip(1)
            .find(|(idx, id)| ids[idx - 1] != *id - 1)
            .unwrap()
            .1
            - 1
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_five");

        input.lines().map(|line| line.trim().to_string()).collect()
    }
}
