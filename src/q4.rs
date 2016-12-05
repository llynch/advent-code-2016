
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
    static ref re_inner_braquet : Regex = Regex::new(r"^(?P<encrypted_name>[-\da-zA-Z]+)-(?P<sector_id>\d+)\[(?P<hash>[:alpha:]+)\]$").unwrap();
}

pub struct Room {
    pub encrypted_name: String,
    pub sector_id: i32,
    pub hash: String
}

impl Room {

    fn new(line: &str) -> Room {
        let caps = re_inner_braquet.captures(line).unwrap();

        return Room {
            encrypted_name : caps.name("encrypted_name").unwrap().to_string(),
            sector_id : caps.name("sector_id").unwrap().parse::<i32>().unwrap(),
            hash : caps.name("hash").unwrap().to_string()
        }
    }
}

impl Clone for Room {
    fn clone(&self) -> Room {
        return Room {
            encrypted_name: self.encrypted_name.clone(),
            sector_id: self.sector_id,
            hash: self.hash.clone()
        };
    }
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
    let mut letters: Vec<(char, i32)> = freq.iter().map(|(c, n)| (*c, *n)).collect();
    letters.sort_by(
        |a, b| 
        match b.1.cmp(&a.1) { 
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.0.cmp(&b.0),
            Ordering::Greater => Ordering::Greater,

        });
    return letters;
}

fn check(room: &Room) -> bool {
    // fn (s: String) -> str { return &(s)[..]; }
    let result :Vec<(char, i32)> = sort_by_freq(count_freq(&(room.encrypted_name)[..]));
    let hash_computed : Vec<char> = result.iter().map(|e| e.0).take(5).collect();
    let hash_computed_string: String = hash_computed.into_iter().collect();

    /*
    println!("{}", room.encrypted_name);
    println!("{}", room.hash);
    println!("{}", hash_computed_string);
    */

    return &(room.hash)[..] == hash_computed_string;
}

fn shift(c: char, n: i32) -> char {
    let min = "abcdefghijklmnopqrstuvwxyz".to_string();
    let min_len = min.len();

    if c == '-' {
        return ' ';
    }

    let index = min.chars().position(|r| r == c).unwrap();
    let vc: Vec<char> =  min.chars().collect();
    let new_c: char = vc[(index + n as usize) % min_len];
    return new_c;
}

fn main() {
    // test regexes
    assert_eq!(re_inner_braquet.replace("abcd-cds-312[abc]", "$hash"), "abc");
    assert_eq!(re_inner_braquet.replace("abcd-cds-312[abc]", "$encrypted_name"), "abcd-cds");
    assert_eq!(re_inner_braquet.replace("abcd-cds-312[abc]", "$sector_id"), "312");

    // test frequent counter
    assert_eq!(
        sort_by_freq(count_freq("ab")), 
        vec!(('a', 1), ('b', 1)));
    assert_eq!(
        sort_by_freq(count_freq("abccd")), 
        vec!(('c', 2), ('a', 1), ('b', 1), ('d', 1)));

    // https://doc.rust-lang.org/std/env/fn.args.html
    let filename = &std::env::args().nth(1).unwrap();
    let content : String = utils::read(filename);
    let mut real_rooms: Vec<Room> = Vec::new();
    for line in content.lines() {
        let room = Room::new(line);
        if check(&room) {
            real_rooms.push(room.clone());
        }
    }

    let mut sum = 0;
    for room in real_rooms {
        sum += room.sector_id;
        let encrypted_name: &str = &(room.encrypted_name)[..];

        let decrypted_name : Vec<char> = encrypted_name.chars().map(|c| shift(c, room.sector_id)).collect();
        let decrypted_name_string: String = decrypted_name.into_iter().collect();
        // use grep to search for specific name. we print them all.
        println!("{}: {}", room.sector_id, decrypted_name_string);
    }
    println!("sum sector_id: {}", sum);
}
