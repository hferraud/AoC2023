use std::cmp::min;

const INPUT_PATH: &str = "resource/input";

#[derive(Clone, Copy)]
struct Converter {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

enum KeyRequest {
    PartOne,
    PartTwo,
}

fn main() {
    let (seeds, maps) = parse_input(INPUT_PATH);
    let request: KeyRequest;
    let key: u64;

    request = get_user_input();
    match request {
        KeyRequest::PartOne => key = get_part_one_key(seeds, &maps),
        KeyRequest::PartTwo => key = get_part_two_key(seeds, &maps),
    }
    println!("The secret key is: {key}");
}

fn get_part_one_key(seeds: Vec<u64>, maps: &Vec<Vec<Converter>>) -> u64 {
    let key: u64;

    key = get_min_location(seeds, maps);
    return key;
}

fn get_part_two_key(seeds: Vec<u64>, maps: &Vec<Vec<Converter>>) -> u64 {
    let mut key: u64 = u64::MAX;
    let seed_ranges: Vec<(u64, u64)>;

    seed_ranges = get_seed_ranges(seeds);
    for i in 0..seed_ranges.len() {
        println!("Seed start:  {}", seed_ranges[i].0);
        println!("Seed length: {}", seed_ranges[i].1);
        for seed in seed_ranges[i].0..seed_ranges[i].0 + seed_ranges[i].1 {
            if is_seed_already_tested(seed, &seed_ranges, i) {
                continue;
            }
            key = min(key, get_seed_location(seed, maps));
        }
    }
    return key;
}

fn is_seed_already_tested(seed: u64, seed_ranges: &Vec<(u64, u64)>, index: usize) -> bool {
    for i in 0..index {
        if (seed_ranges[i].0..seed_ranges[i].0 + seed_ranges[i].1).contains(&seed) {
            return true;
        }
    }
    return false;
}

fn get_seed_ranges(seeds: Vec<u64>) -> Vec<(u64, u64)> {
    let mut seed_ranges: Vec<(u64, u64)> = vec![];

    let mut i = 0;
    while i < seeds.len() {
        seed_ranges.push((seeds[i], seeds[i + 1]));
        i += 2;
    }
    return seed_ranges;
}

fn get_min_location(seeds: Vec<u64>, maps: &Vec<Vec<Converter>>) -> u64 {
    let mut locations: Vec<u64> = vec![];
    for seed in seeds {
        locations.push(get_seed_location(seed, maps));
    }
    let min_location = locations.iter().min().unwrap();
    return *min_location;
}

fn get_seed_location(seed: u64, maps: &Vec<Vec<Converter>>) -> u64 {
    let mut dest: u64 = seed;

    for map in maps {
        for converter in map {
            if is_seed_in_range(dest, converter) {
                dest = converter.destination_start + dest - converter.source_start;
                break;
            }
        }
    }
    return dest;
}

fn is_seed_in_range(seed: u64, converter: &Converter) -> bool {
    return seed >= converter.source_start && seed < (converter.source_start + converter.length);
}

fn get_user_input() -> KeyRequest {
    loop {
        println!("Please enter which key you need: ");
        println!("1 => Part one key");
        println!("2 => Part two key");
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read stdin");
        let input = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
        match input {
            1 => return KeyRequest::PartOne,
            2 => return KeyRequest::PartTwo,
            _ => println!("Invalid input"),
        }
    }
}

fn parse_input(input_path: &str) -> (Vec<u64>, Vec<Vec<Converter>>) {
    let seeds: Vec<u64>;
    let mut maps: Vec<Vec<Converter>> = vec![];
    let mut map: Vec<Converter> = vec![];
    let mut converter: Converter = Converter {
        source_start: 0,
        destination_start: 0,
        length: 0,
    };

    let binding = std::fs::read_to_string(input_path).unwrap();
    let mut lines = binding.lines();
    seeds = parse_seeds(lines.nth(0).unwrap());
    lines.next();
    for line in lines {
        if line != "" && line.chars().nth(0).unwrap().is_digit(10) {
            let mut nbs = line.split(' ');
            converter.destination_start = nbs.next()
                .unwrap().parse()
                .expect("parse_input(): parse()");
            converter.source_start = nbs.next()
                .unwrap().parse()
                .expect("parse_input(): parse()");
            converter.length = nbs.next()
                .unwrap().parse()
                .expect("parse_input(): parse()");
            map.push(converter);
        } else if map.len() > 0{
            maps.push(map.clone());
            map.clear()
        }
    }
    maps.push(map.clone());
    return (seeds, maps)
}

fn parse_seeds(line: &str) -> Vec<u64> {
    let mut seeds: Vec<u64> = vec![];

    let line = &line[line.find(':').unwrap() + 1..];
    for seed in line.split_whitespace() {
        seeds.push(seed.parse().unwrap());
    }
    return seeds;
}
