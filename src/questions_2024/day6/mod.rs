use crate::questions_2024::day6::Direction::{Down, Left, Right, Up};
use crate::questions_2024::day6::Tile::{Empty, Obstacle};
use rayon::prelude::IntoParallelIterator;
use std::collections::{HashMap, HashSet};

#[derive(Default, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn apply_direction(mut self, direction: Direction) -> Self {
        match direction {
            Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };

        self
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn tilt_90(mut self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Obstacle,
    Empty,
}

#[derive(Clone)]
struct Map {
    recursion_detected: bool,
    visited_positions: HashMap<Pos, usize>,
    guard_direction: Direction,
    guard_pos: Pos,
    tiles: Vec<Vec<Tile>>,
}

const RECURSION_TRESHOLD: usize = 5;

impl Map {
    fn new(input: &str) -> Self {
        let mut guard_pos = Pos::default();
        let tiles = input
            .split("\n")
            .filter(|line| !line.is_empty())
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .into_iter()
                    .enumerate()
                    .map(|(x, char)| match char {
                        '^' => {
                            guard_pos = Pos {
                                x: x as isize,
                                y: y as isize,
                            };
                            Empty
                        }
                        '#' => Obstacle,
                        _ => Empty,
                    })
                    .collect()
            })
            .collect();

        Self {
            recursion_detected: false,
            visited_positions: HashMap::new(),
            guard_direction: Up,
            guard_pos,
            tiles,
        }
    }

    fn get_tile(&self, pos: &Pos) -> Option<&Tile> {
        Some(self.tiles.get(pos.y as usize)?.get(pos.x as usize)?)
    }

    fn guard_step(&mut self) -> Result<(), ()> {
        if let None = self.get_tile(&self.guard_pos) {
            return Err(());
        }

        loop {
            if let Some(Obstacle) =
                self.get_tile(&self.guard_pos.apply_direction(self.guard_direction))
            {
                self.guard_direction = self.guard_direction.tilt_90();
            } else {
                if *self.visited_positions.entry(self.guard_pos).or_default() > RECURSION_TRESHOLD {
                    self.recursion_detected = true;
                }

                break;
            }
        }

        *self.visited_positions.entry(self.guard_pos).or_default() += 1;

        self.guard_pos = self.guard_pos.apply_direction(self.guard_direction);

        Ok(())
    }
}

fn calculate(input: &str) -> usize {
    let mut map = Map::new(input);

    solve_map(&mut map, false);

    map.visited_positions.len()
}

fn solve_map(map: &mut Map, stop_on_recursion: bool) {
    loop {
        match map.guard_step() {
            Ok(_) => {
                if map.recursion_detected && stop_on_recursion {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}

fn calculate_obstacles(input: &str) -> usize {
    let map = Map::new(input);

    let mut solved_map = map.clone();

    solve_map(&mut solved_map, false);

    solved_map
        .visited_positions
        .iter()
        .map(|(pos, _)| pos)
        .filter(|pos| **pos != solved_map.guard_pos)
        .map(|pos| {
            let mut test_map = map.clone();

            *test_map
                .tiles
                .get_mut(pos.y as usize)
                .unwrap()
                .get_mut(pos.x as usize)
                .unwrap() = Obstacle;

            solve_map(&mut test_map, true);

            if test_map.recursion_detected {
                return 1;
            }

            return 0;
        })
        .sum()
}

#[cfg(test)]
mod day5_tests {
    use crate::questions_2024::day6::{calculate, calculate_obstacles};

    #[test]
    fn test_input_challenge_1() {
        let input = include_str!("test_input.txt");
        let result = calculate(input);

        assert_eq!(result, 41);
    }

    #[test]
    fn input_challenge_1() {
        let input = include_str!("input.txt");
        let result = calculate(input);

        println!("{result}")
    }

    #[test]
    fn test_input_challenge_2() {
        let input = include_str!("test_input.txt");
        let result = calculate_obstacles(input);

        assert_eq!(result, 6);
    }

    #[test]
    fn input_challenge_2() {
        let input = include_str!("input.txt");
        let result = calculate_obstacles(input);

        println!("{result}")
    }
}
