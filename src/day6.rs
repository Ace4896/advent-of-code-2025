use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    println!("----- Day 6 -----");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file specified");
        process::exit(1);
    }

    let input_path = &args[1];
    println!("Input File: {}", input_path);

    println!("Part 1: {}", solve_part_1(input_path));
    println!("Part 2: {}", solve_part_2(input_path));
}

fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Unable to open input file");
    let reader = BufReader::new(file);

    reader.lines().filter_map(|l| l.ok())
}

/// Calculates the grand total for all problems in the worksheet.
///
/// **Answer**: `4951502530386`
fn solve_part_1(input_path: &str) -> u64 {
    let mut input_lines = read_lines(input_path);

    // Parse numbers that need to be added/multiplied
    let mut number_lines: Vec<Vec<u64>> = Vec::new();
    let mut grand_total = 0;

    while let Some(line) = input_lines.next() {
        let values = line.trim().split_whitespace().collect::<Vec<_>>();

        if let Ok(_) = values.first().unwrap().parse::<u64>() {
            // First value in the line is a number, so add to list of number lines
            let number_line = values
                .into_iter()
                .filter_map(|v| v.parse::<u64>().ok())
                .collect::<Vec<_>>();

            number_lines.push(number_line);
        } else {
            // First value is an operator, so start calculating grand total
            for (problem_idx, &op) in values.iter().enumerate() {
                let problem_nums = number_lines.iter().map(|l| l[problem_idx]);
                let intermediate_total = match op {
                    "+" => problem_nums.sum(),
                    "*" => problem_nums.fold(1, |acc, x| acc * x),
                    _ => panic!("Unexpected operator '{}'", op),
                };

                grand_total += intermediate_total;
            }

            break;
        }
    }

    grand_total
}

/// ???
///
/// **Answer**: `???`
fn solve_part_2(input_path: &str) -> u64 {
    0
}
