use std::io::Read;

#[macro_use]
extern crate lazy_static;

fn validate1(first: usize, second: usize, c: u8, phrase: &str) -> bool {
    let count = phrase.matches(c as char).count();

    return first <= count && count <= second;
}

fn validate2(first: usize, second: usize, c: u8, phrase: &str) -> bool {
    if first == 0 || second == 0 || first >= second {
        return false;
    }
    let a = first - 1;
    let b = second - 1;
    let chars = phrase.as_bytes();

    return (chars[a] == c as u8) != (chars[b] == c as u8);
}

fn match_entry(input: &str, validator: fn(usize, usize, u8, &str) -> bool) -> bool {
    if input.is_empty() {
        return false;
    }

    lazy_static! {
        static ref RE: regex::Regex =
            regex::Regex::new(r"(\d+)-(\d+) ([[:alpha:]]): (.+)").unwrap();
    }
    for cap in RE.captures_iter(input) {
        let first = cap[1].parse::<usize>().unwrap();
        let second = cap[2].parse::<usize>().unwrap();
        let c = cap[3].as_bytes()[0];
        let phrase = &cap[4];
        return validator(first, second, c, phrase);
    }

    return false;
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

    let mut lines = data.split('\n');
    let part1 = lines.by_ref().filter(|s| match_entry(s, validate1)).count();

    let part2 = data
        .split('\n')
        .filter(|s| match_entry(s, validate2))
        .count();

    print!("{}\n{}\n", part1, part2);
}
