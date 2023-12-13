use std::{collections::HashMap, env};

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
enum Card {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, T, J, Q, K, A
}

impl From<char> for Card {
    fn from(value: char) -> Card {
        use Card::*;

        match value {
            'A' => A,
            'K' => K,
            'Q' => Q,
            'J' => J,
            'T' => T,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            other => unimplemented!("Invalid character {:?}", other)
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Kind {
    HighCard, OnePair, TwoPair, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Hand {
    cards: [Card; 5],
    bid: u64
}

impl Hand {
    fn kind(&self) -> Kind {
        let map = self.cards.iter().sorted().rev().fold(HashMap::<Card, u32>::new(), |mut acc, item| {
            *(acc.entry(*item).or_insert(0)) += 1;
            acc
        });
        
        let cards_by_count: Vec<_> = map.iter().sorted_by(|a, b| a.1.cmp(&b.1).reverse()).collect();

        debug_assert!(cards_by_count.len() >= 1);
        
        if *cards_by_count[0].1 == 5 {
            Kind::FiveOfAKind
        } else if *cards_by_count[0].1 == 4  {
            Kind::FourOfAKind
        } else if *cards_by_count[0].1 == 3 && *cards_by_count[1].1 == 2 {
            Kind::FullHouse
        } else if *cards_by_count[0].1 == 3 {
            Kind::ThreeOfAKind
        } else if *cards_by_count[0].1 == 2 && *cards_by_count[1].1 == 2 {
            Kind::TwoPair
        } else if *cards_by_count[0].1 == 2 {
            Kind::OnePair
        } else {
            Kind::HighCard
        }
    }
}

impl<'a> From<&'a str> for Hand {
    fn from(value: &'a str) -> Self {
        let cards = &value[..5];
        let bid = &value[6..];
        
        Self {
            cards: cards.chars().map(Card::from).collect::<Vec<_>>().try_into().unwrap(),
            bid: u64::from_str_radix(bid, 10).unwrap()
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return std::cmp::Ordering::Equal;
        }

        let kind_ord = self.kind().cmp(&other.kind());
        if kind_ord != std::cmp::Ordering::Equal {
            return kind_ord;
        }

        for (a, b) in self.cards.iter().zip(other.cards.iter()) {
            let card_ord = a.cmp(b);
            if card_ord != std::cmp::Ordering::Equal {
                return card_ord;
            }
        }

        // how strange.
        unimplemented!("cmp() fell through?");
    }
}

#[cfg(test)]
mod tests {
    use crate::{Hand, Card::*};

    #[test]
    fn test_cmp_hand_1st() {
        let a = /* 33332 */ Hand { cards: [Three, Three, Three, Three, Two], bid: 0 };
        let b = /* 2AAAA */ Hand { cards: [Two, A, A, A, A], bid: 42 };

        assert!(a > b);
    }

    #[test]
    fn test_cmp_hand_2nd() {
        let a = /* 77888 */ Hand { cards: [Seven, Seven, Eight, Eight, Eight], bid: 0 };
        let b = /* 77788 */ Hand { cards: [Seven, Seven, Seven, Eight, Eight], bid: 42 };

        assert!(a > b);
    }

    #[test]
    fn test_cmp_hand_with_sample_data() {
        let a = Hand { cards: [K, T, J, J, T], bid: 42 };
        let b = Hand { cards: [K, K, Six, Seven, Seven], bid: 3 };

        assert!(b > a);
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_name = &args[1];

    let bytes = std::fs::read(file_name)
        .expect("Failed to read file!");

    let string = String::from_utf8(bytes).expect("Invalid file contents! Not UTF-8?");
    let lines = string.lines();
    
    let hands = lines.map(|line| Hand::from(line)).sorted();
    
    let total_winnings = hands.enumerate().map(|(i, hand)| {
        let rank = i + 1;
        let value = (rank as u64) * hand.bid;

        value
    }).sum::<u64>();
    
    dbg!(total_winnings);
}