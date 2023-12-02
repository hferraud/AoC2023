use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let dict = HashMap::from([
        ("one", 1u32),
        ("two", 2u32),
        ("three", 3u32),
        ("four", 4u32),
        ("five", 5u32),
        ("six", 6u32),
        ("seven", 7u32),
        ("eight", 8u32),
        ("nine", 9u32),
    ]);
    let mut line_key: u32;
    let mut secret_key: u32 = 0;

    for line in read_to_string("resource/input.txt").unwrap().lines() {
        line_key = get_key_from_line(line, dict.clone());
        secret_key += line_key;
        println!("Line: {line}");
        println!("Key: {line_key}");
    }
    println!("The secret key is {secret_key}");
}

fn get_key_from_line(line: &str, dict: HashMap<&str, u32>) -> u32 {
    let mut key: u32 = 0;
    let mut last_key: u32 = 0;
    let mut current_key: u32;
    let mut current_line: String;

    for i in 0..line.len() {
        if line.chars().nth(i).unwrap().is_ascii_digit() {
            current_key = line.chars().nth(i).unwrap().to_digit(10)
                .expect("Couldn't conver char to digit");
            last_key = current_key;
            if key == 0 {
                key = current_key * 10;
            }
        } else {
            current_line = line.chars().skip(i).collect();
            current_key = get_key_from_dict(current_line.as_str(), dict.clone());
            if (current_key != 0) {
                last_key = current_key;
                if key == 0 {
                    key = current_key * 10;
                }
            }
        }
    }
    key += last_key;
    return key;
}

fn get_key_from_dict(line: &str, dict: HashMap<&str, u32>) -> u32 {
    for item in dict {
        if line.starts_with(item.0) {
            return item.1;
        }
    }
    return 0;
}