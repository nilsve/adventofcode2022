use lazy_static::lazy_static;

use std::collections::HashMap;
use std::io::Read;
use std::str::from_utf8;

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("one", 1);
        m.insert("two", 2);
        m.insert("three", 3);
        m.insert("four", 4);
        m.insert("five", 5);
        m.insert("six", 6);
        m.insert("seven", 7);
        m.insert("eight", 8);
        m.insert("nine", 9);
        m
    };
}

fn get_spelled<T: AsRef<str>>(input: T) -> Option<u32> {
    let input = input.as_ref();

    HASHMAP.iter().find_map(|(spelled, val)| {
        if input.starts_with(spelled) {
            return Some(*val);
        }

        return None;
    })
}

// fn get_numbers(line: &str) -> Vec<u32> {
//     let mut spelled = "".to_string();
//
//     line.chars()
//         .filter_map(|character| {
//             if character.is_numeric() {
//                 spelled = "".to_string();
//                 return Some(character.to_digit(10).unwrap());
//             } else {
//                 spelled = format!("{}{}", spelled, character);
//
//                 if let Some(spelled_num) = get_spelled(&spelled) {
//                     spelled = "".to_string();
//                     return Some(spelled_num);
//                 }
//             }
//
//             None
//         })
//         .collect()
// }

// fn get_numbers(line: &str) -> Vec<u32> {
//     let chars: Vec<char> = line.chars().collect();
//
//     chars
//         .windows(5)
//         .filter_map(|possible_word| {
//             println!("{:?}", possible_word);
//             // Convert possible_word char slice to string reference without allocating
//             if possible_word.get(0).unwrap().is_numeric() {
//                 return Some(possible_word.get(0).unwrap().to_digit(10).unwrap());
//             }
//
//             if let Some(val) = get_spelled(possible_word.iter().collect::<String>()) {
//                 return Some(val);
//             }
//
//             return None;
//         })
//         .collect()
// }

fn get_numbers(line: &str) -> Vec<u32> {
    line.chars()
        .enumerate()
        .filter_map(|(index, _)| {
            let possible_word = &line[index..];

            // Check if the slice has at least one character and the first character is numeric
            if let Some(first_char) = possible_word.chars().next() {
                if first_char.is_numeric() {
                    return Some(first_char.to_digit(10).unwrap());
                }
            }

            if let Some(val) = get_spelled(possible_word) {
                return Some(val);
            }

            return None;
        })
        .collect()
}

fn calculate(input: &str) -> usize {
    input.split("\n").fold(0, |acc, line| {
        let numbers = get_numbers(&line);

        let mut total = 0;

        if numbers.len() == 0 {
            return acc;
        }

        if numbers.len() == 1 {
            let num = numbers.get(0).unwrap();
            total = format!("{}{}", num, num).parse::<usize>().unwrap();
        } else if numbers.len() > 1 {
            let num1 = numbers.get(0).unwrap();
            let num2 = numbers.get(numbers.len() - 1).unwrap();

            total = format!("{}{}", num1, num2).parse::<usize>().unwrap();
        }

        acc + total
    })
}

#[cfg(test)]
mod day1_tests {
    use crate::questions_2023::day1::calculate;

    #[test]
    fn test_input_challenge_1() {
        let input = include_str!("test_input.txt");
        let result = calculate(input);

        assert_eq!(result, 142);
    }

    #[test]
    fn input_challenge_1() {
        let input = include_str!("input.txt");
        let result = calculate(input);

        println!("{}", result)
    }

    #[test]
    fn test_input_challenge_2() {
        let input = include_str!("test_input_2.txt");
        let result = calculate(input);

        assert_eq!(result, 281);
    }
}
