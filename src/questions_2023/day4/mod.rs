use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Card {
    card_number: usize,
    winning_numbers: Vec<usize>,
    your_numbers: Vec<usize>,
    points: usize,
    correct_numbers: usize,
}
fn calculate(input: &str) -> (usize, usize) {
    let mut cards: Vec<Card> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut card = Card {
                card_number: 0,
                winning_numbers: Vec::new(),
                your_numbers: Vec::new(),
                points: 0,
                correct_numbers: 0,
            };

            let mut splitted_line = line.split(": ");

            card.card_number = splitted_line
                .next()
                .unwrap()
                .replace("Card", "")
                .trim()
                .parse::<usize>()
                .unwrap();

            let mut splitted_numbers = splitted_line.next().unwrap().split("|");

            card.winning_numbers = splitted_numbers
                .next()
                .unwrap()
                .split(" ")
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            card.your_numbers = splitted_numbers
                .next()
                .unwrap()
                .split(" ")
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            card
        })
        .map(|card| {
            let correct_numbers = card
                .your_numbers
                .iter()
                .filter(|your_number| card.winning_numbers.contains(your_number))
                .count();

            let points = card
                .your_numbers
                .iter()
                .filter(|your_number| card.winning_numbers.contains(your_number))
                .enumerate()
                .fold(0, |acc, (index, _)| if index == 0 { 1 } else { 2 * acc });

            Card {
                correct_numbers,
                points,
                ..card
            }
        })
        .collect();

    let mut amounts = vec![1; cards.len()];

    cards.iter().enumerate().for_each(|(index, card)| {
        let correct_numbers = card.correct_numbers;

        for x in 0..*amounts.get(index).unwrap() {
            for i in 1..correct_numbers + 1 {
                let card_index = index + i;

                *amounts.get_mut(card_index).unwrap() += 1;
            }
        }
    });

    let total_cards = amounts.iter().fold(0, |acc, card| acc + card);

    let total_points = cards.iter().fold(0, |acc, card| acc + card.points);

    (total_points, total_cards)
}

#[cfg(test)]
mod day1_tests {
    use crate::questions_2023::day4::calculate;

    #[test]
    fn test_input_challenge() {
        let input = include_str!("test_input.txt");
        let result = calculate(input);

        assert_eq!(result, (13, 30));
    }

    #[test]
    fn input_challenge() {
        let input = include_str!("input.txt");
        let result = calculate(input);

        println!("Challenge 0: {}, challenge 1: {}", result.0, result.1)
    }
}
