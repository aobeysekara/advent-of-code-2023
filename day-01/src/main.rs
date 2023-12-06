use std::fs::File;
use std::io::{BufRead, BufReader};

use tracing::{error, info, warn};

fn main() -> Result<(), std::io::Error> {
    let lines = load_input("input.txt")?;

    let mut cum = 0;

    for line in &lines {
        let digits = find_and_read_numbers(line);
        println!("Digits: {:?}", digits);
        match digits {
            Ok(digits) => {
                let sum = sum_digits(&digits);
                cum += sum;
                println!("Sum: {}", sum);
            }
            Err(error) => {
                error!("Error reading digits: {}", error);
            }
        }
    }
    println!("Cum: {}", cum);
    Ok(())
}

// load input from file
fn load_input(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let mut lines = Vec::new();
    let file = File::open(filename).expect("file not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    info!("Loaded {} lines", lines.len());
    Ok(lines)
}

// read digits from line, convert string to u32
fn find_and_read_numbers(line: &str) -> Result<Vec<u32>, std::io::Error> {
    let mut digits = Vec::new();

    let mut cline = line.to_string();

    let chars = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
    ];
    let _chars = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];
    // convert string numbers to digits using chars
    for char in chars {
        if line.contains(char) {
            println!("char: {}", char);
            // replace string number with digit
            cline = cline.replace(char, _chars[chars.iter().position(|&r| r == char).unwrap()]);
        }
    }
    println!("cline: {}", cline);
    for c in cline.chars() {
        match c.to_digit(10) {
            // Some(d) => digits.push(d),
            Some(d) => {
                digits.push(d);
            }

            None => warn!("Invalid digit: {}", c),
        }
    }

    Ok(digits)
}

// sum first and last digit only
fn sum_digits(digits: &Vec<u32>) -> u32 {
    let mut sum: u32 = 0;

    match digits.len() {
        1 => sum = digits[0] * 11,
        _ => sum = digits[0] * 10 + digits[digits.len() - 1],
    }
    sum
}

// test using inputs_test.txt
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_digits() {
        let digits = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_digits(&digits), 15);
    }

    #[test]
    fn test_find_and_read_numbers() {
        let line = "one23fourfive";
        let digits = find_and_read_numbers(line).unwrap();
        println!("Digits: {:?}", digits);
        assert_eq!(digits, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_main() {
        let lines = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

        let mut cum = 0;

        for line in &lines {
            let digits = find_and_read_numbers(line);
            println!("Digits: {:?}", digits);
            match digits {
                Ok(digits) => {
                    let sum = sum_digits(&digits);
                    cum += sum;
                    println!("Sum: {}", sum);
                }
                Err(error) => {
                    error!("Error reading digits: {}", error);
                }
            }
        }
        assert_eq!(cum, 142);
    }
}
