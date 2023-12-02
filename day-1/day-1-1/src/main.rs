use std::fs::read_to_string;

fn main() {
    let input_path: String = String::from("resource/input");
    let mut secret_key: u32 = 0;
    let mut line_key: u32;

    for line in read_to_string(input_path).unwrap().lines() {
        line_key = parse_line(line);
        secret_key += line_key;
    }
    println!("Secret key: {secret_key}");
}

fn parse_line(line: &str) -> u32 {
    let mut key: u32 = 0;
    let mut last_char: char = '0';
    let mut first_char_found: bool = false;

    for c in line.chars() {
        if c.is_ascii_digit() {
            if !first_char_found {
                key = c.to_digit(10)
                    .expect("Couldn't convert char into digit");
                key *= 10;
                first_char_found = true;
            }
            last_char = c;
        }
    }
    key += last_char.to_digit(10)
        .expect("Couldn't convert char into digit");

    return key;
}
