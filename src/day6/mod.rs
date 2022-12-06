fn parse_input(input: &str, subsection_size: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    for (index, slice) in chars.windows(subsection_size).enumerate() {
        let mut found = false;
        for chr in 0..slice.len() {
            let subslice = &slice[chr + 1..slice.len()];
            if subslice.contains(slice.get(chr).unwrap()) {
                found = true;
                break;
            }
        }

        if !found {
            return index + subsection_size;
        }
    }

    panic!("Not found");
}

#[cfg(test)]
mod day_6_test {
    use crate::day6::parse_input;

    #[test]
    fn test_challenge1_inputs() {
        assert_eq!(parse_input("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(parse_input("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(parse_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(parse_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_challenge2_inputs() {
        assert_eq!(parse_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(parse_input("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(parse_input("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(parse_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(parse_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }

    #[test]
    fn get_challenge1_output() {
        let input = include_str!("input.txt");
        println!("{}", parse_input(input, 4));
    }

    #[test]
    fn get_challenge2_output() {
        let input = include_str!("input.txt");
        println!("{}", parse_input(input, 14));
    }
}
