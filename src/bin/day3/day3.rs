use std::{ops::Add, env, mem::MaybeUninit, rc::{Rc, Weak}, marker::PhantomData, cell::RefCell, borrow::{Borrow, BorrowMut}, hash::Hash};
use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position(i32, i32);

impl Position {
    pub fn x(&self) -> i32 {
        self.0
    }
    
    pub fn y(&self) -> i32 {
        self.1
    }
}

impl Add for Position {
    type Output = Position;
    
    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<'a> Add<&'a Position> for &'a Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone)]
struct Grid {
    grid_width: usize,
    grid_height: usize,
    entries: RefCell<Vec<Vec<Rc<Entry>>>>
}

impl Grid {
    pub fn entries(&self) -> impl Iterator<Item = Rc<Entry>> {
        self.entries.borrow().clone().into_iter().flatten()
    }
    
    pub fn entry(&self, position: Position) -> Option<Rc<Entry>> {
        let Position(x, y) = position;
        
        if x < 0 || y < 0 || x >= self.grid_width as i32 || y >= self.grid_height as i32 {
            return None;
        }
        
        Some(self.entries.borrow()[y as usize][x as usize].clone())
    }
    
    pub fn candidates(&self) -> impl Iterator<Item = Candidate> {
        self.entries()
            .filter(|e| e.is_first_digit())
            .map(|start_entry| {
                let entries = start_entry.contiguous_digit_entries();
                
                let string = entries.iter()
                    .map(|v| v.character)
                    .fold(String::new(), |acc, c| acc + &String::from(c)[..]);
                
                Candidate {
                    entries,
                    number: u32::from_str_radix(&string[..], 10).unwrap()
                }
            })
    }
    
    pub fn part_numbers(&self) -> impl Iterator<Item = Candidate> {
        self.candidates()
            .filter(|v| v.is_part_number())
    }
    
    pub fn gears(&self) -> impl Iterator<Item = Rc<Entry>> {
        self.entries()
            .filter(|e| e.is_gear())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Candidate {
    entries: Vec<Rc<Entry>>,
    number: u32
}

impl Candidate {
    pub fn start_position(&self) -> &Position {
        &self.entries[0].position
    }
    
    pub fn is_part_number(&self) -> bool {
        self.entries.iter().fold(false, |acc, b| acc || b.is_part_number_trigger())
    }
}

#[derive(Debug, Clone)]
struct Entry {
    grid: Weak<Grid>,
    character: char,
    position: Position
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.character == other.character && self.position == other.position
    }
}

impl Eq for Entry {}

impl Hash for Entry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.character.hash(state);
        self.position.hash(state);
    }
}

impl Entry {
    const ADJACENT_POSITIONS: [Position; 8] = [
        Position(-1, -1),
        Position(-1,  0),
        Position(-1,  1),
        Position( 0, -1),
        Position( 0,  1),
        Position( 1, -1),
        Position( 1,  0),
        Position( 1,  1)
    ];
    
    const GEAR_SYMBOL: char = '*';
    
    pub fn adjacent_entries(&self) -> Vec<Rc<Entry>> {
        let grid = self.grid.upgrade().unwrap();
        let vec = Self::ADJACENT_POSITIONS.iter().filter_map(|pos| grid.entry(&self.position + pos)).collect();
        
        vec
    }
    
    pub fn is_part_number_trigger(&self) -> bool {
        self.adjacent_entries().into_iter().fold(false, |acc, b| acc || b.is_symbol())
    }
    
    #[inline]
    pub fn is_symbol(&self) -> bool {
        !self.character.is_ascii_digit() && self.character != '.'
    }
    
    #[inline]
    pub fn is_gear(&self) -> bool {
        self.character == Self::GEAR_SYMBOL
    }
    
    #[inline]
    pub fn is_digit(&self) -> bool {
        self.character.is_ascii_digit()
    }
    
    pub fn left_entry(&self) -> Option<Rc<Entry>> {
        self.grid.upgrade().unwrap().entry(Position(self.position.x() - 1, self.position.y())).clone()
    }
    
    pub fn contiguous_digit_entries(&self) -> Vec<Rc<Entry>> {
        let y = self.position.y();
        
        let grid = self.grid.upgrade().unwrap();
        
        (self.position.x()..(self.position.x() + 3)).into_iter()
            .filter_map(|x| grid.entry(Position(x, y)))
            .take_while(|entry| entry.is_digit())
            .collect()
    }
    
    pub fn is_first_digit(&self) -> bool {
        let left = self.left_entry();
        self.is_digit() && (left.is_none() || left.is_some_and(|v| !v.is_digit()))
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_name = &args[1];

    let bytes = std::fs::read(file_name)
        .expect("Failed to read file!");

    let string = String::from_utf8(bytes).expect("Invalid file contents! Not UTF-8?");
    
    let mut x = 0i32;
    let mut y = 0i32;
    
    let height = string.lines().count();
    let width = string.lines().nth(0).unwrap().len();
    
    let mut grid = Rc::new(Grid {
        grid_width: width,
        grid_height: height,
        entries: RefCell::new(Vec::new())
    });
    
    *grid.entries.borrow_mut() = string.lines().enumerate().map(|(y, l)| {
        l.chars().enumerate().map(|(x, ch)| {
            Rc::new(Entry {
                grid: Rc::downgrade(&grid),
                character: ch,
                position: Position(x as i32, y as i32)
            })
        }).collect()
    }).collect();
    
    let part_numbers: Vec<_> = grid.part_numbers().collect();
    let part_numbers_u32: Vec<_> = part_numbers.iter().map(|v| v.number).collect(); 
    
    println!("Part numbers: {:#?}", part_numbers_u32);
    println!("Part number sum: {}", part_numbers_u32.iter().sum::<u32>());
    
    let gears: u32 = grid.gears()
        .map(|e| e.adjacent_entries().into_iter()
            .filter_map(|a| part_numbers.iter().find(|p| {
                p.entries.contains(&a)
            }))
            .unique()
            .collect::<Vec<_>>())
        .filter(|a| a.len() == 2)
        .map(|v| v.into_iter().map(|a| a.number).reduce(|acc, b| acc * b).unwrap())
        .sum();

    println!("Gear ratio sum: {}", gears);
}
