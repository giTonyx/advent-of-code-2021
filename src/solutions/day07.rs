use crate::solver::Solver;
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = ();
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {}

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        0
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        0
    }
}
