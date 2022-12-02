fn calculate_points(them: &str, us: &str) -> usize {
    match them {
        "A" => {
            match us {
                "X" => 3,
                "Y" => 6,
                _ => 0
            }
        }
        "B" => {
            match us {
                "X" => 0,
                "Y" => 3,
                _ => 6
            }
        }
        "C" => {
            match us {
                "X" => 6,
                "Y" => 0,
                _ => 3
            }
        }
        _ => unimplemented!()
    }
}

fn parse_input_challenge_1(str: &str) -> usize {
    str.split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let mut splitted = line.split(' ').clone();
            let them = splitted.next().unwrap().clone();
            let us = splitted.next().unwrap().clone();

            let match_points = calculate_points(them, us);

            let our_points = match us {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
            };

            Some(match_points + our_points)
        })
        .sum()
}

fn get_winning_input(them: &str) -> &'static str {
    match them {
        "A" => "Y",
        "B" => "Z",
        "C" => "X",
        _ => panic!()
    }
}

fn get_losing_input(them: &str) -> &'static str {
    match them {
        "A" => "Z",
        "B" => "X",
        "C" => "Y",
        _ => panic!()
    }
}

fn get_matching_input(them: &str) -> &'static str {
    match them {
        "A" => "X",
        "B" => "Y",
        "C" => "Z",
        _ => panic!()
    }
}

fn get_input_points(us: &str) -> usize {
    match us {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unimplemented!()
    }
}

fn parse_input_challenge_2(str: &str) -> usize {
    str.split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let mut splitted = line.split(' ').clone();
            let them = splitted.next().unwrap().clone();
            let us = splitted.next().unwrap().clone();

            let (match_points, input) = match us {
                "X" => (0, get_losing_input(them)),
                "Y" => (3, get_matching_input(them)),
                "Z" => (6, get_winning_input(them)),
                _ => panic!()
            };

            let input_points = get_input_points(input);

            Some(match_points + input_points)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::day2::{calculate_points, parse_input_challenge_1, parse_input_challenge_2};

    #[test]
    fn test_match_points() {
        assert_eq!(calculate_points("A", "X"), 3);
        assert_eq!(calculate_points("A", "Y"), 6);
        assert_eq!(calculate_points("A", "Z"), 0);
    }

    #[test]
    fn challenge_1_test() {
        let input = include_str!("test_input.txt");

        let result = parse_input_challenge_1(input);
        assert_eq!(result, 15);
    }

    #[test]
    fn challenge_2_test() {
        let input = include_str!("input.txt");

        let result = parse_input_challenge_2(input);
        assert_eq!(result, 15442);
    }

    #[test]
    fn challenge_1() {
        let input = include_str!("input.txt");

        let result = parse_input_challenge_1(input);
        println!("{}", result);
    }

    #[test]
    fn challenge_2() {
        let input = include_str!("input.txt");

        let result = parse_input_challenge_1(input);
        println!("{}", result);
    }
}
