/*
to compile and run from root folder.
:! clear && rm -rf build/q2 && mkdir -p build && rustc src/q2.rs -o build/q2 && build/./q2 data/q2
 */
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn read(filename: &String) -> String {
    let mut file = File::open(filename).unwrap();
    let mut buff: Vec<u8> = Vec::new();
    #[allow(unused_must_use)]
    file.read_to_end(&mut buff);
    let content = std::str::from_utf8(&buff).unwrap();
    return content.to_string();
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    return (a.0 + b.0, a.1 + b.1);
}

fn mul(t: (i32, i32), n: i32) -> (i32, i32) {
    return (t.0 * n, t.1 * n);
}

fn main() {
    // https://doc.rust-lang.org/std/env/fn.args.html
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let content = read(filename);


    let mut positions: Vec<(i32, i32)> = Vec::new();
    for line in content.lines() {
        let mut position = (1, 1);
        for c in line.chars() {
            let movement = match c {
                'U' => ( 0, -1),
                'D' => ( 0, 1),
                'R' => ( 1,  0),
                'L' => (-1,  0),
                _ => panic!("show never get here :/")
            };
            let new_position = add(position, movement);
            if (new_position.0 < 0) || (new_position.0 > 2) {
                continue;
            }
            if (new_position.1 < 0) || (new_position.1 > 2) {
                continue;
            }
            position = new_position;
        }
        positions.push(position);
    }

    let pad = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]];

    for p in positions {
        print!("{}", pad[p.1 as usize][p.0 as usize]);
    }
}
