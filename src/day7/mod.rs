use std::collections::HashMap;
use std::fs;
use std::ops::Deref;
use std::path::Path;

#[derive(Debug)]
struct File(usize);

#[derive(Debug)]
struct Directory {
    name: String,
    sub_directories: HashMap<String, Directory>,
    files: HashMap<String, File>,
}

impl Directory {
    fn new(name: String) -> Self {
        Self {
            name,
            files: HashMap::new(),
            sub_directories: HashMap::new(),
        }
    }

    fn upsert_directory(&mut self, dir: &str) {
        if !self.sub_directories.contains_key(dir) {
            self.sub_directories
                .insert(dir.to_owned(), Directory::new(dir.to_owned()));
        }
    }

    fn upsert_file(&mut self, file: &str, size: usize) {
        if !self.files.contains_key(file) {
            self.files.insert(file.to_owned(), File(size));
        }
    }

    fn get_size(&self) -> usize {
        let file_sizes: usize = self.files.values().map(|file| file.0).sum();

        let subdirectory_sizes: usize = self
            .sub_directories
            .values()
            .map(|sub_directory| sub_directory.get_size())
            .sum();

        file_sizes + subdirectory_sizes
    }
}

#[derive(Debug)]
struct FileSystem {
    current_path: Vec<String>,
    root: Directory,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            root: Directory::new("/".to_string()),
            current_path: vec![],
        }
    }

    fn navigate(&mut self, dir: &str) {
        match dir {
            "/" => {
                self.current_path = vec![];
            }
            ".." => {
                self.current_path.pop();
            }
            path => self.current_path.push(path.to_owned()),
        };
    }

    fn get_current_dir(&mut self) -> &mut Directory {
        let mut dir = &mut self.root;
        for path in &self.current_path {
            dir = dir
                .sub_directories
                .get_mut(path)
                .expect("Directory not found");
        }

        dir
    }

    fn get_files(&mut self) -> &HashMap<String, File> {
        &self.get_current_dir().files
    }
}

#[derive(Debug)]
struct Command<'a> {
    data: CommandType<'a>,
    result: Vec<&'a str>,
}

impl<'a> Command<'a> {
    fn parse(input: &'a str) -> Self {
        let mut splitted = input.split('\n').into_iter();
        Self {
            data: CommandType::parse(splitted.next().expect("Command wasn't parseable")),
            result: splitted.collect::<Vec<&'a str>>(),
        }
    }
}

#[derive(Debug)]
enum CommandType<'a> {
    Cd(&'a str),
    Ls,
}

impl<'a> CommandType<'a> {
    fn parse(input: &'a str) -> Self {
        let mut splitted = input.split(' ');

        match splitted.next().expect("Command not parseable") {
            "cd" => CommandType::Cd(splitted.next().expect("Couldn't parse cd args")),
            "ls" => CommandType::Ls,
            _ => unimplemented!("Command not implemented"),
        }
    }
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .split("$ ")
        .filter_map(|data| {
            if data.is_empty() {
                return None;
            }

            let command = Command::parse(data);

            Some(command)
        })
        .collect()
}

fn build_fs(input: &str) -> FileSystem {
    const DIR: &'static str = "dir ";

    let mut fs = FileSystem::new();

    let commands: Vec<Command> = parse_commands(input);

    commands.iter().for_each(|command| match command.data {
        CommandType::Cd(dir) => {
            fs.navigate(dir);
        }
        CommandType::Ls => command.result.iter().for_each(|output| {
            let mut current_directory = fs.get_current_dir();

            if output.starts_with(DIR) {
                current_directory.upsert_directory(&output[DIR.len()..output.len()]);
            } else if !output.is_empty() {
                // It's a file
                let mut splitted = output.split(' ');
                let file_size = splitted
                    .next()
                    .expect("No size found")
                    .parse::<usize>()
                    .expect("Not a number");
                let file_name = splitted.next().expect("No name found");

                current_directory.upsert_file(file_name, file_size);
            }
        }),
    });

    fs
}

fn challenge_1(fs: &FileSystem) -> usize {
    challenge_1_calculator(&fs.root)
}

fn challenge_1_calculator(directory: &Directory) -> usize {
    let mut my_size = directory.get_size();

    if my_size > 100000 {
        my_size = 0;
    }

    let other_sizes: usize = directory
        .sub_directories
        .values()
        .map(|sub_directory| challenge_1_calculator(sub_directory))
        .sum();

    my_size + other_sizes
}

fn challenge_2(fs: FileSystem) -> usize {
    let required_disk_space = 30000000;
    let unused_disk_space = TOTAL_DISK_SPACE - fs.root.get_size();
    let should_free = required_disk_space - unused_disk_space;

    challenge_2_calculator(&fs.root, should_free, 1000000000000)
}

fn challenge_2_calculator(
    directory: &Directory,
    required_space: usize,
    mut closest: usize,
) -> usize {
    let size = directory.get_size();

    if size > required_space && size < closest {
        closest = size;
    }

    let min = directory
        .sub_directories
        .values()
        .map(|sub_directory| challenge_2_calculator(sub_directory, required_space, closest))
        .min()
        .unwrap_or(closest);

    min
}

const TOTAL_DISK_SPACE: usize = 70000000;

#[cfg(test)]
mod day_7_tests {
    use crate::day7::{
        build_fs, challenge_1, challenge_2, challenge_2_calculator, TOTAL_DISK_SPACE,
    };

    #[test]
    fn test_input_challenge_1() {
        let input = include_str!("test_input.txt");
        let file_system = build_fs(input);
        let result = challenge_1(&file_system);
        assert_eq!(result, 95437);
    }

    #[test]
    fn calculate_challenge_1() {
        let input = include_str!("input.txt");
        let file_system = build_fs(input);
        let result = challenge_1(&file_system);
        println!("{}", result);
    }

    #[test]
    fn test_input_challenge_2() {
        let input = include_str!("test_input.txt");
        let file_system = build_fs(input);

        assert_eq!(challenge_2(file_system), 24933642);
    }

    #[test]
    fn calculate_challenge_2() {
        let input = include_str!("input.txt");
        let file_system = build_fs(input);

        println!("{}", challenge_2(file_system));
    }
}
