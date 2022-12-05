struct CleanRange(usize, usize);

impl CleanRange {
    fn new(input: &str) -> Self {
        let mut splitted = input.split('-');

        Self {
            0: splitted.next().unwrap().parse::<usize>().unwrap(),
            1: splitted.next().unwrap().parse::<usize>().unwrap(),
        }
    }

    fn has_full_overlap(&self, other: &CleanRange) -> bool {
        self.0 >= other.0 && self.1 <= other.1
    }

    fn has_some_overlap(&self, other: &CleanRange) -> bool {
        (self.0 <= other.0 && self.1 >= other.0) || (self.0 <= other.1 && self.1 >= other.1)
    }
}

fn parse_input(input: &str, challenge_2: bool) -> usize {
    input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }

            let mut splitted = line.split(',');
            let range_1 = CleanRange::new(splitted.next().unwrap());
            let range_2 = CleanRange::new(splitted.next().unwrap());

            if !challenge_2 {
                if range_1.has_full_overlap(&range_2) || range_2.has_full_overlap(&range_1) {
                    Some(1)
                } else {
                    Some(0)
                }
            } else {
                if range_1.has_some_overlap(&range_2) || range_2.has_some_overlap(&range_1) {
                    Some(1)
                } else {
                    Some(0)
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod day4_tests {
    use crate::day4::parse_input;

    #[test]
    fn challenge_1_test_input() {
        let input = include_str!("test_input.txt");
        let result = parse_input(input, false);

        assert_eq!(result, 2);
    }

    #[test]
    fn challenge_2_test_input() {
        let input = include_str!("test_input.txt");
        let result = parse_input(input, true);

        assert_eq!(result, 4);
    }

    #[test]
    fn challenge_1_output() {
        let input = include_str!("input.txt");
        let result = parse_input(input, false);

        println!("{}", result);
    }

    #[test]
    fn challenge_2_output() {
        let input = include_str!("input.txt");
        let result = parse_input(input, true);

        println!("{}", result);
    }
}
