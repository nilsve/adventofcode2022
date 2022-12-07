#[no_mangle]
fn parse_input(str: &str) -> Vec<usize> {
    let mut result: Vec<usize> = str
        .split("\n\n")
        .map(|elf_calorie_lines: &str| {
            elf_calorie_lines
                .split('\n')
                .filter_map(|calories: &str| match calories.is_empty() {
                    true => None,
                    false => Some(calories.parse::<usize>().unwrap()),
                })
                .sum()
        })
        .collect::<Vec<usize>>();

    result.sort();
    result.reverse();

    result
}

#[cfg(test)]
mod day1_test_performance {
    use crate::day1_faster::parse_input;
    use benchmark_simple::{Bench, Options};

    #[test]
    fn get_top_3() {
        let bench = Bench::new();
        let options = Options::default();
        let res = bench.run(&options, run);
        println!("result: {} nano seconds", res.as_ns());
    }

    #[no_mangle]
    fn run() {
        let input = include_str!("input.txt");
        parse_input(input);
    }
}
