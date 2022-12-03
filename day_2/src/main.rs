use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::File::open(args.get(1).expect("Please specify file!").clone()).expect("Failed to open file.");
    let reader = io::BufReader::new(file);

    /* read file as iterated list of strings */
    let guide = reader.lines().take_while(Result::is_ok).map(Result::unwrap);
    let mut pt1_total = 0;
    let mut pt2_total = 0;

    /* loop through each round */
    for line in guide {
        println!("{}", line);
        let mut iter = line.split(" ");
    
        let ops = iter.next().unwrap();
        let player = iter.next().unwrap();
        /*
         * A X rock (1 pt)
         * B Y paper (2 pt)
         * C Z scissors (3 pt)
         */
        pt1_total += match (ops, player) {
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
        };

        /*
         * A rock (1)
         * B paper (2)
         * C scissors (3)
         * X lose
         * Y draw
         * Z win
         */
        pt2_total += match (ops, player) {
        ("A", "X") => 3 + 0, /* scissors */
        ("B", "X") => 1 + 0, /* rock */
        ("C", "X") => 2 + 0, /* paper */
        ("A", "Y") => 1 + 3, /* rock */
        ("B", "Y") => 2 + 3, /* paper */
        ("C", "Y") => 3 + 3, /* scissors */
        ("A", "Z") => 2 + 6, /* paper */
        ("B", "Z") => 3 + 6, /* scissors */
        ("C", "Z") => 1 + 6, /* rock */
        _ => 0,        
        };
    }

    println!("Pt1: {}, Pt2: {}", pt1_total, pt2_total);
}