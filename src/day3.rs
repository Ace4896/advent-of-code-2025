use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const EXAMPLE_PATH: &'static str = "inputs/day-3/example.txt";
const INPUT_PATH: &'static str = "inputs/day-3/input.txt";

fn main() {
    println!("----- Day 3 -----");
    println!("Examples:");
    println!(
        "- Part 1: Expected=357, Actual={}",
        solve_part_1(EXAMPLE_PATH)
    );
    println!(
        "- Part 2: Expected=???, Actual={}",
        solve_part_2(EXAMPLE_PATH)
    );
    println!();
    println!("Final Answers:");
    println!("- Part 1: Actual={}", solve_part_1(INPUT_PATH));
    println!("- Part 2: Actual={}", solve_part_2(INPUT_PATH));
    println!("-----------------");
}

fn read_non_empty_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Unable to open input file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok().filter(|line| !line.is_empty()))
}

/// Finds the largest possible joltage from each bank when only two batteries are enabled.
///
/// **Answer**: `17196`
fn solve_part_1(input_path: &str) -> u32 {
    let input_lines = read_non_empty_lines(input_path);
    let mut total = 0;

    for input_line in input_lines {
        let digits = input_line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        let n = digits.len();

        // Find lowest index x of largest digit between [0, n-1]
        let (x, first_digit) = digits
            .iter()
            .enumerate()
            .take(n - 1)
            .min_by(|(idx1, digit1), (idx2, digit2)| {
                digit1.cmp(digit2).reverse().then(idx1.cmp(idx2))
            })
            .unwrap();

        // Find largest digit between [x+1, n]
        let second_digit = digits.iter().skip(x + 1).max().unwrap();

        let max_bank_joltage = (*first_digit * 10) + *second_digit;
        // println!(
        //     "{} -> {}{} ({})",
        //     input_line, *first_digit, *second_digit, max_bank_joltage
        // );

        total += max_bank_joltage;
    }

    total
}

fn solve_part_2(input_path: &str) -> u32 {
    // TODO: Implement
    0
}
