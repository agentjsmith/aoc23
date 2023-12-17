#![allow(dead_code)]

use rayon::prelude::*;
use rstest::rstest;
use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    score: usize,
    bid: usize,
    string: String,
}

#[derive(Debug)]
struct Puzzle {
    hands: Vec<Hand>,
}

impl Puzzle {
    fn new() -> Self {
        Self { hands: Vec::new() }
    }

    fn input_lines<'a, I>(&mut self, lines: I)
    where
        I: IntoIterator<Item = &'a str>,
    {
        //    let mut lines = lines.into_iter();

        for line in lines {
            let (cards, bid) = line.split_once(' ').unwrap();

            let score = Self::score(cards);
            let bid = bid.parse().unwrap();

            self.hands.push(Hand {
                score,
                bid,
                string: cards.to_string(),
            })
        }
    }

    fn score(cards: &str) -> usize {
        let hand_bonus = dbg!(Self::hand_bonus(cards));
        let card_score = cards
            .chars()
            .map(Self::card_value)
            .fold(hand_bonus, |acc, v| acc * 100 + v);
        card_score
    }

    fn card_value(card: char) -> usize {
        match card {
            '2'..='9' => card.to_digit(10).unwrap() as usize,
            'T' => 10,
            'J' => 1, // jack is now joker - weakest card
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0usize,
        }
    }

    fn answer(&mut self) -> usize {
        self.hands.sort_by_key(|h| h.score);
        self.hands
            .iter()
            .inspect(|x| {
                dbg!(x);
            })
            .enumerate()
            .map(|(i, h)| (i + 1) * h.bid)
            .sum()
    }

    fn hand_bonus(hand: &str) -> usize {
        let mut counts: HashMap<char, u8> = HashMap::new();
        for c in hand.chars() {
            counts.insert(c, counts.get(&c).unwrap_or(&0u8) + 1);
        }

        let &jokers = counts.get(&'J').unwrap_or(&0);
        // put jokers into the next biggest pile
        if jokers > 0 {
            counts.insert('J', 0);
            let biggest_pile = counts
                .iter()
                .max_by_key(|&(_, v)| v)
                .map(|(&k, _)| k)
                .unwrap();

            counts.insert(
                biggest_pile,
                counts.get(&biggest_pile).unwrap_or(&0) + jokers,
            );
        }

        if counts.values().any(|&x| x == 5) {
            6
        } else if counts.values().any(|&x| x == 4) {
            5
        } else if counts.values().any(|&x| x == 3) {
            if counts.values().any(|&x| x == 2) {
                // full house
                4
            } else {
                // three of a kind
                3
            }
        } else if counts.values().filter(|&&x| x == 2).count() == 2 {
            // two pair
            2
        } else if counts.values().any(|&x| x == 2) {
            // one pair
            1
        } else {
            0
        }
    }

    fn solve(&mut self, input: &str) -> usize {
        self.input_lines(input.lines());
        self.answer()
    }
}

fn main() {
    let mut puzzle = Puzzle::new();
    let answer = puzzle.solve(include_str!("../../input.txt"));

    println!("The answer is {}", answer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[rstest]
    #[case("23456", 0)]
    #[case("22456", 1)]
    #[case("22336", 2)]
    #[case("22256", 3)]
    #[case("23332", 4)]
    #[case("22522", 5)]
    #[case("22222", 6)]
    fn test_hand_bonus(#[case] hand: &str, #[case] expected: usize) {
        assert_eq!(expected, Puzzle::hand_bonus(hand));
    }

    #[rstest]
    #[case("QQQQ2", "JKKK2")]
    #[case("KTJJT", "QQQJA")]
    #[case("QQQJA", "T55J5")]
    #[case("T55J5", "KK677")]
    #[case("KK677", "32T3K")]
    fn test_left_beats_right(#[case] left: &str, #[case] right: &str) {
        assert!(Puzzle::score(left) > Puzzle::score(right));
    }

    #[test]
    fn test_solving() {
        let mut puzzle = Puzzle::new();
        let answer = puzzle.solve(include_str!("../../test.txt"));
        assert_eq!(5905, answer);
    }
}
