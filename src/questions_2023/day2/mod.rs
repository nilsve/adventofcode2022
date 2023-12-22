const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn calculate(input: &str) -> (usize, usize) {
    input
        .split("\n")
        .into_iter()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let mut split = line.split(": ");

            let x = split.next().unwrap();
            let mut game_id = x.replace("Game ", "").parse::<usize>().unwrap();

            let next = split.next().unwrap();

            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            for set in next.split("; ") {
                for hand in set.split(", ") {
                    let mut hand = hand.split(' ');
                    let total = hand.next().unwrap().parse::<usize>().unwrap();
                    let color = hand.next().unwrap();

                    match color {
                        "red" => {
                            if total > min_red {
                                min_red = total;
                            }
                            if total > MAX_RED {
                                game_id = 0;
                            }
                        }
                        "green" => {
                            if total > min_green {
                                min_green = total;
                            }
                            if total > MAX_GREEN {
                                game_id = 0;
                            }
                        }
                        "blue" => {
                            if total > min_blue {
                                min_blue = total;
                            }
                            if total > MAX_BLUE {
                                game_id = 0;
                            }
                        }
                        _ => {
                            panic!("Unknown color");
                        }
                    }
                }
            }

            Some((game_id, min_blue * min_green * min_red))
        })
        .fold((0, 0), |(game_ids, minimums), (game_id, minimum)| {
            (game_ids + game_id, minimums + minimum)
        })
}

#[cfg(test)]
mod day1_tests {
    use crate::questions_2023::day2::calculate;

    #[test]
    fn test_input_challenge_1() {
        let input = include_str!("test_input.txt");
        let result = calculate(input);

        assert_eq!(result, (8, 2286));
    }

    #[test]
    fn input_challenge_1() {
        let input = include_str!("input.txt");
        let result = calculate(input);

        println!("{:?}", result)
    }
    //
    // #[test]
    // fn test_input_challenge_2() {
    //     let input = include_str!("test_input_2.txt");
    //     let result = calculate(input);
    //
    //     assert_eq!(result, 281);
    // }
}
