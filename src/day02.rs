use std::fs::File;
use std::vec::Vec;
use std::io::{self, prelude::*, BufReader};

const RED: u32 = 12;
const GREEN: u32 = 13; 
const BLUE: u32 = 14;

// a game is made up of rounds
// for a game to be possible all rounds must also to be possible
// ie n_red <= RED_MAX \ n_green <= GREEN_MAX \ n_blue <= BLUE_MAX


fn parse_game(line: String) -> io::Result<u32> {

	let parts: Vec<&str> = line.split(": ").collect();

	let game_id = parts
					.first().expect("")
					.split(" ")
					.last().expect("")
					.parse::<u32>().expect("");
	
	let unparsed_rounds: 
		Vec<&str> = parts
						.last().expect("")
						.split("; ")
						.collect();

	for round_string in unparsed_rounds {
		//println!("{}", round_string);

		if !parse_round(round_string) {
			// invalid round, return 0
			return Ok(0);
		}
	}

	Ok(game_id)
}

fn parse_round(round_string: &str) -> bool {

	let pairs: Vec<&str> = round_string
							.split(", ")
							.collect();

	for pair in pairs {
		let mut tmp = pair.split(" ");

		let amount = (tmp.nth(0).unwrap()).parse::<u32>().expect("");
		let color = tmp.nth(0).unwrap();

		//println!("{} -> {}", color, amount);
		//println!("{}", pair);

		match color {
			"red" => if amount > RED { return false; },
			"green" => if amount > GREEN { return false; },
			"blue" => if amount > BLUE { return false; },
			_ => return false,
		}
	}
	
	return true;
}

fn game_sum_cubes(line: String) -> u32 {
	let parts: Vec<&str> = line.split(": ").collect();

	let unparsed_rounds: 
		Vec<&str> = parts
					.last().expect("")
					.split("; ")
					.collect();

	let mut max_red: u32 = 1;
	let mut max_green: u32 = 1;
	let mut max_blue: u32 = 1;
			
	for round_string in unparsed_rounds {
		//println!("{}", round_string);

		(max_red, max_green, max_blue) = round_cube(round_string, max_red, max_green, max_blue);
	}

	max_red * max_green * max_blue
}

fn round_cube(round_string: &str, max_red: u32, max_green: u32, max_blue: u32) -> (u32, u32, u32) {
	let mut r = max_red;
	let mut g = max_green;
	let mut b = max_blue;

	let pairs: Vec<&str> = round_string
							.split(", ")
							.collect();

	for pair in pairs {
		let mut tmp = pair.split(" ");

		let amount = (tmp.nth(0).unwrap()).parse::<u32>().expect("");
		let color = tmp.nth(0).unwrap();

		//println!("{} -> {}", color, amount);
		//println!("{}", pair);

		match color {
			"red" => if amount > r { r = amount; },
			"green" => if amount > g { g = amount; },
			"blue" => if amount > b { b = amount; },
			_ => {},
		}
	}

	(r, g, b)
}

// in part one, we had to figure out what games were possible
// given the max number of R/G/B cubes
pub fn part1() -> io::Result<u32> {
	let path = "src/inputs/input02.txt";

	let file = File::open(path)?;
    let reader = BufReader::new(file);

	let mut id_sum: u32 = 0;
    for line in reader.lines() {		
		id_sum += (parse_game(line.unwrap())).unwrap();
    }
	
	Ok(id_sum)
}

// in part two, we had to find the maximum number
// of cubes of each color that appear in rounds
pub fn part2() -> io::Result<u32> {
	let path = "src/inputs/input02.txt";

	let file = File::open(path)?;
	let reader = BufReader::new(file);

	let mut soc: u32 = 0;

	for line in reader.lines() {
		soc += game_sum_cubes(line.unwrap());
	}

	Ok(soc)
}