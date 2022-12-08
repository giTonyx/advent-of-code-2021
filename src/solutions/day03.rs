use std::collections::HashMap;
use crate::solver::{ReadExt, Solver};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let bitMap = input
            .iter()
            .map(|x| {
                x.chars()
                    .enumerate()
                    .filter(|&(_, bit)| bit == '1')
                    .map(|(idx,_)| idx) })
            .fold(HashMap::new(), |mut acc:HashMap<usize, usize>, v| {
                v.for_each(|idx| { acc.insert(idx, 1 + if acc.contains_key(&idx) { acc[&idx] } else {0}); });
                acc
            });

        let mut gamma = String::new();
        let mut epsilon = String::new();

        let quorum = input.len() / 2;
        for i in 0..input[0].len() {
            if bitMap.contains_key(&i) && bitMap[&i] > quorum {
                gamma.push('1');
                epsilon.push('0');
            } else {
                gamma.push('0');
                epsilon.push('1');
            }
        }

        let gamma = u64::from_str_radix(&gamma, 2).unwrap_or_default();
        let epsilon = u64::from_str_radix(&epsilon, 2).unwrap_or_default();
        gamma * epsilon
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let oxygen = find_rating(input, true);
        let scrubber = find_rating(input, false);
        oxygen * scrubber
    }
}

fn find_rating(numbers: &Vec<String>, most_common: bool) -> u64 {
    let mut current_numbers = numbers.clone();
    let mut bit = 0;
    while current_numbers.len() > 1 {
        current_numbers = filter_on_bit_commonality(&current_numbers, bit, most_common);
        bit += 1;
    }
    u64::from_str_radix(&current_numbers[0], 2).unwrap_or_default()
}

fn filter_on_bit_commonality(numbers: &[String], bit: usize, most_common: bool) -> Vec<String>{
    let one_count = numbers.iter().map(|x| x.chars().nth(bit).unwrap()).filter(|&x| x == '1').collect::<Vec<char>>().len();
    let zero_count = numbers.len() - one_count;

    let target_char = match (one_count > zero_count, one_count == zero_count, most_common) {
        (_, true, true) => '1',
        (_, true, false) => '0',
        (true, _, true) => '1',
        (true, _, false) => '0',
        (_, _, true) => '0',
        (_, _, _) => '1'
    };
    numbers.iter().filter(|&x| x.chars().nth(bit).unwrap() == target_char).cloned().collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        let base_input = ["00100","11110","10110","10111","10101","01111","00111","11100","10000","11001","00010","01010"].map(String::from).to_vec();
        let true_rating = find_rating(&base_input, true);
        assert!(true_rating == 23);
        let false_rating = find_rating(&base_input, false);
        assert!(false_rating == 10);
    }
}