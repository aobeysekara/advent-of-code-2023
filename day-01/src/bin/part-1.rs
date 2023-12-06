use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use tracing::{info, warn};

fn main() -> Result<()> {
    let lines = load_input("input.txt")?;
    let digits = lines.iter().map(|line| find_all_digits_in_string(line));

    let sum = digits
        .map(|digits| sum_digits(&digits.unwrap()))
        .sum::<u32>();
    println!("Sum: {}", sum);

    Ok(())
}

// load input from file
fn load_input(filename: &str) -> Result<Vec<String>> {
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
fn find_all_digits_in_string(line: &str) -> Result<Vec<u32>> {
    let mut digits = Vec::new();

    let cline = line.to_string();

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
        let digits = find_all_digits_in_string(line);
        println!("Digits: {:?}", digits);
        assert_eq!(digits.unwrap(), vec![2, 3]);
    }

    #[test]
    fn test_main() {
        let lines = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

        let digits = lines.iter().map(|line| find_all_digits_in_string(line));

        println!("Digits: {:?}", digits.clone().collect::<Vec<_>>());
        let sum = digits
            .map(|digits| sum_digits(&digits.unwrap()))
            .sum::<u32>();

        assert_eq!(sum, 142);
    }
}
