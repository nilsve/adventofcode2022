fn calculate(input: &str, correct: bool) -> usize {
    let sections: Vec<_> = input.split("\n\n").collect();

    let ordering: Vec<(u32, u32)> = sections
        .get(0)
        .unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<u32> = line.split("|").map(|part| part.parse::<u32>().unwrap()).collect();

            (*nums.get(0).unwrap(), *nums.get(1).unwrap())
        })
        .collect();

    let docs: Vec<Vec<u32>> = sections
        .get(1)
        .unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.split(",").map(|part| part.parse::<u32>().unwrap()).collect())
        .collect();

    let final_docs: Vec<Vec<u32>> = if correct {
        docs.into_iter().filter(|doc| ordering_is_correct(&doc, &ordering)).collect()
    } else {
        docs
            .into_iter()
            .filter(|doc| !ordering_is_correct(&doc, &ordering))
            .map(|doc| fix_doc(&doc, &ordering))
            .collect()
    };

    final_docs.iter().map(|doc| {
        let middle = (doc.len() as f32 / 2.0).floor() as usize;

        *doc.get(middle).unwrap() as usize
    }).sum()
}

fn fix_doc(doc: &[u32], ordering: &[(u32, u32)]) -> Vec<u32> {
    let mut fixed_doc: Vec<u32> = Vec::new();

    for num in doc {
        fixed_doc.push(*num);

        if !ordering_is_correct(&fixed_doc, ordering) {
            for i in (1..fixed_doc.len()).rev() {
                fixed_doc.swap(i, i - 1);

                if ordering_is_correct(&fixed_doc, ordering) {
                    break;
                }
            }
        }
    }

    fixed_doc
}

fn ordering_is_correct(doc: &[u32], ordering: &[(u32, u32)]) -> bool {
    for order in ordering {
        let (first, second) = order;

        let first_position = doc.iter().position(|num| num == first);
        let second_position = doc.iter().position(|num| num == second);

        if let Some(first_position) = first_position {
            if let Some(second_position) = second_position {
                if first_position > second_position {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod day5_tests {
    use crate::questions_2024::day5::calculate;

    #[test]
    fn test_input_challenge_1() {
        let input = include_str!("test_input.txt");
        let result = calculate(input, true);

        assert_eq!(result, 143);
    }

    #[test]
    fn input_challenge_1() {
        let input = include_str!("input.txt");
        let result = calculate(input, true);

        println!("{result}")
    }

    #[test]
    fn test_input_challenge_2() {
        let input = include_str!("test_input.txt");
        let result = calculate(input, false);

        assert_eq!(result, 123);
    }

    #[test]
    fn input_challenge_2() {
        let input = include_str!("input.txt");
        let result = calculate(input, false);

        println!("{result}")
    }
}