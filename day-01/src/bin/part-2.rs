use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Error, Result};
use tracing::{info, warn};

fn main() -> Result<()> {
    let lines = load_input("input.txt")?;
    let digits = lines.iter().map(|line| find_all_digits_in_string(line));

    let sum = digits
        .map(|digits| collect_digits(&digits.unwrap()))
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

    let mut cline = line.to_string();

    let mut chars = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
    ];
    // let _chars = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];

    let mut locs = Vec::new();

    // if any of chars in cline replace with digit
    for (i, &c) in chars.iter().enumerate() {
        if line.contains(c) {
            // get location of c in line
            let loc = line.find(c).unwrap();
            println!("Found {} at {}", c, loc);

            // add c to index vector in order using loc
            digits.push(i as u32 + 1);
            locs.push(loc);
        }
    }
    println!("Digits: {:?}", digits);
    println!("Index: {:?}", locs);

    for c in line.chars() {
        match c.to_digit(10) {
            // Some(d) => digits.push(d),
            Some(d) => {
                digits.push(d);
                // find location of d in line
                let loc = line.find(c).unwrap();
                locs.push(loc);
            }

            None => warn!("Invalid digit: {}", c),
        }
    }

    println!("Digits: {:?}", digits);
    println!("Index: {:?}", locs);

    // sort digits based on loc.sort()

    // Create a vector of indices (0, 1, 2, ...) and sort it based on the numbers vector
    let mut indices: Vec<usize> = (0..digits.len()).collect();
    indices.sort_by_key(|&i| locs[i]);
    println!("Indices: {:?}", indices);
    // Rearrange the strings vector based on the sorted indices
    let sorted: Vec<u32> = indices.iter().map(|&i| digits[i]).collect::<Vec<_>>();

    println!("Sorted: {:?}", sorted);

    Ok(sorted)
}

// sum first and last digit only
fn collect_digits(digits: &Vec<u32>) -> u32 {
    let sum: u32;

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
        assert_eq!(collect_digits(&digits), 15);
    }

    #[test]
    fn test_find_and_read_numbers() {
        let line = "oneight23fourfive";
        let digits = find_all_digits_in_string(line);
        println!("Digits: {:?}", digits);
        assert_eq!(digits.unwrap(), vec![1, 8, 2, 3, 4, 5]);
    }

    #[test]
    fn test_main() {
        let lines = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        let digits = lines.iter().map(|line| find_all_digits_in_string(line));

        println!("Digits: {:?}", digits.clone().collect::<Vec<_>>());
        let sum = digits
            .map(|digits| collect_digits(&digits.unwrap()))
            .sum::<u32>();

        assert_eq!(sum, 281);
    }
}
