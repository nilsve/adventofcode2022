use std::collections::HashMap;

struct File {
    name: String,
    size: usize,
}

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
}

struct StatefulFileSystem {}

fn parse_input(input: &str) -> usize {
    let fs = Directory::new("".to_string());

    0
}

#[cfg(test)]
mod day_7_tests {
    use crate::day7::parse_input;

    #[test]
    fn test_input_challenge_1() {
        let input = include_str!("test_input.txt");
        let result = parse_input(input);

        assert_eq!(result, 95437);
    }
}
