use std;
use std::io::prelude::*;
use std::fs::File;

pub fn read(filename: &String) -> String {
    let mut file = File::open(filename).unwrap();
    let mut buff: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buff);
    let content = std::str::from_utf8(&buff).unwrap();
    return content.to_string();
}

/*
Notes:

    Vec<char> -> String
        v.into_iter().collect()

    String -> Vec<char>
        s.chars().collect()

    String -> &str
        &s[..]

    &str -> String
        s.to_string()

    &str -> i32
        s.parse::<i32>().unwrap()

    string.indexOf(c):
        s.chars().position(|r| r == c).unwrap();

    string.charAt(n):
        let vc: Vec<char> = s.chars().collect()
        vc[n as usize]

 */
