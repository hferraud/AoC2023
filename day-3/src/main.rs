use std::fs;

const INPUT_PATH: &str = "resource/input";

fn main() {
    let mut blueprint: Vec<Vec<char>> = vec![];
    let valid_numbers: Vec<Vec<u32>>;
    let mut part: u32;
    let key: u32;

    for line in fs::read_to_string(INPUT_PATH).unwrap().lines() {
        blueprint.push(line.chars().collect());
    }
    valid_numbers = scan_blueprint(&blueprint);
    loop {
        part = user_choose_part();
        match part {
            1 => {
                key = build_part_one_key(&valid_numbers);
                break;
            }
            2 => {
                key = build_part_two_key(&valid_numbers);
                break;
            }
            _ => {
                println!("Invalid input");
            }
        }
    }
    println!("The secret key is: {key}");
}

fn user_choose_part() -> u32 {
    loop {
        println!("Please enter which key do you need?");
        println!("1: Part one");
        println!("2: Part two");
        let mut part = String::new();
        std::io::stdin()
            .read_line(&mut part)
            .expect("Failed to read line");

        let part: u32 = match part.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue
            },
        };
        return part;
    }
}

fn build_part_one_key(valid_numbers: &Vec<Vec<u32>>) -> u32 {
    let mut key: u32 = 0;

    for numbers in valid_numbers {
        for nb in numbers {
            key += nb;
        }
    }
    return key;
}

fn build_part_two_key(valid_numbers: &Vec<Vec<u32>>) -> u32 {
    let mut key: u32 = 0;

    for numbers in valid_numbers {
        if numbers.len() == 2 {
            key += numbers[0] * numbers[1];
        }
    }
    return key;
}

fn scan_blueprint(blueprint: &Vec<Vec<char>>) -> Vec<Vec<u32>> {
    let mut valid_numbers: Vec<Vec<u32>> = vec![];

    let mut y: usize = 0;
    while y < blueprint.len() {
        let mut x: usize = 0;
        while x < blueprint[y].len() {
            if is_valid_symbol(blueprint[y][x]) {
                valid_numbers.push(scan_symbol(blueprint, x, y))
            }
            x += 1;
        }
        y += 1;
    }
    return valid_numbers;
}

fn is_valid_symbol(c: char) -> bool {
    return c != '.' && !c.is_digit(10)
}

fn scan_symbol(blueprint: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<u32> {
    let mut nb_vec: Vec<u32> = vec![];

    let mut scan_y = y - 1;
    while scan_y <= y + 1 {
        let mut scan_x = x - 1;
        while scan_x <= x + 1 {
            if blueprint[scan_y][scan_x].is_digit(10) {
                nb_vec.push(extract_number(blueprint, scan_x, scan_y));
                while scan_x < blueprint[y].len() && blueprint[scan_y][scan_x].is_digit(10) {
                    scan_x += 1;
                }
            }
            scan_x += 1;
        }
        scan_y += 1;
    }
    return nb_vec;
}

fn extract_number(blueprint: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut nb: u32 = 0;
    let mut scan_x = x;
    while scan_x > 0 && blueprint[y][scan_x - 1].is_digit(10) {
        scan_x -= 1;
    }
    while scan_x < blueprint[y].len() && blueprint[y][scan_x].is_digit(10) {
        nb = nb * 10 + blueprint[y][scan_x].to_digit(10).unwrap();
        scan_x += 1;
    }
    return nb;
}
