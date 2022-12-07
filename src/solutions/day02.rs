use std::error::Error;
use crate::solver::{ReadExt, Solver};
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Command>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut pos = 0;
        let mut depth = 0;

        for c in input {
            match c {
                Command::Forward(n) => pos += n,
                Command::Up(n) => depth -= n,
                Command::Down(n) => depth += n
            }
        }
        pos * depth
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut pos = 0;
        let mut depth = 0;
        let mut aim = 0;

        for c in input {
            match c {
                Command::Forward(n) => {pos += n; depth += aim*n},
                Command::Up(n) => aim -= n,
                Command::Down(n) => aim += n
            }
        }
        pos * depth
    }
}

pub enum Command{
    Forward(u32),
    Down(u32),
    Up(u32)
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, n_string) = s.split_once(" ").unwrap();
        let n = n_string.parse()?;

        match command {
            "forward" => Ok(Command::Forward(n)),
            "up" => Ok(Command::Up(n)),
            "down" => Ok(Command::Down(n)),
            _ => Err("couldn't parse".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_from_str() {
        let u = Command::from_str("up 6").unwrap();
        assert!(matches!(u, Command::Up(6)));
        let result = Command::from_str("forward ");
        assert!(result.is_err());
    }
}