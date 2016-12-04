
/*
to compile and run from root folder.
:! cargo build && cargo run --bin q4 data/q4
 */
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::str;
use std::collections::HashMap;
use std::cmp::Ordering;

mod utils;

lazy_static! {
    static ref re_inner_braquet : Regex = Regex::new(r"^(?P<before>[-\da-zA-Z]+)-(?P<sector_id>\d+)\[(?P<inner>[:alpha:]+)\]$").unwrap();
}

fn count_freq(text: &str) -> HashMap<char, i32> {
    let mut freq : HashMap<char, i32> = HashMap::new();

    // count frequences of chars
    for c in text.chars() {
        if c == '-' {
            continue;
        }
        
        match freq.get_mut(&c) {
            Some(v) => *v += 1,
            None => ()
        };
        if !freq.contains_key(&c) {
            freq.insert(c, 1);
        }
    }
    return freq;
}

fn sort_by_freq(freq: HashMap<char, i32>) -> Vec<(char, i32)> {
    // order them
    let mut letters: Vec<(char, i32)> = Vec::new();
    letters = freq.iter().map(|(c, n)| (c.clone(), n.clone())).collect();
    letters.sort_by(
        |a, b| 
        match b.1.cmp(&a.1) { 
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.0.cmp(&b.0),
            Ordering::Greater => Ordering::Greater,

        });
    return letters;
}

fn check(line: &str) -> bool {
    // fn (s: String) -> str { return &(s)[..]; }
    let hash : &str = &(re_inner_braquet.replace(line, "$inner"))[..];
    let before : &str = &(re_inner_braquet.replace(line, "$before"))[..];

    let result :Vec<(char, i32)> = sort_by_freq(count_freq(before));
    let hash_computed : Vec<char> = result.iter().map(|e| e.0).take(5).collect();
    let hash_computed_string: String = hash_computed.into_iter().collect();

    /*
    println!("{}", line);
    println!("{}", hash);
    println!("{}", hash_computed_string);
    */

    return hash == hash_computed_string;
}

fn shift(c: char, n: i32) -> char {
    let min = "abcdefghijklmnopqrstuvwxyz".to_string();
    let caps = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

    if c == '-' {
        return ' ';
    }

    if min.contains(c) {
        let index = min.chars().position(|r| r == c).unwrap();
        let vc: Vec<char> =  min.chars().collect();
        let new_c: char = vc[(index + n as usize) % min.len()];
        return new_c;
    }

    return '/';
}

fn main() {
    // test regexes
    assert_eq!(re_inner_braquet.replace("abcd-cds-312[abc]", "$inner"), "abc");
    assert_eq!(re_inner_braquet.replace("abcd-cds-312[abc]", "$before"), "abcd-cds");
    assert_eq!(re_inner_braquet.replace("abcd-cds-312[abc]", "$sector_id"), "312");

    // test frequent counter
    assert_eq!(
        sort_by_freq(count_freq("ab")), 
        vec!(('a', 1), ('b', 1)));
    assert_eq!(
        sort_by_freq(count_freq("abccd")), 
        vec!(('c', 2), ('a', 1), ('b', 1), ('d', 1)));

    // https://doc.rust-lang.org/std/env/fn.args.html
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let content = utils::read(filename);

    let mut count = 0;
    let mut real_rooms: Vec<&str> = Vec::new();
    for line in content.lines() {
        if check(line) {
            real_rooms.push(line);
            count = count + 1;
        }
    }

    let mut sum = 0;
    for room in real_rooms {
        let sector_id: i32 = re_inner_braquet.replace(room, "$sector_id").parse::<i32>().unwrap();
        sum += sector_id;
        let encrypted_name: &str = &(re_inner_braquet.replace(room, "$before"))[..];

        let mut decrypted_name : Vec<char> = Vec::new();
        for c in encrypted_name.chars() {
            let new_c = shift(c, sector_id);
            decrypted_name.push(new_c)
        }

        let decrypted_name_string: String = decrypted_name.into_iter().collect();
        // use grep to search for specific name. we print them all.
        println!("{}: {}", room, decrypted_name_string);
    }
    println!("sum sector_id: {}", sum);
}
