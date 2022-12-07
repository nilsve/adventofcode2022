use std::cmp::Ordering;

struct ElfLists(Vec<ElfList>);
struct SortedElfLists(Vec<ElfList>);

#[derive(Clone, Eq)]
struct ElfList {
    calories: Vec<usize>,
}

impl ElfList {
    fn new() -> ElfList {
        ElfList {
            calories: Vec::new(),
        }
    }

    fn get_total_calories(&self) -> usize {
        self.calories.iter().sum()
    }
}

impl Ord for ElfList {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_total_calories().cmp(&other.get_total_calories())
    }
}

impl PartialOrd<Self> for ElfList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for ElfList {
    fn eq(&self, other: &Self) -> bool {
        self.get_total_calories() == other.get_total_calories()
    }
}

fn sort_elf_lists(mut elf_lists: ElfLists) -> SortedElfLists {
    elf_lists.0.sort();
    elf_lists.0.reverse();

    SortedElfLists(elf_lists.0)
}

fn get_day_calories_for_elf(calorie_lists: &SortedElfLists, rank: usize) -> usize {
    calorie_lists
        .0
        .get(rank)
        .expect("Elf list not found")
        .get_total_calories()
}

fn get_top_3(calorie_lists: &SortedElfLists) -> usize {
    get_day_calories_for_elf(calorie_lists, 0)
        + get_day_calories_for_elf(calorie_lists, 1)
        + get_day_calories_for_elf(calorie_lists, 2)
}

fn parse_input(input: &str) -> ElfLists {
    let lines: Vec<&str> = input.split('\n').collect();

    let mut result = Vec::new();
    let mut current_list = ElfList::new();

    for line in lines {
        if line.is_empty() {
            result.push(current_list.clone());
            current_list = ElfList::new();
            continue;
        }

        current_list
            .calories
            .push(line.parse::<usize>().expect("Incorrect data"));
    }

    ElfLists(result)
}

#[cfg(test)]
mod day1_test {
    use crate::day1::{get_day_calories_for_elf, get_top_3, parse_input, sort_elf_lists};

    #[test]
    fn challenge_1() {
        let input = include_str!("input.txt");
        let elf_lists = parse_input(input);

        let sorted_elf_lists = sort_elf_lists(elf_lists);
        let sum_calories = get_day_calories_for_elf(&sorted_elf_lists, 0);

        assert_eq!(sum_calories, 70720);
    }

    #[test]
    fn challenge_2() {
        let input = include_str!("input.txt");
        let elf_lists = parse_input(input);

        let sorted_elf_lists = sort_elf_lists(elf_lists);
        let sum_calories = get_top_3(&sorted_elf_lists);

        println!("{}", sum_calories);
    }
}
