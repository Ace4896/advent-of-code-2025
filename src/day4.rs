use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const EXAMPLE_PATH: &'static str = "inputs/day-4/example.txt";
const INPUT_PATH: &'static str = "inputs/day-4/input.txt";

fn main() {
    println!("----- Day 4 -----");
    println!("Examples:");
    println!(
        "- Part 1: Expected=13, Actual={}",
        solve_part_1(EXAMPLE_PATH)
    );
    println!();
    println!("Final Answers:");
    println!("- Part 1: Actual={}", solve_part_1(INPUT_PATH));
    println!("-----------------");
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CellType {
    Empty,
    PaperRoll,
}

impl TryFrom<char> for CellType {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use CellType::*;

        match value {
            '.' => Ok(Empty),
            '@' => Ok(PaperRoll),
            _ => Err("Invalid grid cell"),
        }
    }
}

fn read_non_empty_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Unable to open input file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok().filter(|line| !line.is_empty()))
}

/// Finds number of paper rolls that have fewer than 4 adjacent paper rolls in the 8 surrounding cells.
///
/// **Answer**: `1518`
fn solve_part_1(input_path: &str) -> u32 {
    let input_lines = read_non_empty_lines(input_path);

    // Parse the input grid
    let mut grid: Vec<Vec<CellType>> = Vec::new();

    for input_line in input_lines {
        let row = input_line
            .trim()
            .chars()
            .map(|c| CellType::try_from(c).unwrap())
            .collect::<Vec<_>>();

        grid.push(row);
    }

    let mut total_accessible = 0;
    const ACCESSIBLE_THRESHOLD: usize = 4;

    for row_idx in 0..grid.len() {
        let current_row = &grid[row_idx];
        let row_above = if row_idx > 0 {
            Some(&grid[row_idx - 1])
        } else {
            None
        };

        let row_below = if row_idx < grid.len() - 1 {
            Some(&grid[row_idx + 1])
        } else {
            None
        };

        for col_idx in 0..current_row.len() {
            let current_cell = current_row[col_idx];
            if current_cell != CellType::PaperRoll {
                continue;
            }

            let mut adjacent_occupied = 0;

            // Count occupied cells to left
            if col_idx > 0 {
                if row_above.is_some_and(|r| r[col_idx - 1] == CellType::PaperRoll) {
                    adjacent_occupied += 1;
                }

                if current_row[col_idx - 1] == CellType::PaperRoll {
                    adjacent_occupied += 1;
                }

                if row_below.is_some_and(|r| r[col_idx - 1] == CellType::PaperRoll) {
                    adjacent_occupied += 1;
                }
            }

            // Count occupied cells above + below
            if row_above.is_some_and(|r| r[col_idx] == CellType::PaperRoll) {
                adjacent_occupied += 1;
            }

            if row_below.is_some_and(|r| r[col_idx] == CellType::PaperRoll) {
                adjacent_occupied += 1;
            }

            // Count occupied cells to right
            if col_idx < current_row.len() - 1 {
                if row_above.is_some_and(|r| r[col_idx + 1] == CellType::PaperRoll) {
                    adjacent_occupied += 1;
                }

                if current_row[col_idx + 1] == CellType::PaperRoll {
                    adjacent_occupied += 1;
                }

                if row_below.is_some_and(|r| r[col_idx + 1] == CellType::PaperRoll) {
                    adjacent_occupied += 1;
                }
            }

            if adjacent_occupied < ACCESSIBLE_THRESHOLD {
                total_accessible += 1;
            }
        }
    }

    total_accessible
}
