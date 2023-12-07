use crate::puzzle::Puzzle;
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        self.solve_generic(compare_hands)
    }

    fn solve_part_2(&self) -> String {
        self.solve_generic(compare_hands_with_wildcards)
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }

    fn parse_hands(&self) -> Vec<Hand> {
        self.input
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let cards = parts.next().unwrap();
                let bid = parts.next().unwrap().parse::<i32>().unwrap();
                Hand {
                    cards: cards.to_string(),
                    bid,
                }
            })
            .collect()
    }

    fn solve_generic<F>(&self, compare: F) -> String
    where
        F: Fn(&str, &str) -> Ordering,
    {
        let mut hands = self.parse_hands();
        hands.sort_unstable_by(|a, b| compare(&a.cards, &b.cards));
        hands
            .into_iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i as i32 + 1))
            .sum::<i32>()
            .to_string()
    }
}

struct Hand {
    cards: String,
    bid: i32,
}

fn card_counts(cards: &str) -> Vec<(char, u8)> {
    let mut counts = HashMap::new();
    for c in cards.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }
    let mut counts_vec: Vec<(char, u8)> = counts.into_iter().collect();
    counts_vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    counts_vec
}

fn hand_strength(cards: &str) -> u8 {
    let counts = card_counts(cards);
    match counts[0].1 {
        5 => 7,                      // Five of a kind
        4 => 6,                      // Four of a kind
        3 if counts.len() == 2 => 5, // Full house
        3 => 4,                      // Three of a kind
        2 if counts.len() == 3 => 3, // Two pair
        2 => 2,                      // One pair
        _ => 1,                      // High card
    }
}

fn hand_strength_with_wildcards(cards: &str) -> u8 {
    if !cards.contains('J') {
        return hand_strength(cards);
    }
    let counts = card_counts(&cards.replace('J', ""));
    if counts.is_empty() {
        return 7; // All wildcards (five of a kind)
    }
    let replaced = cards.replace('J', &counts[0].0.to_string());
    hand_strength(&replaced)
}

fn compare_hands(cards_1: &str, cards_2: &str) -> Ordering {
    let strength_1 = hand_strength(cards_1);
    let strength_2 = hand_strength(cards_2);
    if strength_1 != strength_2 {
        return strength_1.cmp(&strength_2);
    }
    for (c1, c2) in cards_1.chars().zip(cards_2.chars()) {
        if c1 != c2 {
            let i1 = "23456789TJQKA".find(c1).unwrap();
            let i2 = "23456789TJQKA".find(c2).unwrap();
            return i1.cmp(&i2);
        }
    }
    Ordering::Equal
}

fn compare_hands_with_wildcards(cards_1: &str, cards_2: &str) -> Ordering {
    let strength_1 = hand_strength_with_wildcards(cards_1);
    let strength_2 = hand_strength_with_wildcards(cards_2);
    if strength_1 != strength_2 {
        return strength_1.cmp(&strength_2);
    }
    for (c1, c2) in cards_1.chars().zip(cards_2.chars()) {
        if c1 != c2 {
            let i1 = "J23456789TQKA".find(c1).unwrap();
            let i2 = "J23456789TQKA".find(c2).unwrap();
            return i1.cmp(&i2);
        }
    }
    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "32T3K 765\n\
            T55J5 684\n\
            KK677 28\n\
            KTJJT 220\n\
            QQQJA 483";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "6440");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/07")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "251106089");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "32T3K 765\n\
            T55J5 684\n\
            KK677 28\n\
            KTJJT 220\n\
            QQQJA 483";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "5905");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/07")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "249620106");
    }
}
