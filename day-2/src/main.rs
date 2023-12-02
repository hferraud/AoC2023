use std::cmp::max;
use std::collections::HashMap;
use std::fs;

struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let games = parse_input_file("resource/input");
    let cube_set = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    loop {
        println!("Which key do you need?");
        println!("1: part one");
        println!("2: part two");
        let mut part = String::new();
        std::io::stdin()
            .read_line(&mut part)
            .expect("Failed to read line");

        let part: u32 = match part.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match part {
            1 => {
                let key = get_part_one_key(cube_set, games);
                println!("Key: {}", key);
                break;
            }
            2 => {
                let key = get_part_two_key(games);
                println!("Key: {}", key);
                break;
            }
            _ => {
                continue;
            }
        }
    }
}

fn get_part_one_key(initial_set: CubeSet, games: HashMap<u32, Vec<CubeSet>>) -> u32 {
    let mut key: u32 = 0;

    for game in games {
        let game_id = game.0;
        let max_set = get_max_set(game.1);
        if is_valid_set(&initial_set, max_set) {
            key += game_id;
        }
    }
    return key;
}

fn get_part_two_key(games: HashMap<u32, Vec<CubeSet>>) -> u32 {
    let mut key: u32 = 0;

    for game in games {
        let max_set = get_max_set(game.1);
        key += max_set.red * max_set.green * max_set.blue;
    }
    return key;
}

fn get_max_set(sets: Vec<CubeSet>) -> CubeSet {
    let mut max_set = CubeSet {
        red: 0,
        green: 0,
        blue: 0
    };

    for set in sets {
        max_set.red = max(max_set.red, set.red);
        max_set.blue = max(max_set.blue, set.blue);
        max_set.green = max(max_set.green, set.green);
    }
    return max_set;
}

fn is_valid_set(src: &CubeSet, other: CubeSet) -> bool {
    if other.red > src.red {
        return false;
    }
    if other.green > src.green {
        return false;
    }
    if other.blue > src.blue {
        return false;
    }
    return true;
}

fn parse_input_file(path: &str) -> HashMap<u32, Vec<CubeSet>> {
    let mut map: HashMap<u32, Vec<CubeSet>> = Default::default();

    for line in fs::read_to_string(path).unwrap().lines() {
        map.insert(parse_game_id(line), parse_cube_sets(line));
    }
    return map;
}

fn parse_game_id(line: &str) -> u32 {
    let id: u32;
    let id_str: &str;
    let size = line.find(':').unwrap();
    let slice: &str = &line[0usize..size];

    id_str = &slice["Game ".len()..];
    id = id_str.parse().expect("parse_game_id() : parse()");
    return id;
}

fn parse_cube_sets(line: &str) -> Vec<CubeSet> {
    let mut cube_sets: Vec<CubeSet> = vec![];
    let sets: &str;


    sets = &line[line.find(':').unwrap() + 2..];
    for set in sets.split("; ") {
        for cube in set.split(", ") {
            cube_sets.push(parse_cube_set(cube));
        }
    }
    return cube_sets;
}

fn parse_cube_set(line: &str) -> CubeSet {
    let mut cube_set = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };
    let cube_nb: u32;
    let cube_color: &str;

    let cube_nb_str: &str = &line[..line.find(' ').unwrap()];
    cube_nb = cube_nb_str.parse()
        .expect("parse_cube_set() : parse()");
    cube_color = &line[line.find(' ').unwrap() + 1..];
    match cube_color {
        "red" => {
            cube_set.red = cube_nb;
        }
        "green" => {
            cube_set.green = cube_nb;
        }
        "blue" => {
            cube_set.blue = cube_nb;
        }
        &_ => {
            panic!("Cube color did not match any.")
        }
    }
    return cube_set;
}
