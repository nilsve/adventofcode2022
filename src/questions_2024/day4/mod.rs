use std::fmt::{Debug, Formatter, Pointer};

pub trait GridIteratorType: Sized {}

#[derive(Default)]
pub struct HorizontalIterator {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct VerticalIterator {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct DiagonalIterator {
    x: usize,
    y: usize,
    diagonal: usize,
    insert_space: bool,
}

impl GridIteratorType for HorizontalIterator {}
impl GridIteratorType for VerticalIterator {}
impl GridIteratorType for DiagonalIterator {}

pub struct GridIterator<I: GridIteratorType> {
    grid: Grid,
    data: I,
}

#[derive(Clone)]
pub struct Grid {
    data: Vec<Vec<char>>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = self.data.iter().map(|line| line.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join("\n");
        f.write_str(&text)
    }
}

impl Grid {
    fn new_from_grid_coord(grid: &Grid, x: usize, y: usize) -> Self {
        Grid {
            data: vec![
                vec![grid.data[y - 1][x - 1], grid.data[y - 1][x], grid.data[y - 1][x + 1]],
                vec![grid.data[y][x - 1], grid.data[y][x], grid.data[y][x + 1]],
                vec![grid.data[y + 1][x - 1], grid.data[y + 1][x], grid.data[y + 1][x + 1]],
            ]
        }
    }

    fn horizontal_iterator(self) -> GridIterator<HorizontalIterator> {
        GridIterator {
            grid: self,
            data: Default::default(),
        }
    }

    fn vertical_iterator(self) -> GridIterator<VerticalIterator> {
        GridIterator {
            grid: self,
            data: Default::default(),
        }
    }

    fn diagonal_left_iterator(self) -> GridIterator<DiagonalIterator> {
        GridIterator {
            data: Default::default(),
            grid: Grid {
                data: self
                    .data
                    .into_iter()
                    .map(|line| line.iter().rev().copied().collect())
                    .collect(),
            },
        }
    }
    fn diagonal_right_iterator(self) -> GridIterator<DiagonalIterator> {
        GridIterator {
            grid: self,
            data: Default::default(),
        }
    }
}

impl Iterator for GridIterator<HorizontalIterator> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let horizontal_line = match self.grid.data.get(self.data.y) {
            None => return None,
            Some(line) => line,
        };

        match horizontal_line.get(self.data.x) {
            None => {
                self.data.y += 1;
                self.data.x = 0;
                Some('\n')
            }
            Some(chr) => {
                self.data.x += 1;
                Some(*chr)
            }
        }
    }
}

impl Iterator for GridIterator<VerticalIterator> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let horizontal_line = match self.grid.data.get(self.data.y) {
            None => {
                self.data.x += 1;
                self.data.y = 0;

                return Some('\n');
            }
            Some(line) => line,
        };

        match horizontal_line.get(self.data.x) {
            None => {
                None
            }
            Some(chr) => {
                self.data.y += 1;
                Some(*chr)
            }
        }
    }
}

impl Iterator for GridIterator<DiagonalIterator> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.insert_space {
            self.data.insert_space = false;
            return Some('\n');
        }

        let len = self.grid.data.len() as isize;

        if self.data.x >= len as usize || self.data.y >= len as usize {
            return None;
        }

        let chr = self
            .grid
            .data
            .get(self.data.y)
            .unwrap()
            .get(self.data.x)
            .unwrap();

        let mut x = self.data.x as isize;
        let mut y = self.data.y as isize;

        x -= 1;
        y += 1;

        if y == len {
            self.data.diagonal += 1;
            self.data.insert_space = true;
            y = self.data.diagonal as isize;
            x = len - 1;
        } else if x < 0 {
            self.data.insert_space = true;
            x = y.clamp(0, len - 1);
            y = 0;
        }

        if y > len {
            y = x;
        }

        self.data.x = x as usize;
        self.data.y = y as usize;

        Some(*chr)
    }
}

const XMAS_PATTERN: &'static str = "XMAS";
const MAS_PATTERN: &'static str = "MAS";

fn calculate(input: &str, calc_mas: bool) -> usize {
    let data: Vec<Vec<char>> = input
        .split("\n")
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|str| str.chars().collect())
        .collect();

    let grid = Grid { data };

    if !calc_mas {
        let horizontal_count = count_occurrences(grid.clone().horizontal_iterator(), XMAS_PATTERN);
        let vertical_count = count_occurrences(grid.clone().vertical_iterator(), XMAS_PATTERN);
        let diagonal_right_count = count_occurrences(grid.clone().diagonal_right_iterator(), XMAS_PATTERN);
        let diagonal_left_count = count_occurrences(grid.diagonal_left_iterator(), XMAS_PATTERN);

        return horizontal_count + vertical_count + diagonal_left_count + diagonal_right_count;
    }

    let mut total = 0;

    for y in 1..grid.data.len() - 1 {
        for x in 1..grid.data.len() - 1 {
            let sub_grid = Grid::new_from_grid_coord(&grid, x, y);


            let diagonal_right_count = count_occurrences(sub_grid.clone().diagonal_right_iterator(), MAS_PATTERN);
            let diagonal_left_count = count_occurrences(sub_grid.clone().diagonal_left_iterator(), MAS_PATTERN);

            if diagonal_right_count > 0|| diagonal_left_count > 0 {
                println!("{:?} {diagonal_right_count} {diagonal_left_count}", sub_grid);
                total += 1;
            }
        }
    }

    total
}

fn count_occurrences(iterator: impl Iterator<Item = char>, pattern: &str) -> usize {
    let chars = iterator
        .map(|char| char.to_string())
        .collect::<Vec<String>>()
        .join("");

    let forwards = chars.match_indices(pattern).collect::<Vec<_>>().len();
    let reversed_pattern: String = pattern.chars().rev().collect();
    let backwards = chars.match_indices(&reversed_pattern).collect::<Vec<_>>().len();

    forwards + backwards
}

#[cfg(test)]
mod day4_tests {
    use crate::questions_2024::day4::calculate;

    #[test]
    fn test_input_challenge_1() {
        let input = include_str!("test_input.txt");
        let result = calculate(input, false);

        assert_eq!(result, 18);
    }

    #[test]
    fn test_input_challenge_2() {
        let input = include_str!("test_input.txt");
        let result = calculate(input, true);

        assert_eq!(result, 9);
    }

    #[test]
    fn test_challenge_2() {
        let input = include_str!("input.txt");

        // Calculate the time for the following call
        let current_time = std::time::Instant::now();
        let result = calculate(input, true);

        println!("Time: {:?}", current_time.elapsed());

        println!("{}", result);
    }
}
