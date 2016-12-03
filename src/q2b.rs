/*
to compile and run from root folder.
:! clear && rm -rf build/q2b && mkdir -p build && rustc src/q2b.rs -o build/q2b && build/./q2b data/q2b
 */
use std::io::prelude::*;
use std::fs::File;

fn read(filename: &String) -> String {
    let mut file = File::open(filename).unwrap();
    let mut buff: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buff);
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

    fn pad(p: (i32, i32)) -> char {
        let data = [
                   "  1  ",
                   " 234 ",
                   "56789",
                   " ABC ",
                   "  D  "];

        return data[p.1 as usize].chars().nth(p.0 as usize).unwrap();
    };

    assert_eq!('5', pad((0, 2)));

    let mut values: Vec<char> = Vec::new();
    for line in content.lines() {
        let mut position = (0, 2);
        let mut value = pad(position);
        for c in line.chars() {
            let movement = match c {
                'U' => ( 0, -1),
                'D' => ( 0,  1),
                'R' => ( 1,  0),
                'L' => (-1,  0),
                _ => panic!("should never get here...")
            };
            let new_position = add(position, movement);
            if (new_position.0 < 0) || (new_position.0 > 4) {
                continue;
            }
            if (new_position.1 < 0) || (new_position.1 > 4) {
                continue;
            }

            let new_value = pad(new_position);
            if new_value == ' ' {
                continue;
            }

            position = new_position;
            value = new_value;
        }
        values.push(value);
    }

    for v in values {
        print!("{}", v)
    }
}
