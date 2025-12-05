use std::{
    collections::BTreeMap,
    env,
    fs::{File, read},
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    process,
};

fn main() {
    println!("----- Day 5 -----");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file specified");
        process::exit(1);
    }

    let input_path = &args[1];
    println!("Input File: {}", input_path);

    println!("Part 1: {}", solve_part_1(input_path));
}

#[derive(Default)]
struct Database {
    /// Ingredient ID ranges, indexed by start of each range
    id_ranges: BTreeMap<u64, RangeInclusive<u64>>,
    id_ranges_2: Vec<RangeInclusive<u64>>
}

impl Database {
    pub fn add_id_range(&mut self, id_range: RangeInclusive<u64>) {
        self.id_ranges_2.push(id_range);

        // TODO: Merge ID ranges that overlap
        // let mut merged_start = *id_range.start();
        // let mut merged_end = *id_range.end();
        // loop {

        // }

        // let start = *id_range.start();
        // let end = *id_range.end();
        // if let Some((&nearest_start, nearest_range)) =
        //     self.id_ranges.range(..=id_range.start()).next_back()
        // {
        //     // Find other ranges that overlap
        //     let

        //     let merged_start = start.min(nearest_start);
        //     let merged_end = end.max(*nearest_range.end());
        //     let merged_range = merged_start..=merged_end;

        //     self.id_ranges.remove(&nearest_start);
        //     self.id_ranges.insert(merged_start, merged_range);
        // } else {
        //     self.id_ranges.insert(start, id_range);
        // }
    }

    pub fn contains_id(&self, id: u64) -> bool {
        // Find entry whose key matches or is just lower than this ID
        // self.id_ranges
        //     .range(..=id)
        //     .next_back()
        //     .map(|(_, nearest_range)| nearest_range.contains(&id))
        //     .unwrap_or(false)

        self.id_ranges_2.iter().any(|id_range| id_range.contains(&id))
    }
}

fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Unable to open input file");
    let reader = BufReader::new(file);

    reader.lines().filter_map(|l| l.ok())
}

fn parse_input(input_path: &str) -> (Database, Vec<u64>) {
    let mut input_iter = read_lines(input_path);

    // Parse ID ranges up until 1st empty line
    let mut database = Database::default();

    while let Some(id_range_line) = input_iter.next() {
        if let Some((start_str, end_str)) = id_range_line.trim().split_once("-") {
            let start = start_str.parse::<u64>().unwrap();
            let end = end_str.parse::<u64>().unwrap();

            database.add_id_range(start..=end);
        } else {
            break;
        }
    }

    // Parse available IDs
    let available_ids = input_iter.filter_map(|l| l.trim().parse::<u64>().ok()).collect();

    (database, available_ids)
}

/// Finds how many IDs in the input list are fresh, i.e. are present in the database.
///
/// **Answer**: `798`
fn solve_part_1(input_path: &str) -> usize {
    let (database, available_ids) = parse_input(input_path);

    available_ids.iter()
        .filter(|id| database.contains_id(**id))
        .count()
}
