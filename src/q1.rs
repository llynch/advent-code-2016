/*
to compile and run from root folder.
:! clear && rm -rf build/q1 && mkdir -p build && rustc src/q1.rs -o build/q1 && build/./q1 data/q1
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

    let directions = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0)
    ];
    let mut direction_index = 4000; // 4000 to prevent overflow with negative direction_index (see direction_index - 1)
    let mut position = (0, 0);

    let mut found_visited_twice = false;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let items = content.lines().next().unwrap().split(", ");
    for item in items {
        let letter = item.chars().nth(0).unwrap();
        let trim_chars : &[char] = &['L', 'R'];
        let n = item.to_string().trim_matches(trim_chars).parse::<i32>().unwrap();

        // turn
        if letter == 'L' {
            direction_index = direction_index - 1;
        }
        // turn right
        if letter == 'R' {
            direction_index = direction_index + 1;
        }

        let current_direction = directions[direction_index % directions.len()];
        /*
        // was usefull for solution a
        let movement = mul(current_direction, n);
        position = add(position, movement);
        */

        for i in 0..n {
            position = add(position, current_direction);

            if visited.contains(&position) && found_visited_twice == false {
                found_visited_twice = true;
                println!("visited twice distance: {}", position.0.abs() + position.1.abs());
            }
            visited.insert(position);
        }
    }
    println!("final distance: {}", position.0 + position.1);
}
