use std::collections::HashMap;

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn num_matches(&self) -> u32 {
        let mut num_matches = 0;
        for wnum in self.winning_numbers.iter() {
            for num in self.numbers.iter() {
                if wnum == num {
                    num_matches += 1;
                }
            }
        }
        num_matches
    }

    fn points(&self) -> u32 {
        let mut out = 0;
        if self.num_matches() > 0 {
            out = 2_u32.pow(self.num_matches() - 1);
        }
        out
    }
}

fn main() {
    let input1 = include_str!("./input1.txt");
    let output1 = part_1(input1);
    println!("{output1}");
    let input2 = include_str!("./input1.txt");
    let output2 = part_2(input2);
    println!("{output2}");
}

fn part_1(input: &str) -> String {
    let output = process_1(input);
    output
}
fn part_2(input: &str) -> String {
    let output = process_2(input);
    output
}

fn process_1(input: &str) -> String {
    let mut cards: Vec<Card> = Vec::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split(':').collect();
        let id: u32 = split[0]
            .split_whitespace()
            .skip(1)
            .take(1)
            .collect::<Vec<&str>>()[0]
            .parse()
            .unwrap();
        let card_numbers: Vec<&str> = split[1].split('|').collect();
        let winning_numbers: Vec<u32> = card_numbers[0]
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let numbers: Vec<u32> = card_numbers[1]
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        cards.push(Card {
            id,
            winning_numbers,
            numbers,
        })
    }
    let out: u32 = cards.iter().map(|c| c.points()).sum();
    out.to_string()
}

fn process_2(input: &str) -> String {
    let mut cards: Vec<Card> = Vec::new();
    let mut cards_map: HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split(':').collect();
        let id: u32 = split[0]
            .split_whitespace()
            .skip(1)
            .take(1)
            .collect::<Vec<&str>>()[0]
            .parse()
            .unwrap();
        let card_numbers: Vec<&str> = split[1].split('|').collect();
        let winning_numbers: Vec<u32> = card_numbers[0]
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let numbers: Vec<u32> = card_numbers[1]
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        cards.push(Card {
            id,
            winning_numbers,
            numbers,
        });
    }
    for card in cards.iter() {
        {
            let num_copies = cards_map.entry(card.id).or_insert(0);
            *num_copies += 1;
        }
        let num_copies = *cards_map.entry(card.id).or_insert(1);
        let num_matches = card.num_matches();
        for idx in 1..=num_matches {
            let val = cards_map.entry(card.id + idx).or_insert(0);
            *val += num_copies;
        }
    }
    let out: u32 = cards_map.values().sum();
    out.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_1() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "13".to_string())
    }
    #[test]
    fn test_2() {
        let output = part_2(EXAMPLE_TEXT);
        assert_eq!(output, "30".to_string())
    }
}
