/*
to compile and run from root folder.
:! cargo build && cargo run --bin q3 data/q3
 */
use std::io::prelude::*;
use std::fs::File;
extern crate regex;
use regex::Regex;

fn read(filename: &String) -> String {
    let mut file = File::open(filename).unwrap();
    let mut buff: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buff);
    let content = std::str::from_utf8(&buff).unwrap();
    return content.to_string();
}

fn check(sides: Vec<i32>) -> bool {
    return sides[0] + sides[1] > sides[2];
}

fn main() {
    // https://doc.rust-lang.org/std/env/fn.args.html
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let content = read(filename);

    let mut count: i32 = 0;
    let mut nblines: i32 = 0;
    let re = Regex::new(r"[ \t]+").unwrap();
    for line in content.lines() {
        // parse line in vector
        let sides_str: Vec<&str> = re.split(line).collect();

        // remove empty and convert to i32
        let mut sides: Vec<i32> = sides_str.iter().filter(|&s| s.trim() != "").map(|&s| s.parse::<i32>().unwrap()).collect();

        sides.sort();
        if check(sides) {
            count = count + 1;
        }

        nblines = nblines + 1;
    }
    println!("q3a: {}", count);

    let mut count : i32 = 0;
    // we read the file 3 times in columns
    for col in 0..3 {
        let mut iter = content.lines();

        // we take 3 lines with iter so we do it n / 3 times for each columns
        for _ in 0..(nblines / 3) {
            let mut sides: Vec<i32> = Vec::new();
            for _ in 0..3 {
                let line :&str = iter.next().unwrap();

                // parse line in vector
                let sides_str: Vec<&str> = re.split(line).collect();

                // remove empty string and convert to i32
                let sides_i32: Vec<i32> = sides_str.iter().filter(|&s| s.trim() != "").map(|&s| s.parse::<i32>().unwrap()).collect();
                sides.push(sides_i32[col])
            }

            sides.sort();
            assert_eq!(sides.len(), 3);

            //println!("{}, {}, {}", sides[0], sides[1], sides[2]);

            if check(sides) {
                count = count + 1;
            }
        }
    }

    println!("q3b: {}", count);
}
