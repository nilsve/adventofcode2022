use regex::Regex;

fn calculate(input: &str) -> usize {
    let mut input = format!("do(){input}");

    loop {
        if let Some(dont_index) = input.find("don't()") {
            let do_index = input[dont_index..].find("do()").unwrap_or(input[dont_index..].len());

            input.replace_range(&dont_index..&(do_index+dont_index), &"");

            println!("replaced")
        } else {
            println!("Done");
            break;
        }
    }


    let mul_regex = Regex::new("mul\\([0-9]{1,3}\\,[0-9]{1,3}\\)").unwrap();

    mul_regex.find_iter(&input).map(|matches| {
        let operation = matches.as_str();

        let parts: Vec<&str> = operation.split(",").collect();
        let first = parts.get(0).unwrap().replace("mul(", "").parse::<usize>().unwrap();
        let second = parts.get(1).unwrap().replace(")", "").parse::<usize>().unwrap();

        first * second
    }).sum()
}

#[cfg(test)]
mod day3_tests {
    use crate::questions_2024::day3::calculate;

    #[test]
    fn test_input_challenge_1() {
        let input = include_str!("test_input.txt");
        let result = calculate(input);

        assert_eq!(result, 48);
    }

    #[test]
    fn input_challenge_1() {
        let input = include_str!("input.txt");
        let result = calculate(input);

        println!("{result}")
    }
}