const INPUT_PATH: &str = "resource/input";

enum KeyRequest {
    PartOne,
    PartTwo,
}

fn main() {
	let key: u64;

	let request = get_user_input();
	match request {
		KeyRequest::PartOne => key = get_part_one_key(),
		KeyRequest::PartTwo => key = get_part_two_key(),
	}
	println!("The secret key is: {key}"); 
}

fn get_part_one_key() -> u64 {
	let mut hands: Vec<(String, u32, u32)>;
	hands = parse_input(INPUT_PATH, KeyRequest::PartOne);
	hands.sort_by(hand_cmp1);
	return get_key(hands);
}

fn get_part_two_key() -> u64 {
	let mut hands: Vec<(String, u32, u32)>;
	hands = parse_input(INPUT_PATH, KeyRequest::PartTwo);
	hands.sort_by(hand_cmp2);
	for hand in &hands {
		println!("{}", hand.0);
	}
	return get_key(hands);
}

fn get_key(hands: Vec<(String, u32, u32)>) -> u64 {
	let mut key: u64 = 0;
	let mut rank: u64 = 0;

	for hand in hands {
		rank += 1;
		key += hand.1 as u64 * rank;
	}
	return key;
}

fn hand_cmp1(a: &(String, u32, u32), b: &(String, u32, u32)) -> std::cmp::Ordering {
	if a.2 != b.2 {
		return a.2.cmp(&b.2);
	}
	let chars_a = a.0.chars();
	let mut chars_b = b.0.chars();
	let mut c_b;
	for c_a in chars_a {
		c_b = chars_b.next().unwrap();
		if c_a != c_b {
			return get_part_one_strength(c_a).cmp(&get_part_one_strength(c_b));	
		}
	}
	return std::cmp::Ordering::Equal;
}

fn hand_cmp2(a: &(String, u32, u32), b: &(String, u32, u32)) -> std::cmp::Ordering {
	if a.2 != b.2 {
		return a.2.cmp(&b.2);
	}
	let chars_a = a.0.chars();
	let mut chars_b = b.0.chars();
	let mut c_b;
	for c_a in chars_a {
		c_b = chars_b.next().unwrap();
		if c_a != c_b {
			return get_part_two_strength(c_a).cmp(&get_part_two_strength(c_b));	
		}
	}
	return std::cmp::Ordering::Equal;
}

fn get_part_one_strength(c: char) -> u32 {
	return match c {
		'2' => 1,
		'3' => 2,
		'4' => 3,
		'5' => 4,
		'6' => 5,
		'7' => 6,
		'8' => 7,
		'9' => 8,
		'T' => 9,
		'J' => 10,
		'Q' => 11,
		'K' => 12,
		'A' => 13,
		_ => 0,
	}
}

fn get_part_two_strength(c: char) -> u32 {
	return match c {
		'J' => 1,
		'2' => 2,
		'3' => 3,
		'4' => 4,
		'5' => 5,
		'6' => 6,
		'7' => 7,
		'8' => 8,
		'9' => 9,
		'T' => 10,
		'Q' => 11,
		'K' => 12,
		'A' => 13,
		_ => 0,
	}
}

fn get_hand_type(hand: String, request: &KeyRequest) -> u32 {
	let combo: Vec<u32>;

	combo = match request {
		KeyRequest::PartOne => get_part_one_combo(hand.clone()),
		KeyRequest::PartTwo => get_part_two_combo(hand.clone()),
	};
	if combo.contains(&5u32) {
		return 7;
	}
	if combo.contains(&4u32) {
		return 6;
	}
	if combo.contains(&3u32) && combo.contains(&2u32) {
		return 5;
	}
	if combo.contains(&3u32) {
		return 4;
	}
	if combo.iter().filter(|&&x| x == 2u32).count() == 2 {
		return 3;
	}
	if combo.contains(&2u32) {
		return 2;
	}
	return 1;
}

fn get_part_one_combo(hand: String) -> Vec<u32> {
	let mut combo: Vec<u32> = vec![];
	let mut found: Vec<char> = vec![];
	let mut count: u32;

	for c in hand.chars() {
		if found.contains(&c) {
			continue;
		}
		count = hand.chars().filter(|&cc| cc == c).count() as u32;
		combo.push(count);
		found.push(c);
	}
	return combo;
}

fn get_part_two_combo(hand: String) -> Vec<u32> {
	let mut combo: Vec<u32> = vec![];
	let mut found: Vec<char> = vec![];
	let mut count: u32;

	for c in hand.chars() {
		if found.contains(&c) {
			continue;
		}
		found.push(c);
		if c == 'J' {
			continue;
		}
		count = hand.chars().filter(|&cc| cc == c).count() as u32;
		combo.push(count);
	}
	if found.contains(&'J') {
		count = hand.chars().filter(|&cc| cc == 'J').count() as u32;
		if let Some((index, _)) = combo.iter().enumerate().max_by_key(|&(_, &value)| value) {
			combo[index] += count;
		} else {
			combo.push(5);
		}
	}
	return combo;
}

fn parse_input(input_path: &str, request: KeyRequest) -> Vec<(String, u32, u32)> {
	let mut hands: Vec<(String, u32, u32)> = vec![];
	let mut hand: String;
	let mut bid: u32;
	let mut hand_type: u32;

	for line in std::fs::read_to_string(input_path).unwrap().lines() {
		hand = (&line[..line.find(' ').unwrap()]).to_string();
		bid = (&line[line.find(' ').unwrap() + 1..]).parse().unwrap();
		hand_type = get_hand_type(hand.clone(), &request);
		hands.push((hand, bid, hand_type));
	}
	return hands;
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

