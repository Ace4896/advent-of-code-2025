use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const EXAMPLE_PATH: &'static str = "inputs/day-2/example.txt";
const INPUT_PATH: &'static str = "inputs/day-2/input.txt";

fn main() {
    println!("----- Day 2 -----");
    println!("Examples:");
    println!(
        "- Part 1: Expected=1227775554, Actual={}",
        solve_part_1(EXAMPLE_PATH)
    );
    println!();
    println!("Final Answers:");
    println!("- Part 1: Actual={}", solve_part_1(INPUT_PATH));
    println!("-----------------");
}

fn read_file_contents(path: &str) -> String {
    let file = File::open(path).expect("Unable to open input file");
    let mut input_reader = BufReader::new(file);

    let mut input_str = String::new();
    input_reader
        .read_line(&mut input_str)
        .expect("No input lines found");

    input_str
}

fn parse_id_ranges<'a>(input: &'a str) -> impl Iterator<Item = (&'a str, &'a str)> {
    input
        .trim()
        .split(",")
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.split_once("-"))
}

/// Determines which IDs contain 2 repeated halves, e.g. 113113, then sums them together.
///
/// **Answer**: `19128774598`
fn solve_part_1(input_path: &str) -> u64 {
    let input = read_file_contents(input_path);
    let id_ranges_iter = parse_id_ranges(&input);

    let mut total = 0;

    for (range_start_str, range_end_str) in id_ranges_iter {
        println!("{}-{}", range_start_str, range_end_str);

        // Determine where to start looking for invalid IDs
        let start_digits = range_start_str.chars().count();
        let start_middle_digit = start_digits / 2;

        let invalid_start = if start_digits % 2 == 0 {
            // Even Case: Split at middle digit
            // - If left >= right, start at left, e.g. 1402 => start at 14
            // - Otherwise, start at left + 1, e.g. 1425 => start at 15
            let (left, right) = range_start_str.split_at(start_middle_digit);
            let left_parsed = left.parse::<u32>().unwrap();

            if left >= right {
                left_parsed
            } else {
                left_parsed + 1
            }
        } else {
            // Odd Case: Start at 10 ^ (middle digit index)
            // e.g. 12345 => start at 100
            10_u32.pow(start_middle_digit as u32)
        };

        // Determine where to stop looking for invalid IDs
        let end_digits = range_end_str.chars().count();
        let end_middle_digit = end_digits / 2;

        let invalid_end = if end_digits % 2 == 0 {
            // Even Case: Split at middle digit
            // - If left <= right, end at left, e.g. 1470 => end at 14
            // - Otherwise, end at left - 1,    e.g. 1402 => end at 13
            let (left, right) = range_end_str.split_at(end_middle_digit);
            let left_parsed = left.parse::<u32>().unwrap();

            if left <= right {
                left_parsed
            } else {
                left_parsed - 1
            }
        } else {
            // Odd Case: Start at 10 ^ (middle digit index) - 1
            // e.g. 12345 => end at 99
            10_u32.pow(end_middle_digit as u32) - 1
        };

        for i in invalid_start..=invalid_end {
            // NOTE: Can definitely do this without string parsing, too lazy LOL
            let invalid_id = format!("{}{}", i, i).parse::<u64>().unwrap();
            println!("- Invalid: {}", invalid_id);

            total += invalid_id;
        }
    }

    total
}
