const INPUT_PATH: &str = "resource/input";

enum KeyRequest {
    PartOne,
    PartTwo,
}

#[derive(Clone, Copy)]
struct Race {
    time: u64,
    distance: u64,
}

fn main() {
    let races: Vec<Race> = parse_input(INPUT_PATH);
    let key: u64;
    let request: KeyRequest;

    request = get_user_input();
    let start_time = std::time::Instant::now();
    match request {
        KeyRequest::PartOne => key = get_part_one_key(races),
        KeyRequest::PartTwo => key = get_part_two_key(races),
    }
    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("The secret key is: {key}");
    println!("Key retrieved in {} ms", duration.as_millis());
}

fn get_part_one_key(races: Vec<Race>) -> u64 {
    let mut key: u64 = 0;

    let winning_ways = get_winning_ways(races);
    for way in winning_ways {
        if key == 0 {
            key = way;
        } else {
            key *= way;
        }
    }
    return key;
}

fn get_part_two_key(races: Vec<Race>) -> u64 {
    let key: u64;
    let race: Race;

    race = get_single_race(races);
    key = calculate_winning_ways(race);
    return key;
}

fn get_single_race(races: Vec<Race>) -> Race {
    let mut single_race: Race = Race {
        time: 0,
        distance: 0
    };
    let mut time_str = String::from("");
    let mut distance_str = String::from("");

    for race in races {
        time_str = time_str + &*race.time.to_string();
        distance_str = distance_str + &*race.distance.to_string();
    }
    single_race.time = time_str.parse().unwrap();
    single_race.distance = distance_str.parse().unwrap();
    return single_race;
}

fn get_winning_ways(races: Vec<Race>) -> Vec<u64> {
    let mut winning_ways: Vec<u64> = vec![];
    for race in races {
        winning_ways.push(calculate_winning_ways(race));
    }
    return winning_ways;
}

fn calculate_winning_ways(race: Race) -> u64 {
    let mut winning_way: u64 = 0;
    let mut distance: u64;
    let mut flag: bool = false;

    for i in 1..race.time {
        distance = i * (race.time - i);
        if distance > race.distance {
            winning_way += 1;
            flag = true;
        } else if flag {
            return winning_way;
        }
    }
    return winning_way;
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


fn parse_input(input_path: &str) -> Vec<Race> {
    let mut races: Vec<Race> = vec![];
    let mut race = Race{
        time: 0,
        distance: 0,
    };

    let binding = std::fs::read_to_string(input_path).unwrap();
    let mut lines = binding.lines();
    let time_line = lines.next().unwrap();
    let time_nbs = &mut time_line[time_line.find(':').unwrap() + 1..].split_whitespace();
    let distance_line = lines.next().unwrap();
    let distance_nbs = &mut distance_line[distance_line.find(':').unwrap() + 1..].split_whitespace();
    for nb in time_nbs {
        race.time = nb.parse().unwrap();
        race.distance = distance_nbs.next().unwrap().parse().unwrap();
        races.push(race);
    }
    return races;
}