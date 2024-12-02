use std::{cmp, collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file");
    let result = parse_input(&input);
    println!("{result}");
}

fn parse_input(input: &str) -> u32 {
    let hands: Vec<PokerHand> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            PokerHand::new(hand.to_string(), bid.parse().unwrap())
        })
        .collect();

    let sorted = sort_poker_hands(hands);

    let result: Vec<u32> = sorted
        .into_iter()
        .enumerate()
        .map(|(i, x)| (i as u32 + 1) * x.bid)
        .collect();

    result.iter().sum()
}

fn sort_poker_hands(mut hands: Vec<PokerHand>) -> Vec<PokerHand> {
    // bubble sort
    let vec_len = hands.len();
    let mut swapped;

    for i in 0..vec_len - 1 {
        swapped = false;
        for j in 0..vec_len - i - 1 {
            if hands[j].is_bigger(&hands[j + 1]) {
                let temp = hands[j].clone();
                hands[j] = hands[j + 1].clone();
                hands[j + 1] = temp;
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }

    hands
}

#[derive(Debug, Clone)]
struct PokerHand {
    hand: String,
    bid: u32,
    htype: usize,
}

impl PokerHand {
    fn new(hand: String, bid: u32) -> Self {
        let mut hand = Self {
            hand,
            bid,
            htype: 0,
        };
        hand.determine_type();
        hand
    }

    fn is_bigger(&self, other: &Self) -> bool {
        match self.htype.cmp(&other.htype) {
            cmp::Ordering::Less => return false,
            cmp::Ordering::Greater => return true,
            cmp::Ordering::Equal => {}
        }

        for i in 0..self.hand.len() {
            let card_hand_1 = PokerHand::get_card_value(self.hand.chars().nth(i).unwrap());
            let card_hand_2 = PokerHand::get_card_value(other.hand.chars().nth(i).unwrap());
            match card_hand_1.cmp(&card_hand_2) {
                cmp::Ordering::Less => return false,
                cmp::Ordering::Greater => return true,
                cmp::Ordering::Equal => {}
            }
        }
        false
    }

    fn get_card_value(c: char) -> u32 {
        let card_values = HashMap::from([('T', 10), ('J', 11), ('Q', 12), ('K', 13), ('A', 14)]);
        let num = match c.is_ascii_digit() {
            true => c.to_digit(10),
            false => Some(card_values[&c]),
        };
        num.unwrap()
    }

    // Method to count the number of equal cards and determines the type based of that
    fn determine_type(&mut self) {
        let num_chars = self.hand.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let hand: Vec<&i32> = num_chars.values().collect();

        if hand.len() == 1 {
            // Five of a kind
            self.htype = 6;
        } else if hand.len() == 2 {
            if hand.contains(&&4) {
                // Four of a kind
                self.htype = 5;
            } else {
                // Full house
                self.htype = 4;
            }
        } else if hand.len() == 3 {
            if hand.contains(&&3) {
                // Three of a kind
                self.htype = 3;
            } else {
                // Two Pair
                self.htype = 2;
            }
        } else if hand.len() == 4 {
            // One pair
            self.htype = 1;
        } else {
            self.htype = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQAA 483
AAAAA 483";
        assert_eq!(parse_input(input), 1111);
    }

    #[test]
    fn get_card_value_test() {
        assert_eq!(PokerHand::get_card_value('2'), 2);
        assert_eq!(PokerHand::get_card_value('9'), 9);
        assert_eq!(PokerHand::get_card_value('T'), 10);
        assert_eq!(PokerHand::get_card_value('J'), 11);
        assert_eq!(PokerHand::get_card_value('Q'), 12);
        assert_eq!(PokerHand::get_card_value('K'), 13);
        assert_eq!(PokerHand::get_card_value('A'), 14);
    }

    #[test]
    fn is_bigger_test() {
        let hand1 = PokerHand::new("32T3K".to_string(), 765);
        let hand2 = PokerHand::new("T55J5".to_string(), 765);

        assert_eq!(hand1.is_bigger(&hand2), false);

        let hand1 = PokerHand::new("QQQJA".to_string(), 765);
        let hand2 = PokerHand::new("T55J5".to_string(), 765);

        assert_eq!(hand1.is_bigger(&hand2), true);

        let hand1 = PokerHand::new("3233K".to_string(), 765);
        let hand2 = PokerHand::new("T55J5".to_string(), 765);

        assert_eq!(hand1.is_bigger(&hand2), false);

        let hand1 = PokerHand::new("KKKKK".to_string(), 765);
        let hand2 = PokerHand::new("AQQQQ".to_string(), 765);

        assert_eq!(hand1.is_bigger(&hand2), true);
    }
}
