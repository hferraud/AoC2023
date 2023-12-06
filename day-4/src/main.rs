const INPUT_PATH: &str = "resource/input";

enum KeyRequest {
    PartOne,
    PartTwo,
}

fn main() {
    let mut cards: Vec<(Vec<u32>, Vec<u32>)> = vec![];
    let points: Vec<u32>;
    let request: KeyRequest;
    let key: u32;

    for line in std::fs::read_to_string(INPUT_PATH).unwrap().lines() {
        cards.push(line_to_card(line));
    }
    points = get_card_points(&cards);
    request = get_user_input();
    match request {
        KeyRequest::PartOne => key = get_part_one_key(points),
        KeyRequest::PartTwo => key = get_part_two_key(points),
    }
    println!("Secret key is: {key}");
}

fn get_part_one_key(points: Vec<u32>) -> u32 {
    let mut key: u32 = 0;
    for point in points {
        if point > 0 {
            key += 2u32.pow(point - 1);
        }
    }
    return key;
}

fn get_part_two_key(points: Vec<u32>) -> u32 {
    let mut key: u32 = 0;

    for i in 0..points.len() {
        key += compute_card_point(&points, i);
    }
    return key + points.len() as u32;
}

fn compute_card_point(points: &Vec<u32>, index: usize) -> u32 {
    let mut point: u32 = points[index];

    for i in index + 1..=index + points[index] as usize {
        point += compute_card_point(points, i);
    }
    return point;
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

fn line_to_card(line: &str) -> (Vec<u32>, Vec<u32>) {
    let mut winning_numbers: Vec<u32> = vec![];
    let mut input_numbers: Vec<u32> = vec![];
    let mut part: Vec<&str> = line.split('|').collect();
    part[0] = &part[0][part[0].find(':').unwrap() + 1..];
    for nb in part[0].split_whitespace() {
        winning_numbers.push(nb.parse().expect("line_to_card(): parse()"));
    }
    for nb in part[1].split_whitespace() {
        input_numbers.push(nb.parse().expect("line_to_card(): parse()"));
    }
    return (winning_numbers, input_numbers);
}

fn get_card_points(cards: &Vec<(Vec<u32>, Vec<u32>)>) -> Vec<u32> {
    let mut points: Vec<u32> = vec![];
    let mut point: u32;

    for card in cards {
        let (winning_nb, input_nb) = card;
        point = 0;
        for nb in input_nb {
            if winning_nb.contains(nb) {
                point += 1;
            }
        }
        points.push(point);
    }
    return points;
}
