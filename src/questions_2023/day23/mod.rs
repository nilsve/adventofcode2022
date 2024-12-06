use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn get_relative_direction(&self, other: &Coord) -> Direction {
        if other.y > self.y {
            return Direction::Down;
        }
        if other.y < self.y {
            return Direction::Up;
        }
        if other.x > self.x {
            return Direction::Right;
        }
        if other.x < self.x {
            return Direction::Left;
        }

        panic!("No direction found");
    }
}

enum PathFindResult {
    StillSearching,
    FoundExit(usize),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TileType {
    Path,
    Forest,
    Slope(Direction),
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    tile_type: TileType,
    position: Coord,
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let tiles = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let tile_type = match c {
                            '.' => TileType::Path,
                            '#' => TileType::Forest,
                            '>' => TileType::Slope(Direction::Right),
                            '<' => TileType::Slope(Direction::Left),
                            '^' => TileType::Slope(Direction::Up),
                            'v' => TileType::Slope(Direction::Down),
                            _ => panic!("Unknown tile"),
                        };

                        Tile {
                            tile_type,
                            position: Coord {
                                x: x as isize,
                                y: y as isize,
                            },
                        }
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();

        Self { tiles }
    }

    fn get_width(&self) -> usize {
        self.tiles.get(0).unwrap().len()
    }

    fn get_height(&self) -> usize {
        self.tiles.len()
    }

    fn find_exits(&self) -> Vec<Coord> {
        // Parses everything instead of just the edges but whatever
        self.tiles
            .iter()
            .flatten()
            .enumerate()
            .filter(|(index, tile)| {
                if (tile.position.y == 0
                    || tile.position.y == self.get_height() as isize - 1
                    || tile.position.x == 0
                    || tile.position.x == self.get_width() as isize - 1)
                    && tile.tile_type == TileType::Path
                {
                    true
                } else {
                    false
                }
            })
            .map(|(_, tile)| tile.position)
            .collect()
    }

    fn get_tile_at_coord(&self, coord: &Coord) -> &Tile {
        &self
            .tiles
            .get(coord.y as usize)
            .expect("Tile not found at Y coord")
            .get(coord.x as usize)
            .expect("Tile not found at X coord")
    }

    fn get_walkable_adjacent_coords(
        &self,
        coord: &Coord,
        exclude: &HashSet<Coord>,
        can_climb_slopes: bool,
    ) -> Vec<Coord> {
        let coords = [
            Coord {
                x: coord.x - 1,
                y: coord.y,
            },
            Coord {
                x: coord.x + 1,
                y: coord.y,
            },
            Coord {
                x: coord.x,
                y: coord.y - 1,
            },
            Coord {
                x: coord.x,
                y: coord.y + 1,
            },
        ];

        let coords = coords
            .into_iter()
            .filter(|coord| {
                coord.x >= 0
                    && coord.y >= 0
                    && coord.x < self.get_width() as isize
                    && coord.y < self.get_height() as isize
            })
            .filter(|possible_coord| !exclude.contains(possible_coord))
            .filter(|possible_coord| {
                let tile = self.get_tile_at_coord(&possible_coord);

                match tile.tile_type {
                    TileType::Path => true,
                    TileType::Forest => false,
                    TileType::Slope(direction) => {
                        if can_climb_slopes {
                            return direction == coord.get_relative_direction(&possible_coord)
                                || direction == possible_coord.get_relative_direction(&coord);
                        }

                        direction == coord.get_relative_direction(&possible_coord)
                    }
                }
            })
            .collect();

        coords
    }

