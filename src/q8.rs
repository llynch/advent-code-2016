#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::io::BufRead;
use std::str;
use std::io;
use std::collections::HashMap;
use std::cmp::Ordering;

mod utils;

static W: i32 = 50;
static H: i32 = 6;


lazy_static! {
    static ref re_rect : Regex = Regex::new(
        r"^rect (?P<w>\d+)x(?P<h>\d+)$").unwrap();

    static ref re_row : Regex = Regex::new(
        r"^rotate row y=(?P<y>\d+) by (?P<n>\d+)$").unwrap();

    static ref re_column : Regex = Regex::new(
        r"^rotate column x=(?P<x>\d+) by (?P<n>\d+)$").unwrap();
}

fn print(screen: &Vec<Vec<char>>) {
    for line in screen {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
}

fn rect(screen: &mut Vec<Vec<char>>, w: i32, h: i32) {
    for x in 0..w {
        for y in 0..h {
            screen[y as usize][x as usize] = '#';
        }
    }
}

fn rotate_row(screen: &mut Vec<Vec<char>>, y: i32, n: i32) {
    let mut new_row : Vec<char> = vec!['x'; W as usize];
    for x in 0..W {
        new_row[x as usize] = screen[y as usize][((x-n+W) % W) as usize];
    }
    for x in 0..W {
        screen[y as usize][x as usize] = new_row[x as usize];
    }

}

fn rotate_column(screen: &mut Vec<Vec<char>>, x: i32, n: i32) {
    let mut new_col: Vec<char> = vec!['x'; H as usize];
    for y in 0..H {
        new_col[y as usize] = screen[((y-n+H) % H) as usize][x as usize];
    }
    for y in 0..H {
        screen[y as usize][x as usize] = new_col[y as usize]
    }
}

fn count(screen: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for line in screen {
        for c in line {
            if *c == '#' {
                count += 1; }}}
    return count;
}


fn main() {
    let line : Vec<char> = vec!['.'; W as usize];
    let mut screen: Vec<Vec<char>> = vec![line; H as usize];
    for x in 0..W {
        for y in 0..H {
            screen[y as usize][x as usize] = '.';
        }
    }

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = &l.unwrap();
        if line.trim() == "" {
            continue;
        }

        if re_rect.is_match(line) {
            let caps = re_rect.captures(line).unwrap();
            let w = caps.name("w").unwrap().parse::<i32>().unwrap();
            let h = caps.name("h").unwrap().parse::<i32>().unwrap();
            rect(&mut screen, w, h);
        }
        if re_row.is_match(line) {
            let caps = re_row.captures(line).unwrap();
            let y = caps.name("y").unwrap().parse::<i32>().unwrap();
            let n = caps.name("n").unwrap().parse::<i32>().unwrap();
            rotate_row(&mut screen, y, n);
        }
        if re_column.is_match(line) {
            let caps = re_column.captures(line).unwrap();
            let y = caps.name("x").unwrap().parse::<i32>().unwrap();
            let n = caps.name("n").unwrap().parse::<i32>().unwrap();
            rotate_column(&mut screen, y, n);
        }
    }
    print(&screen);
    let c = count(&screen);
    println!("{}", c);
    // ZJHRKCPLYJ
}
