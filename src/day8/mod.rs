fn challenge_1(input: &str) -> usize {
    let grid = build_grid(input);

    let mut edge_visible = (grid.len() - 1) * 2;
    edge_visible = edge_visible + ((grid.get(0).unwrap().len() - 1) * 2);

    let mut result = 0;

    for y in 1..grid.len() - 1 {
        let row = grid.get(y).unwrap();
        for x in 1..row.len() - 1 {
            result += calculate_point(x, y, &grid, false);
        }
    }
    edge_visible + result
}

fn challenge_2(input: &str) -> usize {
    let grid = build_grid(input);

    let mut highest = 0;
    for y in 1..grid.len() - 1 {
        let row = grid.get(y).unwrap();
        for x in 1..row.len() - 1 {
            let result = calculate_point(x, y, &grid, true);
            if result > highest {
                highest = result;
            }
        }
    }
    highest
}

fn build_grid(input: &str) -> Vec<Vec<usize>> {
    let grid: Vec<Vec<usize>> = input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            Some(
                line.chars()
                    .map(|char| char.to_string().parse::<usize>().unwrap())
                    .collect(),
            )
        })
        .collect();
    grid
}

fn is_visible_in_line(line: &[usize], index: usize) -> bool {
    let our_value = line.get(index).unwrap();

    let mut highest = true;
    for pos in 0..line.len() {
        let value = line.get(pos).unwrap();

        if pos == index {
            if highest {
                return true;
            }

            highest = true;
        } else if value >= our_value {
            highest = false;
        }
    }

    highest
}

fn calculate_viewing_score(line: &[usize], index: usize) -> usize {
    let our_value = line.get(index).unwrap();

    let mut left = line[0..index].to_owned();
    left.reverse();
    let right = line[index + 1..line.len()].to_vec();
    let mut total_left: usize = 0;
    let mut total_right: usize = 0;

    for x in left {
        total_left += 1;
        if x >= *our_value {
            break;
        }
    }

    for x in right {
        total_right += 1;
        if x >= *our_value {
            break;
        }
    }

    total_left * total_right
}

fn calculate_point(x: usize, y: usize, grid: &Vec<Vec<usize>>, challenge_2: bool) -> usize {
    let horizontal_line = grid.get(y).unwrap();
    let vertical_line: Vec<usize> = grid
        .iter()
        .map(|horizontal_line| *horizontal_line.get(x).expect(&format!("Couldnt get {}", x)))
        .collect();

    if challenge_2 {
        return calculate_viewing_score(horizontal_line, x)
            * calculate_viewing_score(&vertical_line, y);
    } else if is_visible_in_line(horizontal_line, x) || is_visible_in_line(&vertical_line, y) {
        return 1;
    }

    return 0;
}

#[cfg(test)]
mod day_8_tests {
    use crate::day8::{challenge_1, challenge_2, is_visible_in_line};

    #[test]
    fn test_challenge1_output() {
        let input = include_str!("test_input.txt");
        let result = challenge_1(input);

        assert_eq!(result, 21);
    }

    #[test]
    fn test_challenge2_output() {
        let input = include_str!("test_input.txt");
        let result = challenge_2(input);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_line() {
        assert_eq!(is_visible_in_line(&[5, 2, 4, 2, 5], 2), false);
        assert_eq!(is_visible_in_line(&[1, 2, 4, 2, 5], 2), true);
        assert_eq!(is_visible_in_line(&[1, 2, 3, 4, 5], 0), true);
        assert_eq!(is_visible_in_line(&[5, 2, 4, 6], 2), false);
    }

    #[test]
    fn calculate_challenge1_output() {
        let input = include_str!("input.txt");
        let result = challenge_1(input);

        println!("{}", result)
    }

    #[test]
    fn calculate_challenge2_output() {
        let input = include_str!("input.txt");
        let result = challenge_2(input);

        println!("{}", result)
    }
}
