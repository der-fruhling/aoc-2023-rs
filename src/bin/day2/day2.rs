use std::{fs, env, cmp::{min, max}};

const MAX_R: u32 = 12;
const MAX_G: u32 = 13;
const MAX_B: u32 = 14;

#[derive(Debug, Default, Eq, PartialEq)]
struct GameSet {
    r: u32,
    g: u32,
    b: u32
}

impl GameSet {
    fn possible(&self) -> bool {
        self.r <= MAX_R && self.g <= MAX_G && self.b <= MAX_B
    }
    
    fn mult(self) -> u32 {
        self.r * self.g * self.b
    }
}

impl From<&str> for GameSet {
    // 3 blue, 4 red
    // 1 red, 2 green, 6 blue
    // 2 green
    fn from(value: &str) -> Self {
        let mut game_set = GameSet::default();
        let count_colors = value.split(", ");
        
        for count_color_string in count_colors {
            let count_color: Vec<_> = count_color_string.split(" ").collect();
            
            let &[count_str, color] = &count_color[..] else { unimplemented!() };
            let count = u32::from_str_radix(count_str, 10)
                .expect("Invalid number!");
            
            match color {
                "red" => game_set.r += count,
                "green" => game_set.g += count,
                "blue" => game_set.b += count,
                other => unimplemented!("No color {}", other)
            }
        }
        
        game_set
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: u32,
    game_sets: Vec<GameSet>
}

impl Game {
    fn possible(&self) -> bool {
        self.game_sets.iter()
            .map(|game_set: &GameSet| game_set.possible())
            .reduce(|acc, b| acc && b)
            .unwrap_or_default() /* false */
    }
    
    fn minimum_game_set(self) -> GameSet {
        self.game_sets.into_iter()
            .reduce(|acc, b| GameSet {
                r: max(acc.r, b.r),
                g: max(acc.g, b.g),
                b: max(acc.b, b.b)
            }).unwrap()
    }
}

impl From<&str> for Game {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green    
    fn from(value: &str) -> Self {
        let parts: Vec<_> = value.split(": ").collect();
        
        let &[game_and_number, game_sets_str] = &parts[..] else { unimplemented!() };
        
        let game_sets: Vec<_> = game_sets_str.split("; ")
            .map(|x| GameSet::from(x))
            .collect();
        
        let id = u32::from_str_radix(&game_and_number.replace("Game ", "")[..], 10)
            .expect("Invalid number!");
        
        Self { id, game_sets }
    }
}

#[derive(Debug)]
struct Conundrum {
    games: Vec<Game>
}

impl Conundrum {
    fn sum_of_possible_games(&self) -> u32 {
        self.games.iter()
            .filter(|game| game.possible())
            .map(|game| game.id)
            .sum()
    }
}

impl From<Vec<Game>> for Conundrum {
    fn from(value: Vec<Game>) -> Self {
        Self { games: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_game_set_possible() {
        assert_eq!(true, GameSet {
            r: 1,
            g: 2,
            b: 3
        }.possible());
        
        assert_eq!(false, GameSet {
            r: 20,
            g: 2,
            b: 3
        }.possible());
    }

    #[test]
    fn test_game_set_parse() {
        const SAMPLE: &str = "1 red, 2 green, 6 blue";
        
        let game_set = GameSet::from(SAMPLE);
        
        assert_eq!(GameSet { r: 1, g: 2, b: 6 }, game_set);
    }

    #[test]
    fn test_game_parse() {
        assert_eq!(Game {
            id: 1,
            game_sets: vec![
                GameSet { r: 4, g: 0, b: 3 },
                GameSet { r: 1, g: 2, b: 6 },
                GameSet { r: 0, g: 2, b: 0 }
            ]
        }, Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"))
    }

    #[test]
    fn test_game_possible() {
        let game = Game {
            id: 42,
            game_sets: vec![
                GameSet {
                    r: 2,
                    g: 3,
                    b: 4
                }
            ]
        };

        assert_eq!(true, game.possible());
    }

    #[test]
    fn test_game_possible_with_max_values() {
        let game = Game {
            id: 42,
            game_sets: vec![
                GameSet {
                    r: MAX_R,
                    g: MAX_G,
                    b: MAX_B
                }
            ]
        };

        assert_eq!(true, game.possible());
    }
    
    #[test]
    fn test_game_impossible() {
        let game = Game {
            id: 42,
            game_sets: vec![
                GameSet {
                    r: 42,
                    g: 53,
                    b: 64
                }
            ]
        };
        
        assert_eq!(false, game.possible());
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_name = &args[1];
    
    let bytes = std::fs::read(file_name)
        .expect("Failed to read file!");
    
    let string = String::from_utf8(bytes).expect("Invalid file contents! Not UTF-8?");
    
    let conundrum: Conundrum = string
        .lines()
        .filter(|v| !v.is_empty())
        .map(|line| Game::from(line))
        .collect::<Vec<Game>>()
        .into();
    
//    println!("{:#?}", conundrum);
    println!("Sum of all possible games => {}", conundrum.sum_of_possible_games());
    
    let minimums: u32 = conundrum.games.into_iter()
        .map(|game| game.minimum_game_set().mult())
        .sum();
    
    println!("{:#?}", minimums);
}
