use std::{env, collections::{HashSet, HashMap}};

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    copies: u32,
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>
}

impl Card {
    fn my_winning_numbers(&self) -> impl Iterator<Item = &u32> {
        self.winning_numbers.intersection(&self.my_numbers)
    }
    
    fn score(&self) -> u32 {
        let my_winning_numbers: Vec<_> = self.my_winning_numbers().collect();
        if my_winning_numbers.len() == 0 {
            return 0
        }
        
        2u32.pow((my_winning_numbers.len() - 1usize) as u32)
    }
    
    fn winning_number_count(&self) -> u32 {
        self.my_winning_numbers().count() as u32
    }
}

impl From<&str> for Card {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53    
    fn from(value: &str) -> Self {
        let id_str = &value[4..8];
        let Some((winning_numbers_str, my_numbers_str)) = &value[10..].split_once('|') else { unimplemented!("Invalid line {}", value) };
        
        let winning_numbers: HashSet<_> = winning_numbers_str.split(' ').filter(|v| !v.is_empty()).map(|v| u32::from_str_radix(v, 10).unwrap()).collect();
        let my_numbers: HashSet<_> = my_numbers_str.split(' ').filter(|v| !v.is_empty()).map(|v| u32::from_str_radix(v, 10).unwrap()).collect();
        
        Self {
            id: u32::from_str_radix(id_str.trim(), 10).unwrap(),
            copies: 1,
            winning_numbers,
            my_numbers
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_name = &args[1];

    let bytes = std::fs::read(file_name)
        .expect("Failed to read file!");

    let string = String::from_utf8(bytes).expect("Invalid file contents! Not UTF-8?");
    
    let mut cards: Vec<_> = string.lines()
        .map(|v| Card::from(v))
        .collect();
    
    for i in 0..cards.len() {
        let card = cards[i].clone();
        let card_idx = (card.id - 1) as usize;
        let count = card.winning_number_count() as usize;
        
        if count == 0 { continue }
        
        for j in (card_idx + 1)..(card_idx + 1 + count) {
            cards[j].copies += card.copies;
        }
    }
    
    println!("{:?}", cards);
    println!("Sum of scores: {}", cards.iter().map(|v| v.score()).sum::<u32>());
    println!("Sum of copies: {}", cards.iter().map(|v| v.copies).sum::<u32>())
}
