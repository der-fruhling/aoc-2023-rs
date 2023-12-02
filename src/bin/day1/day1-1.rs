use std::str::FromStr;

const INPUT: &str = include_str!("day1-jc.txt");

fn main() {
    let lines = INPUT.lines();

    let sum: u32 = lines.map(|line| {
        let first_char = line.chars().find(|c| c.is_ascii_digit());
        let last_char = line.chars().rev().find(|c| c.is_ascii_digit());
        let string = String::from_iter([first_char.unwrap(), last_char.unwrap()]);
        let number = u32::from_str(&string[..]).unwrap();

        println!("{} => {:?}, {:?} => {}", line, first_char, last_char, number);

        number
    }).sum();

    println!("Sum: {}", sum);
}
