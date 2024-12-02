use crate::questions_2024::day2::Following::{Decreasing, Increasing};

fn calculate(input: &str, enable_dampener: bool) -> usize {
    let reports = input.split("\n").collect::<Vec<&str>>();

    let safe_reports: Vec<&str> = reports.into_iter().filter(|report| {
        let report: Vec<u32> = report.split(" ").map(|chr| chr.parse::<u32>().unwrap()).collect();

        let is_correct = record_is_correct(&report);

        if is_correct {
            return true;
        }

        if enable_dampener {
            for i in 0..report.len() {
                let dampened_report: Vec<u32> = report.iter().enumerate().filter(|(index, _)| *index != i).map(|(_, val) | *val).collect();

                if record_is_correct(&dampened_report) {
                    return true;
                }
            }
        }

        false

    }).collect();

    safe_reports.len()
}

#[derive(Eq, PartialEq)]
enum Following {
    Decreasing,
    Increasing,
}

fn record_is_correct(record: &[u32]) -> bool {
    let first = *record.get(0).unwrap();
    let second = *record.get(1).unwrap();

    let following = if first > second {
        Decreasing
    } else if first < second {
        Increasing
    } else {
        return false
    };

    for window in record.windows(2) {
        let first = *window.get(0).unwrap();
        let second = *window.get(1).unwrap();

        let delta = first.abs_diff(second);

        if delta < 1 || delta > 3 {
            return false;
        }

        match following {
            Decreasing => {
                if first < second {
                    return false;
                }
            }
            Increasing => {
                if first > second {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod day2_tests {
    use crate::questions_2024::day2::calculate;

    #[test]
    fn test_input_challenge_1() {
        let input = include_str!("test_input.txt");
        let result = calculate(input, false);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_challenge_1() {
        let input = include_str!("input.txt");

        // Calculate the time for the following call
        let current_time = std::time::Instant::now();
        let result = calculate(input, false);

        println!("Time: {:?}", current_time.elapsed());

        println!("{}", result);
    }

    #[test]
    fn test_input_challenge_2() {
        let input = include_str!("test_input.txt");
        let result = calculate(input, true);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_challenge_2() {
        let input = include_str!("input.txt");
        let result = calculate(input, true);

        println!("{}", result);
    }
}