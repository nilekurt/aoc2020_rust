use aoc2020_common::cantor_pairing;

use std::collections::HashSet;
use std::io::Read;

fn solve_base(target: u32, values: &[u32]) -> Option<Vec<u32>> {
    for a in values {
        if *a >= target {
            return None;
        }

        let b = target - *a;
        if *a == b || values.binary_search(&b).is_ok() {
            return Some(vec![*a, b]);
        }
    }

    return None;
}

fn solve(
    target: u32,
    depth: u32,
    values: &[u32],
    fail_cache: &mut HashSet<u64>,
) -> Option<Vec<u32>> {
    let index: u64 = cantor_pairing(target, depth);

    if fail_cache.contains(&index) {
        return None;
    }

    if depth <= 2 {
        let result = solve_base(target, values);
        if let None = result {
            fail_cache.insert(index);
        }
        return result;
    }

    for a in values {
        if *a >= target {
            fail_cache.insert(index);
            return None;
        }

        let b = target - *a;
        if let Some(mut what) = solve(b, depth - 1, values, fail_cache) {
            what.push(*a);
            return Some(what);
        }
    }

    fail_cache.insert(index);
    return None;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let input_path = std::path::Path::new(&args[1]);

    let mut input_file = std::fs::File::open(&input_path).expect("Unable to open input file");

    let mut data = String::new();
    input_file
        .read_to_string(&mut data)
        .expect("Failed to open input file");

    let mut unsorted: Vec<u32> = data
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().expect("Parsing failed"))
        .collect();

    // Values must be sorted for the solver to work
    unsorted.sort();

    let entries = unsorted;
    let target: u32 = 2020;
    let mut cache = HashSet::new();

    match solve(target, 2, &entries, &mut cache) {
        None => println!("No solution!"),
        Some(what) => println!("{}", what.into_iter().fold(1 as u64, |a, b| a * b as u64)),
    }
    match solve(target, 3, &entries, &mut cache) {
        None => println!("No solution!"),
        Some(what) => println!("{}", what.into_iter().fold(1 as u128, |a, b| a * b as u128)),
    }
}
