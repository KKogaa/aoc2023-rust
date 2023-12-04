use regex::Regex;
use std::{array, fs};

fn scanWord(position: usize, list_numbers: &[&str], line: &str) -> i32 {
    for i in position..line.len() {}
    return -1;
}

fn evaluate(line: &str) -> bool {
    let list_numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (i, c) in line.chars().enumerate() {
        scanWord(i, list_numbers, line);
    }
    return false;
}

fn main() {
    let file_path = "./src/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let re = Regex::new(r"[0-9]").unwrap();

    let mut sum = 0;
    let mut first_char: Option<char> = None;
    let mut second_char: Option<char> = None;
    for c in contents.chars() {
        if re.is_match(&c.to_string()) {
            if first_char.is_none() {
                first_char = Some(c);
            }
            second_char = Some(c);
        }

        if c == '\n' {
            if let (Some(first), Some(second)) = (first_char, second_char) {
                let string_number = format!("{}{}", first, second);
                let number = string_number.parse::<i32>().unwrap();
                sum += number;
                first_char = None;
                second_char = None;
            }
        }
    }
    println!("final sum: {sum}");
}