    fn find_longest_path(
        &self,
        mut start: Coord,
        end: &Coord,
        mut traversed: HashSet<Coord>,
        mut start_length: usize,
        can_climb_slopes: bool,
    ) -> PathFindResult {
        loop {
            traversed.insert(start.clone());

            if start == *end {
                return PathFindResult::FoundExit(start_length);
            }

            let possible_coords =
                self.get_walkable_adjacent_coords(&start, &traversed, can_climb_slopes);

            if possible_coords.len() == 1 {
                start = *possible_coords.get(0).unwrap();
                start_length += 1;
            } else {
                return if let Some(longest_path) = possible_coords
                    .iter()
                    .map(|coord| {
                        self.find_longest_path(
                            coord.clone(),
                            &end,
                            traversed.clone(),
                            start_length + 1,
                            can_climb_slopes,
                        )
                    })
                    .filter(|result| match result {
                        PathFindResult::StillSearching => false,
                        PathFindResult::FoundExit(_) => true,
                    })
                    .map(|result| match result {
                        PathFindResult::StillSearching => panic!("Shouldn't happen"),
                        PathFindResult::FoundExit(length) => length,
                    })
                    .max()
                {
                    println!("Found exit with {} steps", longest_path);
                    PathFindResult::FoundExit(longest_path)
                } else {
                    PathFindResult::StillSearching
                };
            }
        }
    }
}

fn calculate(input: &str) -> (usize, usize) {
    let map = Map::new(input);

    let exits = map.find_exits();

    let longest_path_size = match map.find_longest_path(
        exits.get(0).unwrap().clone(),
        exits.get(1).unwrap(),
        HashSet::new(),
        0,
        false,
    ) {
        PathFindResult::StillSearching => panic!("Exit not found!"),
        PathFindResult::FoundExit(length) => length,
    };

    let longest_path_size_without_slopes = match map.find_longest_path(
        exits.get(0).unwrap().clone(),
        exits.get(1).unwrap(),
        HashSet::new(),
        0,
        true,
    ) {
        PathFindResult::StillSearching => panic!("Exit not found"),
        PathFindResult::FoundExit(length) => length,
    };

    (longest_path_size, longest_path_size_without_slopes)
}

#[cfg(test)]
mod day1_tests {
    use crate::questions_2023::day23::{calculate, Coord, Map};
    use std::collections::HashSet;

    #[test]
    fn test_input_challenge() {
        let input = include_str!("test_input.txt");
        let result = calculate(input);

        assert_eq!(result, (94, 154));
    }

    #[test]
    fn input_challenge() {
        let input = include_str!("input.txt");
        let result = calculate(input);

        println!("Challenge 0: {}, challenge 1: {}", result.0, result.1)
    }

    #[test]
    fn find_exits() {
        let input = include_str!("test_input.txt");
        let map = Map::new(input);

        let exits = map.find_exits();

        assert_eq!(exits.len(), 2);
    }

    #[test]
    fn get_adjacent() {
        let input = include_str!("test_input.txt");
        let map = Map::new(input);
        let mut traversed = HashSet::new();

        assert_eq!(
            map.get_walkable_adjacent_coords(&Coord { x: 1, y: 0 }, &traversed, false)
                .len(),
            1
        );
        assert_eq!(
            map.get_walkable_adjacent_coords(&Coord { x: 2, y: 1 }, &traversed, false)
                .len(),
            2
        );

        assert_eq!(
            map.get_walkable_adjacent_coords(&Coord { x: 2, y: 1 }, &traversed, false)
                .len(),
            2
        );

        assert_eq!(
            map.get_walkable_adjacent_coords(&Coord { x: 11, y: 3 }, &traversed, false)
                .len(),
            2
        );
        assert_eq!(
            map.get_walkable_adjacent_coords(&Coord { x: 13, y: 3 }, &traversed, false)
                .len(),
            1
        );

        let vec = map.get_walkable_adjacent_coords(&Coord { x: 11, y: 3 }, &traversed, false);
        traversed.insert(vec.get(0).unwrap().clone());
        let vec2 = map.get_walkable_adjacent_coords(&Coord { x: 11, y: 3 }, &traversed, false);

        assert_eq!(vec.len() - 1, vec2.len());
    }
}
