use std::fmt::{Debug, Formatter};
use std::ops::Index;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Type {
    File(usize),
    Empty,
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let chr = match self {
            Type::File(id) => &id.to_string(),
            Type::Empty => ".",
        };

        f.write_str(&format!("{chr}"))
    }
}

struct Disk {
    data: Vec<Type>,
}

impl Debug for Disk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.data
            .iter()
            .for_each(|t| f.write_str(&format!("{:?}", t)).unwrap());

        Ok(())
    }
}

impl Disk {
    fn new(input: &str) -> Self {
        let data: Vec<Type> = input
            .split("\n")
            .collect::<Vec<&str>>()
            .get(0)
            .map(|line| {
                let mut is_odd = true;
                let mut file_id = 0;
                line.chars()
                    .map(|chr| {
                        is_odd = !is_odd;

                        let parsed: usize = chr.to_string().parse().unwrap();

                        if !is_odd {
                            let result = Type::File(file_id);
                            file_id += 1;
                            return vec![result; parsed];
                        } else {
                            return vec![Type::Empty; parsed];
                        }
                    })
                    .flatten()
                    .collect()
            })
            .unwrap();

        Self { data }
    }

    fn get_first_free_space(&self, ending_position: usize, required_size: usize) -> Option<usize> {
        self.data[0..ending_position]
            .windows(required_size)
            .position(|ts| {
                if ts.len() != required_size {
                    return false;
                }

                !ts.iter().any(|t| match t {
                    Type::File(_) => true,
                    Type::Empty => false,
                })
            })
    }

    fn compact_fragmented(&mut self) {
        for i in (0..self.data.len()).rev() {
            if let Type::File(id) = self.data.get(i).unwrap() {
                let first_free_space = match self.get_first_free_space(i, 1) {
                    None => return,
                    Some(space) => space,
                };
                *self.data.get_mut(first_free_space).unwrap() = Type::File(*id);
                *self.data.get_mut(i).unwrap() = Type::Empty;
            }
        }
    }

    // This is garbage lol
    fn get_file_size(&self, ending_position: usize) -> Option<usize> {
        let file = match self.data.get(ending_position).unwrap() {
            Type::File(index) => index,
            Type::Empty => panic!(),
        };

        self.data[..ending_position]
            .iter()
            .rev()
            .enumerate()
            .find(|(index, t)| match t {
                Type::File(other_file) => file != other_file,
                Type::Empty => true,
            })
            .map(|(index, _)| index + 1)
    }

    fn compact_defragmented(&mut self) {
        let mut skip = 0; // The horrors....
        for mut i in (0..self.data.len()).rev() {
            i -= skip; // close your eyes

            if let Type::File(id) = self.data.get(i).unwrap() {
                let id = *id;
                let file_size = match self.get_file_size(i) {
                    None => return,
                    Some(val) => val,
                };

                match self.get_first_free_space(i, file_size) {
                    None => skip += file_size - 1,
                    Some(space) => {
                        for x in 0..file_size {
                            *self.data.get_mut(space + x).unwrap() = Type::File(id);
                            *self.data.get_mut(i - x).unwrap() = Type::Empty;
                        }
                    }
                }
            }
        }
    }

    fn calculate_checksum(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(index, t)| match t {
                Type::File(file_index) => Some(file_index * index),
                Type::Empty => None,
            })
            .sum()
    }
}

fn calculate(input: &str, defragmented: bool) -> usize {
    let mut disk = Disk::new(input);

    println!("{:?}", disk);

    if defragmented {
        disk.compact_defragmented();
    } else {
        disk.compact_fragmented();
    }

    println!("{:?}", disk);

    disk.calculate_checksum()
}

#[cfg(test)]
mod day9_tests {
    use crate::questions_2024::day9::calculate;

    #[test]
    fn test_input_challenge_1() {
        let input = include_str!("test_input.txt");

        assert_eq!(calculate(input, false), 1928);
    }
    #[test]
    fn test_input_challenge_2() {
        let input = include_str!("test_input.txt");

        assert_eq!(calculate(input, true), 2858);
    }

    #[test]
    fn input_challenge_1() {
        let input = include_str!("input.txt");

        //6362722604045
        //6337921897505
        println!("{}", calculate(input, false));
    }

    #[test]
    fn input_challenge_2() {
        let input = include_str!("input.txt");

        //6337921897505
        println!("{}", calculate(input, true));
    }
}
