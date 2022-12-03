use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::File::open(args.get(1).expect("Please specify file!").clone()).expect("Failed to open file.");
    let reader = io::BufReader::new(file);

    /* read file as iterated list of strings */
    let logsheet = reader.lines().take_while(Result::is_ok).map(Result::unwrap);
    let mut top_elves: Vec<i32> = Vec::new();
    let mut score = 0;

    logsheet.for_each(|s| score = if s.is_empty() {
        top_elves.push(score);
        0
    } else {
        score + s.parse::<i32>().unwrap()
    });

    /* ending elf */
    top_elves.push(score);

    top_elves.sort_by(|a, b| b.cmp(a));
    println!("High score: {}, Top three: {}", top_elves[0], top_elves[..3].iter().sum::<i32>())
}
