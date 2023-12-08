use std::{collections::HashMap, iter::zip};

fn main() {
    let input1 = include_str!("./input1.txt");
    let output1 = part_1(input1);
    println!("{output1}");
    let input2 = include_str!("./input1.txt");
    let output2 = part_2(input2);
    println!("{output2}");
}

const VALID_CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const VALID_CARDS_P2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
struct Hand {
    strength: usize,
    cards: [char; 5],
    bet: u32,
}

impl Hand {
    fn should_swap(&self, other: &Hand) -> bool {
        if self.strength > other.strength {
            return true;
        }
        if self.strength == other.strength {
            for (c, other_c) in zip(self.cards, other.cards) {
                if c != other_c {
                    let val = VALID_CARDS.into_iter().rev().position(|r| r == c).unwrap();
                    let other_val = VALID_CARDS
                        .into_iter()
                        .rev()
                        .position(|r| r == other_c)
                        .unwrap();
                    if val < other_val {
                        return false;
                    } else if val > other_val {
                        return true;
                    } else {
                        continue;
                    }
                }
            }
        }
        false
    }
    fn should_swap_p2(&self, other: &Hand) -> bool {
        if self.strength > other.strength {
            return true;
        }
        if self.strength == other.strength {
            for (c, other_c) in zip(self.cards, other.cards) {
                if c != other_c {
                    let val = VALID_CARDS_P2
                        .into_iter()
                        .rev()
                        .position(|r| r == c)
                        .unwrap();
                    let other_val = VALID_CARDS_P2
                        .into_iter()
                        .rev()
                        .position(|r| r == other_c)
                        .unwrap();
                    // println!("{}:{} {}:{}", c, val, other_c, other_val);
                    if val < other_val {
                        return false;
                    } else if val > other_val {
                        return true;
                    } else {
                        continue;
                    }
                }
            }
        }
        false
    }
}

fn hand_strength(cards: [char; 5]) -> usize {
    let mut letter_counts: HashMap<char, u32> = HashMap::new();

    let input_string: String = cards.iter().collect();
    let char_vec: Vec<char> = input_string.chars().collect();
    for c in char_vec {
        *letter_counts.entry(c).or_insert(0) += 1;
    }
    let counts: Vec<u32> = letter_counts.into_values().collect();
    match (
        counts.clone().into_iter().max(),
        counts.clone().into_iter().min(),
        counts.clone().into_iter().filter(|c| *c == 2).count(),
    ) {
        // five of a kind
        (Some(5), Some(5), _) => 6,
        // four of a kind
        (Some(4), Some(1), _) => 5,
        // full house
        (Some(3), Some(2), _) => 4,
        // three of a kind
        (Some(3), Some(1), _) => 3,
        // two pair
        (Some(2), Some(1), 2) => 2,
        // one pair
        (Some(2), Some(1), 1) => 1,
        // high card
        (Some(1), Some(1), _) => 0,
        _ => 10,
    }
}

fn hand_strength_p2(cards: [char; 5]) -> usize {
    let mut letter_counts: HashMap<char, u32> = HashMap::new();

    let input_string: String = cards.iter().collect();
    let char_vec: Vec<char> = input_string.chars().collect();
    for c in char_vec {
        *letter_counts.entry(c).or_insert(0) += 1;
    }
    let counts: Vec<u32> = letter_counts.clone().into_values().collect();
    match (
        counts.clone().into_iter().max(),
        counts.clone().into_iter().min(),
        counts.clone().into_iter().filter(|c| *c == 2).count(),
        letter_counts.get(&'J').unwrap_or(&0).to_owned() as usize,
    ) {
        // five of a kind
        (Some(5), Some(5), 0, 0) => 6,
        (Some(5), Some(5), 0, 5) => 6,
        (Some(4), Some(1), 0, 1) => 6,
        (Some(4), Some(1), 0, 4) => 6,
        (Some(3), Some(2), 1, 2) => 6,
        (Some(3), Some(2), 1, 3) => 6,
        // four of a kind
        (Some(4), Some(1), 0, 0) => 5,
        (Some(3), Some(1), 0, 1) => 5,
        (Some(3), Some(1), 0, 3) => 5,
        (Some(2), Some(1), 2, 2) => 5,
        // full house
        (Some(3), Some(2), 1, 0) => 4,
        (Some(2), Some(1), 2, 1) => 4,
        // three of a kind
        (Some(3), Some(1), 0, 0) => 3,
        (Some(2), Some(1), 1, 1) => 3,
        (Some(2), Some(1), 1, 2) => 3,
        // two pair
        (Some(2), Some(1), 2, 0) => 2,
        // one pair
        (Some(2), Some(1), 1, 0) => 1,
        (Some(1), Some(1), 0, 1) => 1,
        // high card
        (Some(1), Some(1), 0, 0) => 0,
        _ => 10,
    }
}

fn part_1(input: &str) -> String {
    process_1(input)
}
fn part_2(input: &str) -> String {
    process_2(input)
}

fn process_1(input: &str) -> String {
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let cards: [char; 5] = split
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        let bet = split.next().unwrap().parse::<u32>().unwrap();
        let hand = Hand {
            strength: hand_strength(cards),
            cards,
            bet,
        };
        hands.push(hand)
    }
    // hands.sort();
    let mut swap_idx: Option<usize> = None;
    let mut index = 0;
    while index + 1 != hands.len() {
        for (idx, hand) in hands.windows(2).enumerate() {
            index = idx + 1;
            // println!("idx:{}, hand:{:?}", idx, hand);
            if hand[0].should_swap(&hand[1]) {
                swap_idx = Some(idx);
                break;
            }
            swap_idx = None;
        }
        if let Some(idx) = swap_idx {
            // println!("inside swap");
            hands.swap(idx, idx + 1);
        }
    }
    let out: u32 = hands
        .iter()
        .enumerate()
        .map(|(rank, h)| (rank + 1) as u32 * h.bet)
        .sum();

    out.to_string()
}

fn process_2(input: &str) -> String {
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let cards: [char; 5] = split
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        let bet = split.next().unwrap().parse::<u32>().unwrap();
        let hand = Hand {
            strength: hand_strength_p2(cards),
            cards,
            bet,
        };
        hands.push(hand)
    }
    // hands.sort();
    let mut swap_idx: Option<usize> = None;
    let mut index = 0;
    while index + 1 != hands.len() - 1 {
        for (idx, hand) in hands.windows(2).enumerate() {
            // println!("idx:{}, hand:{:?}", idx, hand);
            if hand[0].should_swap_p2(&hand[1]) {
                swap_idx = Some(idx);
                break;
            }
            index = idx;
            swap_idx = None;
        }
        if let Some(idx) = swap_idx {
            // println!("inside swap");
            hands.swap(idx, idx + 1);
        }
    }
    let out: u32 = hands
        .iter()
        .enumerate()
        .map(|(rank, h)| (rank + 1) as u32 * h.bet)
        .sum();

    out.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_1() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "6440".to_string())
    }
    #[test]
    fn test_2() {
        let output = part_2(EXAMPLE_TEXT);
        assert_eq!(output, "5905".to_string())
    }
}
