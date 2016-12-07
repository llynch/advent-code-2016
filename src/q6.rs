use std::str;
use std::collections::HashMap;
use std::cmp::Ordering;

mod utils;
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

fn main() {
    let filename = &std::env::args().nth(1).unwrap();
    let content : String = utils::read(filename);

    let mut positions: Vec<Vec<char>> = Vec::new();
    for i in 0..8 {
        positions.push(Vec::new());
    }
    for line in content.lines() {
        if line.trim() == "" {
            continue;
        }
        for (i, c) in line.chars().enumerate() {
            let ref mut col: Vec<char> = positions[i as usize];
            col.push(c);
        }
    }
    for i in 0..8 {
        let ref col: Vec<char> = positions[i as usize];
        let freq = count_freq(col);
        let sorted_freq : Vec<(char, i32)> = sort_by_freq(freq);
        print!("{}", sorted_freq[0].0);
    }
}
