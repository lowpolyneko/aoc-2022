use std::env;
use std::fs;
use std::io::{self, BufRead};

fn priority(character: char) -> i32 {
    match character {
    'a'..='z' => character as i32 - 'a' as i32 + 1,
    'A'..='Z' => character as i32 - 'A' as i32 + 26 + 1,
    _ => 0,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::File::open(args.get(1).expect("Please specify file!").clone()).expect("Failed to open file.");
    let reader = io::BufReader::new(file);

    let contents: Vec<String> = reader.lines().take_while(Result::is_ok).map(Result::unwrap).collect();
    let mut pt1_total = 0;
    let mut pt2_total = 0;

    for (i, l1) in contents.iter().enumerate() {
        let (s1, s2) = l1.as_str().split_at(l1.len()/2);
        for c in s1.chars() {
            if s2.contains(c) {
                pt1_total += priority(c);
                break;
            }
        }

        if i % 3 == 2 {
            for c in l1.chars() {
                let l2 = contents.get(i-1).unwrap();
                let l3 = contents.get(i-2).unwrap();

                if l2.contains(c) && l3.contains(c) {
                    pt2_total += priority(c);
                    break;
                }
            }
        }
    }

    println!("Pt 1: {}, Pt 2: {}", pt1_total, pt2_total);
}