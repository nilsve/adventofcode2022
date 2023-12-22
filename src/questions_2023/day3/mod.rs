use std::collections::HashMap;

fn is_adjacent(coord: (usize, usize), input: &Vec<Vec<char>>) -> (bool, Option<(usize, usize)>) {
    let coord = (coord.0 as isize, coord.1 as isize);
    let adjacent_coords = vec![
        (coord.0 - 1, coord.1 - 1),
        (coord.0 - 1, coord.1),
        (coord.0 - 1, coord.1 + 1),
        (coord.0, coord.1 - 1),
        (coord.0, coord.1 + 1),
        (coord.0 + 1, coord.1 - 1),
        (coord.0 + 1, coord.1),
        (coord.0 + 1, coord.1 + 1),
    ];

    let mut gear_coord: Option<(usize, usize)> = None;

    let result = adjacent_coords.iter().any(|&(x, y)| {
        if let Some(Some(character)) = input.get(y as usize).map(|row| row.get(x as usize)) {
            if !character.is_numeric() && *character != '.' {
                if *character == '*' {
                    gear_coord = Some((x as usize, y as usize));
                }

                return true;
            }
        }

        false
    });

    (result, gear_coord)
}

fn calculate(input: &str) -> (usize, usize) {
    let input = input
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    input.iter().enumerate().for_each(|(y, row)| {
        let mut number = String::new();
        let mut number_is_adjecent = false;
        let mut gear_coord: Option<(usize, usize)> = None;

        row.iter().enumerate().for_each(|(x, character)| {
            if character.is_numeric() {
                number.push(*character);
            } else {
                if number.len() > 0 {
                    if number_is_adjecent {
                        let total = number.parse::<usize>().unwrap();
                        gear_coord.map(|coord| {
                            gears.entry(coord).or_insert(Vec::new()).push(total);
                        });

                        sum += total;
                        println!("{} is adjacent", number);
                    }
                    number_is_adjecent = false;
                    number.clear();
                    gear_coord = None;
                }
            }

            if !number_is_adjecent && number.len() > 0 {
                let (is_adjacent, found_gear_coord) = is_adjacent((x, y), &input);

                if is_adjacent {
                    number_is_adjecent = true;
                }
                if found_gear_coord.is_some() {
                    gear_coord = found_gear_coord;
                }
            }
        });

        if number_is_adjecent {
            sum += number.parse::<usize>().unwrap();
            println!("{} is adjacent", number);
        }
    });

    let gear_total = gears
        .iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .map(|(_, numbers)| numbers.get(0).unwrap() * numbers.get(1).unwrap())
        .reduce(|a, b| a + b)
        .unwrap();

    (sum, gear_total)
}

#[cfg(test)]
mod day1_tests {
    use crate::questions_2023::day3::calculate;

    #[test]
    fn test_input_challenge_1() {
        let input = include_str!("test_input.txt");
        let result = calculate(input);

        assert_eq!(result, (4361, 467835));
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
