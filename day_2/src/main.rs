use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::File::open(args.get(1).expect("Please specify file!").clone()).expect("Failed to open file.");
    let reader = io::BufReader::new(file);

    /* read file as iterated list of strings */
    let guide = reader.lines().take_while(Result::is_ok).map(Result::unwrap);
    let mut total = 0;

    /* loop through each round */
    for line in guide {
        println!("{}", line);
        let mut iter = line.split(" ");
    
        /*
         * A X rock (1 pt)
         * B Y paper (2 pt)
         * C Z Scissors (3 pt)
         */
        total += match (iter.next().unwrap(), iter.next().unwrap()) {
        ("A", "X") => 1 + 3, /* draw */
        ("B", "X") => 1 + 0, /* loss */
        ("C", "X") => 1 + 6, /* win */
        ("A", "Y") => 2 + 6, /* win */
        ("B", "Y") => 2 + 3, /* draw */
        ("C", "Y") => 2 + 0, /* loss */
        ("A", "Z") => 3 + 0, /* loss */
        ("B", "Z") => 3 + 6, /* win */
        ("C", "Z") => 3 + 3, /* draw */
        _ => 0,
        }
    }

    println!("{}", total);
}