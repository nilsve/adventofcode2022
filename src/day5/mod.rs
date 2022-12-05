struct Dock {
    stack_count: usize,
    stacks: Vec<CrateStack>,
}

impl Dock {
    fn new(stack_count: usize) -> Self {
        Self {
            stack_count,
            stacks: vec![CrateStack::new(); stack_count],
        }
    }

    fn pop(&mut self, stack: usize) -> char {
        self.stacks
            .get_mut(stack)
            .expect("Stack not found")
            .crates
            .pop()
            .expect("Empty stack")
    }

    fn push(&mut self, stack: usize, chr: char) {
        self.stacks
            .get_mut(stack)
            .expect("Stack not found")
            .crates
            .push(chr)
    }
}

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let splitted: Vec<&str> = input.split(' ').collect();

        Self {
            count: splitted
                .get(1)
                .expect("count not found")
                .parse::<usize>()
                .expect("Couldnt parse count"),
            from: splitted
                .get(3)
                .expect("count not found")
                .parse::<usize>()
                .expect("Couldnt parse count")
                - 1,
            to: splitted
                .get(5)
                .expect("count not found")
                .parse::<usize>()
                .expect("Couldnt parse count")
                - 1,
        }
    }
}

struct DockBuilder;

impl DockBuilder {
    fn build(arrangement: &str) -> Dock {
        let mut dock = Dock::new(DockBuilder::calculate_max_stacks(arrangement));

        let mut lines: Vec<&str> = arrangement.split('\n').collect();
        lines.reverse();

        lines.iter().for_each(|line| {
            if !line.contains("[") {
                return;
            }

            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .enumerate()
                .for_each(|(index, chunk)| {
                    let item = &chunk[1];

                    if *item != ' ' {
                        dock.push(index, *item);
                    }
                });
        });

        dock
    }

    fn calculate_max_stacks(arrangement: &str) -> usize {
        arrangement
            .split('\n')
            .map(|line| (line.len() + 1) / 4)
            .max()
            .expect("Empty line found")
    }
}

#[derive(Clone)]
struct CrateStack {
    crates: Vec<char>,
}

impl CrateStack {
    fn new() -> Self {
        Self { crates: Vec::new() }
    }
}

fn calculate_challenge(input: &str, is_cranelift_9001: bool) -> String {
    let mut splitted = input.split("\n\n");
    let arrangement = splitted.next().unwrap();
    let procedure = splitted.next().unwrap();

    let mut dock = DockBuilder::build(arrangement);

    let instructions: Vec<Instruction> = procedure
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            Some(Instruction::parse(line))
        })
        .collect();

    instructions.iter().for_each(|instruction| {
        if !is_cranelift_9001 {
            for _ in 0..instruction.count {
                let item = dock.pop(instruction.from);
                dock.push(instruction.to, item);
            }
        } else {
            let mut popped: Vec<char> = (0..instruction.count)
                .map(|_| dock.pop(instruction.from))
                .collect();

            popped.reverse();

            popped
                .iter()
                .for_each(|item| dock.push(instruction.to, *item));
        }
    });

    let mut result = String::new();
    for stack in 0..dock.stack_count {
        result.push(dock.pop(stack));
    }

    result
}

#[cfg(test)]
mod day5_tests {
    use crate::day5::calculate_challenge;

    #[test]
    fn test_input_challenge_1() {
        let test_input = include_str!("test_input.txt");

        let result = calculate_challenge(test_input, false);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_input_challenge_2() {
        let test_input = include_str!("test_input.txt");

        let result = calculate_challenge(test_input, true);
        assert_eq!(result, "MCD");
    }

    #[test]
    fn get_challenge_1_output() {
        let test_input = include_str!("input.txt");

        let result = calculate_challenge(test_input, false);
        println!("{}", result);
    }

    #[test]
    fn get_challenge_2_output() {
        let test_input = include_str!("input.txt");

        let result = calculate_challenge(test_input, true);
        println!("{}", result);
    }
}
