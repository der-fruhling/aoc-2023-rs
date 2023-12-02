use std::str::FromStr;

const INPUT: &str = include_str!("day1-jc.txt");

fn number_to_digit_char(number: &str) -> char {
    match number {
        v if v.starts_with(|v: char| v.is_ascii_digit()) => v.chars().last().unwrap(),
        "zero" => '0',
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        other => panic!("Invalid input: {}", other)
    }
}

const DIGITS: &[&str] = &[
    "zero", "0",
    "one", "1",
    "two", "2",
    "three", "3",
    "four", "4",
    "five", "5",
    "six", "6",
    "seven", "7",
    "eight", "8",
    "nine", "9",
];

fn main() {
    let lines = INPUT.lines();

    let sum: u32 = lines.map(|line| {
        let Some((first_char, _)) = DIGITS.iter()
            .filter_map(|v| line.find(v).map(|idx| (number_to_digit_char(v), idx)))
            .min_by(|(_, a), (_, b)| a.cmp(b))
            else { panic!("non-matching pattern in line {}", line) };

        let Some((last_char, _)) = DIGITS.iter()
            .filter_map(|v| line.rfind(v).map(|idx| (number_to_digit_char(v), idx)))
            .max_by(|(_, a), (_, b)| a.cmp(b))
            else { panic!("non-matching pattern in line {}", line) };

        let string = String::from_iter([first_char, last_char]);
        let number = u32::from_str(&string[..]).unwrap();

        println!("{} => {:?}, {:?} => {}", line, first_char, last_char, number);

        number
    }).sum();

    println!("Sum: {}", sum);
}
