use std::collections::{HashMap, HashSet};

fn find_matching_item(input: &[&str]) -> char {
    let first = input.get(0).expect("No first item found");

    let hashmaps: Vec<HashSet<char>> = input
        .iter()
        .skip(1)
        .map(|input_line| {
            let mut found_chars: HashSet<char> = HashSet::new();

            input_line.chars().for_each(|char| {
                if !found_chars.contains(&char) {
                    found_chars.insert(char);
                }
            });

            found_chars
        })
        .collect();

    first
        .chars()
        .find(|chr| {
            let not_in_some_hashmaps = hashmaps.iter().any(|hashmap| !hashmap.contains(&chr));

            !not_in_some_hashmaps
        })
        .expect(&format!("No matching character found {}", first))
}

fn calculate_item_score(chr: char) -> u32 {
    let result;
    if chr >= 'a' && chr <= 'z' {
        result = chr as u32 - 'a' as u32;
    } else {
        result = (chr as u32 - 'A' as u32) + 26;
    }

    result + 1
}

fn challenge1(input: &str) -> usize {
    input
        .split('\n')
        .filter_map(|backpack| {
            if backpack.is_empty() {
                return None;
            }

            let comp1 = &backpack[0..backpack.len() / 2];
            let comp2 = &backpack[backpack.len() / 2..backpack.len()];

            let item = find_matching_item(&[comp1, comp2]);
            let score = calculate_item_score(item);

            Some(score as usize)
        })
        .sum()
}

fn challenge2(input: &str) -> usize {
    input
        .split('\n')
        .map(|str| str.to_owned())
        .collect::<Vec<String>>()
        .chunks(3)
        .filter_map(|backpacks| {
            if backpacks.is_empty() {
                return None;
            }

            let backpack1 = backpacks.get(0).unwrap();

            if backpack1.is_empty() {
                return None;
            }
            let backpack2 = backpacks.get(1).unwrap();
            let backpack3 = backpacks.get(2).unwrap();

            println!(
                "collection: {} \n{} \n{}\n\n",
                backpack1, backpack2, backpack3
            );

            let item = find_matching_item(&[backpack1, backpack2, backpack3]);

            Some(calculate_item_score(item) as usize)
        })
        .sum()
}

#[cfg(test)]
mod test_day_3 {
    use crate::day3::{challenge1, challenge2};

    #[test]
    fn calculate_challenge1_test_input() {
        let input = include_str!("test_input.txt");
        let sum = challenge1(input);
        assert_eq!(sum, 157);
    }

    #[test]
    fn get_challenge1_output() {
        let input = include_str!("input.txt");
        let sum = challenge1(input);

        println!("{}", sum)
    }

    #[test]
    fn calculate_challenge2_test_input() {
        let input = include_str!("test_input.txt");
        let sum = challenge2(input);
        assert_eq!(sum, 70);
    }

    #[test]
    fn calculate_challenge2() {
        let input = include_str!("input.txt");
        let sum = challenge2(input);

        println!("{}", sum);
    }

    // #[test]
    // fn get_challenge1_output() {
    //     let input = include_str!("input.txt");
    //     let sum = parse(input);
    //
    //     println!("{}", sum)
    // }
}
