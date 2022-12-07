use crate::solver::{ReadExt, Solver};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u32>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut total:usize = 0;
        for idx in 1..input.len(){
            if input[idx] > input[idx-1] {
                total += 1;
            }
        }
        total
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut total:usize = 0;
        for idx in 3..input.len(){
            let first_window = input[idx-3] + input[idx-2] + input[idx-1];
            let second_window = input[idx-2] + input[idx-1] + input[idx];
            if second_window > first_window {
                total += 1;
            }
        }
        total
    }
}
