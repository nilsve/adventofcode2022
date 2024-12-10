use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

fn calculate_overshoot_points(
    a: &Coord,
    b: &Coord,
    grid: &[Vec<char>],
    part_2: bool,
) -> Vec<Coord> {
    if !part_2 {
        return vec![
            calculate_overshoot_point(a, b),
            calculate_overshoot_point(b, a),
        ];
    }

    let width = grid.len();
    let height = grid.get(0).unwrap().len();

    let mut l_to_r = calculate_overshoot_point_continuous(a, b, width as i32, height as i32);
    let r_to_l = calculate_overshoot_point_continuous(b, a, width as i32, height as i32);

    l_to_r.extend(r_to_l);

    l_to_r
}

fn calculate_overshoot_point(a: &Coord, b: &Coord) -> Coord {
    let delta_x = b.x - a.x;
    let delta_y = b.y - a.y;

    Coord {
        x: b.x + delta_x,
        y: b.y + delta_y,
    }
}
fn calculate_overshoot_point_continuous(
    a: &Coord,
    b: &Coord,
    width: i32,
    height: i32,
) -> Vec<Coord> {
    let mut result = Vec::new();

    result.push(a.clone());
    result.push(b.clone());

    loop {
        let next = calculate_overshoot_point(
            result.get(result.len() - 2).unwrap(),
            result.get(result.len() - 1).unwrap(),
        );

        if next.y >= 0 && next.x >= 0 && next.x < width && next.y < height {
            result.push(next);
        } else {
            break;
        }
    }

    result
}

fn find_all_char_positions(chr: char, grid: &[Vec<char>]) -> Vec<Coord> {
    let mut result: Vec<Coord> = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid.get(0).unwrap().len() {
            if *grid.get(y).unwrap().get(x).unwrap() == chr {
                result.push(Coord {
                    x: x as i32,
                    y: y as i32,
                })
            }
        }
    }

    result
}

fn calculate(input: &str, part_2: bool) -> usize {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let grid_width = grid.get(0).unwrap().iter().len();
    let grid_height = grid.len();

    let mut combinations: Vec<(Coord, Coord)> = Vec::new();

    for y in 0..grid_height {
        for x in 0..grid_width {
            let chr = *grid.get(y).unwrap().get(x).unwrap();

            if chr != '.' {
                let coords = find_all_char_positions(chr, &grid);

                for coord in coords.clone() {
                    for other_coord in &coords {
                        if coord != *other_coord {
                            combinations.push((coord, other_coord.clone()));
                        }
                    }
                }
            }
        }
    }

    let mut coords: HashSet<Coord> = HashSet::new();

    combinations.into_iter().for_each(|(a, b)| {
        let points = calculate_overshoot_points(&a, &b, &grid, part_2);

        points.into_iter().for_each(|coord| {
            coords.insert(coord);
        });
    });

    coords = coords
        .into_iter()
        .filter(|coord| {
            coord.x >= 0
                && coord.y >= 0
                && coord.x < grid_width as i32
                && coord.y < grid_height as i32
        })
        .collect();

    println!("{:?}", coords);

    coords.len()
}

#[cfg(test)]
mod day8_tests {
    use crate::questions_2024::day8::{calculate, Coord};

    #[test]
    fn test_input_challenge_1() {
        let a = Coord { x: 4, y: 3 };
        let b = Coord { x: 5, y: 5 };

        let input = include_str!("test_input.txt");

        assert_eq!(calculate(input, false), 14);
    }

    #[test]
    fn test_input_challenge_2() {
        let a = Coord { x: 4, y: 3 };
        let b = Coord { x: 5, y: 5 };

        let input = include_str!("test_input.txt");

        assert_eq!(calculate(input, true), 34);
    }

    #[test]
    fn input_challenge_1() {
        let input = include_str!("input.txt");

        // 483 te hoog
        // 450 ook zelfs
        println!("{}", calculate(input, false));
    }

    #[test]
    fn input_challenge_2() {
        let input = include_str!("input.txt");

        // 483 te hoog
        // 450 ook zelfs
        println!("{}", calculate(input, true));
    }
}
