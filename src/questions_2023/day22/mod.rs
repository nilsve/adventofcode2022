#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord {
    fn new(input: &str) -> Self {
        let mut splitted = input.split(",");

        Coord {
            x: splitted.next().unwrap().parse().unwrap(),
            y: splitted.next().unwrap().parse().unwrap(),
            z: splitted.next().unwrap().parse().unwrap(),
        }
    }

    fn filter_vertical_coords_for_falling_block(coords: Vec<Coord>) -> Vec<Coord> {
        // Do all coords have the same x and y?
        let same_x_and_y = coords
            .iter()
            .all(|coord| coord.x == coords[0].x && coord.y == coords[0].y);

        if same_x_and_y {
            let min_z = coords.iter().map(|coord| coord.z).min().unwrap();

            vec![Coord {
                x: coords[0].x,
                y: coords[0].y,
                z: min_z,
            }]
        } else {
            coords
        }
    }

    fn get_coords_between_positions(coord1: &Coord, coord2: &Coord) -> Vec<Coord> {
        let mut coords: Vec<Coord> = Vec::new();

        for x in coord1.x..=coord2.x {
            for y in coord1.y..=coord2.y {
                for z in coord1.z..=coord2.z {
                    coords.push(Coord { x, y, z });
                }
            }
        }

        coords
    }
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    from: Coord,
    to: Coord,
    // brick_number: char,
}

impl Brick {
    fn new(line: &str, line_number: usize) -> Self {
        let mut splitted = line.split("~");

        let from = splitted.next().unwrap();
        let to = splitted.next().unwrap();

        Brick {
            from: Coord::new(from),
            to: Coord::new(to),
            // brick_number: ('A' as u8 + line_number as u8) as char,
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Vec<bool>>>,
}

impl Map {
    fn get_bricks_for_input(input: &str) -> Vec<Brick> {
        input
            .split("\n")
            .enumerate()
            .filter(|(_, line)| !line.is_empty())
            .map(|(index, line)| Brick::new(line, index))
            .collect()
    }

    fn create_map_from_bricks(bricks: &Vec<Brick>) -> Self {
        let mut map: Vec<Vec<Vec<bool>>> = Vec::new();

        bricks.iter().for_each(|brick| {
            let coords = Coord::get_coords_between_positions(&brick.from, &brick.to);

            coords.iter().for_each(|coord| {
                let x = coord.x;
                let y = coord.y;
                let z = coord.z;

                if map.len() <= x {
                    map.resize(x + 1, Vec::new());
                }

                if map[x].len() <= y {
                    map[x].resize(y + 1, Vec::new());
                }

                if map[x][y].len() <= z {
                    map[x][y].resize(z + 1, false);
                }

                map[x][y][z] = true;
            });
        });

        Self { map }
    }

    fn exists_brick_at_coord(&self, coord: &Coord) -> bool {
        self.map[coord.x][coord.y][coord.z]
    }
}

#[derive(Debug)]
struct Simulator {
    bricks: Vec<Brick>,
}

impl Simulator {
    fn new(bricks: Vec<Brick>) -> Self {
        Self { bricks }
    }

    // Returns true if changes were made
    fn simulate_falling_step(bricks: Vec<Brick>) -> (Vec<Brick>, bool) {
        let map = Map::create_map_from_bricks(&bricks);
        let mut changes_made = false;

        let updated_bricks = bricks
            .clone()
            .into_iter()
            .map(|brick| {
                if brick.from.z == 1 || brick.to.z == 1 {
                    return brick;
                }

                let mut fallen_brick = brick.clone();

                fallen_brick.from.z -= 1;
                fallen_brick.to.z -= 1;

                let coords = Coord::filter_vertical_coords_for_falling_block(
                    Coord::get_coords_between_positions(&fallen_brick.from, &fallen_brick.to),
                );

                let can_fall = coords.iter().all(|coord| !map.exists_brick_at_coord(coord));

                if can_fall {
                    // println!("Can fall: {:?}", brick);
                    changes_made = true;
                    fallen_brick
                } else {
                    // println!("Cannot fall: {:?}", brick);
                    brick
                }
            })
            .collect();

        (updated_bricks, changes_made)
    }

    fn simulate_falling(&mut self) {
        loop {
            let (updated_bricks, changes_made) = Self::simulate_falling_step(self.bricks.clone());

            if !changes_made {
                break;
            }

            self.bricks = updated_bricks;
        }
    }

    fn get_removable_brick_count(&self) -> usize {
        self.bricks
            .iter()
            .enumerate()
            .filter(|(index, brick)| {
                let bricks_without_this_brick: Vec<Brick> = self
                    .bricks
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter(|(other_index, _)| *index != *other_index)
                    .map(|(_, brick)| brick)
                    .collect();

                let is_removable = !Self::simulate_falling_step(bricks_without_this_brick).1;

                is_removable
            })
            .count()
    }
}

fn calculate(input: &str) -> (usize, usize) {
    let bricks = Map::get_bricks_for_input(input);

    let mut simulator = Simulator::new(bricks);

    simulator.simulate_falling();

    (simulator.get_removable_brick_count(), 0)
}

#[cfg(test)]
mod day1_tests {
    use crate::questions_2023::day22::{calculate, Coord, Map};

    #[test]
    fn test_input_challenge() {
        let input = include_str!("test_input.txt");
        let result = calculate(input);

        assert_eq!(result, (5, 0));
    }

    #[test]
    fn input_challenge() {
        let input = include_str!("input.txt");
        let result = calculate(input);

        println!("Challenge 0: {}, challenge 1: {}", result.0, result.1)
    }

    #[test]
    fn get_coords() {
        let input = include_str!("test_input.txt");
        let bricks = Map::get_bricks_for_input(input);

        let brick = &bricks[0];

        let coords = Coord::get_coords_between_positions(&brick.from, &brick.to);

        assert_eq!(coords.len(), 3);
    }
}
