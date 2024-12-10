use crate::questions_2024::day7::Operator::{Concatenate, Mul, Plus};
use std::collections::HashMap;
use std::fmt::format;
use std::io::BufRead;

fn calculate(input: &str) -> usize {
    let mut solver = Solver::new();

    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let splitted_line: Vec<&str> = line.split(":").collect();
            let inputs: Vec<usize> = splitted_line
                .get(1)
                .unwrap()
                .split(" ")
                .filter(|input| !input.is_empty())
                .map(|input| input.parse::<usize>().unwrap())
                .collect();

            (
                splitted_line.get(0).unwrap().parse::<usize>().unwrap(),
                inputs,
            )
        })
        .filter(|line| solver.is_line_correct(&line))
        .map(|line| line.0)
        .sum()
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Plus,
    Mul,
    Concatenate,
}

pub struct Solver {
    enum_combination_cache: HashMap<usize, Vec<Vec<Operator>>>,
}

impl Solver {
    fn new() -> Self {
        Self {
            enum_combination_cache: HashMap::new(),
        }
    }

    fn is_line_correct(&mut self, line: &(usize, Vec<usize>)) -> bool {
        let (answer, inputs) = line;

        let combinations =
            self.generate_enum_combinations(&[Plus, Mul, Concatenate], inputs.len() - 1);

        combinations
            .into_iter()
            .find(|combination| Self::apply_operators(&inputs, &combination) == *answer)
            .is_some()
    }

    fn generate_enum_combinations(&mut self, variants: &[Operator], n: usize) -> &[Vec<Operator>] {
        if self.enum_combination_cache.get(&n).is_none() {
            let mut combinations = Vec::new();
            let total_combinations = variants.len().pow(n as u32); // Total combinations = len(variants)^n

            for i in 0..total_combinations {
                let mut combination = Vec::new();
                let mut value = i;

                for _ in 0..n {
                    combination.push(variants[value % variants.len()].clone());
                    value /= variants.len();
                }

                combinations.push(combination);
            }

            self.enum_combination_cache.insert(n, combinations);
        }

        &self.enum_combination_cache.get(&n).unwrap()
    }

    fn apply_operators(inputs: &[usize], operators: &[Operator]) -> usize {
        let mut inputs = inputs.to_owned();
        for i in 0..inputs.len() - 1 {
            let operator = operators.get(i).unwrap();

            let first = *inputs.get(i).unwrap();
            let mut second = inputs.get_mut(i + 1).unwrap();

            *second = match operator {
                Plus => first + *second,
                Mul => first * *second,
                Concatenate => format!("{}{}", first.to_string(), second.to_string())
                    .parse::<usize>()
                    .unwrap(),
            };
        }

        *inputs.get(inputs.len() - 1).unwrap()
    }
}

#[cfg(test)]
mod day7_tests {
    use crate::questions_2024::day7::{calculate, Operator};

    #[test]
    fn test_input_challenge_1() {
        let input = include_str!("test_input.txt");
        let result = calculate(input);

        assert_eq!(result, 3749);
    }

    #[test]
    fn input_challenge_1() {
        let input = include_str!("input.txt");
        let result = calculate(input);

        // 1459904816
        // 25243429947 is te laag
        println!("{result}")
    }
}
