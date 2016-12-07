#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::str;
use std::collections::HashMap;
use std::cmp::Ordering;

mod utils;

lazy_static! {
    static ref re : Regex = Regex::new(
        r"(?P<a>[:alpha:]+)(\[(?P<b>[:alpha:]+)\])?").unwrap();
}

fn count_freq(text: &Vec<char>) -> HashMap<char, i32> {
    let mut freq : HashMap<char, i32> = HashMap::new();

    // count frequences of chars
    for c in text {
        match freq.get_mut(&c) {
            Some(v) => *v += 1,
            None => ()
        };
        if !freq.contains_key(&c) {
            freq.insert(*c, 1);
        }
    }
    return freq;
}

fn sort_by_freq(freq: HashMap<char, i32>) -> Vec<(char, i32)> {
    // order them
    let mut letters: Vec<(char, i32)> = freq.iter().map(|(c, n)| (*c, *n)).collect();
    letters.sort_by(
        |a, b| 
        match a.1.cmp(&b.1) { 
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.0.cmp(&b.0),
            Ordering::Greater => Ordering::Greater,

        });
    return letters;
}

fn check_pair(s: &str) -> bool {
    let cs = s.chars().collect::<Vec<char>>();
    for i in 0..s.len() - 3 {
        let ii : usize = i as usize;
        if cs[ii] == cs[ii + 3] && cs[ii+1] == cs[ii+2] && cs[ii] != cs[ii+1]{
            /*
            print!("{}", cs[ii]);
            print!("{}", cs[ii + 1]);
            print!("{}", cs[ii + 2]);
            println!("{}", cs[ii + 3]);
            */
            return true;
        }
    }
    return false;
}

fn check_pair2(s: &str) -> (bool, Vec<[char; 3]>) {
    let cs = s.chars().collect::<Vec<char>>();
    let mut results : Vec<[char; 3]> = Vec::new();
    for i in 0..s.len() - 2 {
        let ii : usize = i as usize;
        if cs[ii] == cs[ii + 2] && cs[ii] != cs[ii+1]{
            results.push([cs[ii], cs[ii+1], cs[ii+2]]);
        }
    }
    return (results.len() != 0, results);
}

fn main() {
    let filename = &std::env::args().nth(1).unwrap();
    let content : String = utils::read(filename);

    for line in content.lines() {
        if line.trim() == "" {
            continue;
        }

        if ! re.is_match(line) {
            continue;
        }

        let mut valid = false;
        for cap in re.captures_iter(line) { 
            let a: &str = cap.name("a").unwrap();
            let b: &str = cap.name("b").unwrap_or("");
            if b != "" && check_pair(b) {
                valid = false;
                break
            }
            if check_pair(a) {
                valid = true;
            }
        }
        if valid {
            //println!("{}", line);
        }

        let mut as_ : Vec<[char; 3]> = Vec::new();
        let mut bs_ : Vec<[char; 3]> = Vec::new();
        for cap in re.captures_iter(line) { 
            let a: &str = cap.name("a").unwrap();
            let b: &str = cap.name("b").unwrap_or("");
            let a_valid : (bool, Vec<[char; 3]>) = check_pair2(a);
            as_.extend(a_valid.1);

            if b != "" {
                let b_valid : (bool, Vec<[char; 3]>) = check_pair2(b);
                bs_.extend(b_valid.1);
            }
        }

        for a in as_ {
            let must_be_in_b : [char; 3]= [a[1], a[0], a[1]];
            //println!("{}", a.to_vec().into_iter().collect::<String>());
            if bs_.contains(&must_be_in_b) {
                println!("{}", line);
                break;
            }
        }
    }
}
