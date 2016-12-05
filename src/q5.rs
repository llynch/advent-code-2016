extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn m1(input: &str) -> String {
    let mut count = 0;
    let mut md5 = Md5::new();
    let mut digits : Vec<char> = Vec::new();
    loop {
        let current = &(format!("{}{}", input, count))[..];
        md5.input_str(current);
        let hash = md5.result_str();
        count += 1;

        if hash.chars().take(5).into_iter().collect::<String>() == "00000".to_string() {
            let digit = hash.chars().nth(5).unwrap();
            digits.push(digit);
        }

        if digits.len() == 8 {
            break
        }
        md5.reset();
    }

    return digits.into_iter().collect::<String>();
}

fn m2(input: &str) -> String {
    let mut count = 0;
    let mut md5 = Md5::new();
    let mut digits : [char; 8] = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    let mut nbdigits = 0;
    loop {
        let current = &(format!("{}{}", input, count))[..];
        md5.input_str(current);
        let hash = md5.result_str();
        count += 1;

        if hash.chars().take(5).into_iter().collect::<String>() == "00000".to_string() {
            let position = hash.chars().nth(5).unwrap();

            if "01234567".to_string().chars().collect::<Vec<char>>().contains(&position) {
                let digit = hash.chars().nth(6).unwrap();

                let pos = position.to_string().parse::<i32>().unwrap();
                if digits[pos as usize] == ' ' {
                    nbdigits += 1;
                    digits[pos as usize] = digit;
                }
            }
        }

        if nbdigits == 8 {
            break
        }
        md5.reset();
    }

    return digits.iter().cloned().collect::<String>();
}

fn main() {
    // input was = cxdnnyjw
    let input = &std::env::args().nth(1).unwrap();

    println!("m1: {}", m1(input));
    println!("m2: {}", m2(input));
}

