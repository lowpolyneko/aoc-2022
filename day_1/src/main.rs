use std::cmp;
use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::File::open(args.get(1).expect("Please specify file!").clone()).expect("Failed to open file.");
    let reader = io::BufReader::new(file);

    /* read file as iterated list of strings */
    let logsheet = reader.lines().take_while(Result::is_ok).map(Result::unwrap);
    let mut high_score = 0;
    let mut score = 0;

    logsheet.for_each(|s| score = if s.is_empty() {
        high_score = cmp::max(high_score, score);
        0
    } else {
        score + s.parse::<i32>().unwrap()
    });

    println!("High score: {}", high_score)
}
