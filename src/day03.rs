use std::cmp;
use std::fs::File;
use std::vec::Vec;
use std::io::{self, prelude::*, BufReader};

const INPUT: &str = "src/inputs/input03.txt";

fn build_matrix() -> io::Result<Vec<Vec<char>>> {
	let file = File::open(INPUT)?;
	let reader = BufReader::new(file);
	
	let mut matrix: Vec<Vec<char>> = Vec::new();
	
	for line in reader.lines() {
		let mut row: Vec<char> = Vec::new();
		
		for c in line.unwrap().chars() {
			row.push(c);
		}
		matrix.push(row);
	}
	
	Ok(matrix)
}


fn part_number_sum(matrix: Vec<Vec<char>>) -> u32 {

	let mut part_numbers: Vec<String> = Vec::new();

	let width: usize = matrix[0].len();
	let height: usize = matrix.len();
	
	let mut curr_number = String::new();
	for i in 0..height {
		let mut start_j: usize = 0;
		let mut end_j: usize = 0;

		for j in 0..width {
			let c = matrix[i][j];
			
			if curr_number.len() == 0 {
				start_j = j;
			}
			
			if c.is_digit(10) {
				curr_number.push(c);
				
			} else {
				if curr_number.len() > 0 {
					if validate_adjacency(matrix.clone(), i as u32, start_j, j) {
						part_numbers.push(curr_number.clone());
					}
					curr_number.clear();
				}
			}
			end_j = j;
		}
	}

	let sum: u32 = part_numbers
				   .iter()
				   .map(|s| s.parse::<u32>()
				   .unwrap_or(0))
				   .sum();	

	return sum;
}


fn validate_adjacency(matrix: Vec<Vec<char>>, in_i: u32, 
	s_j: usize, e_j: usize) -> bool {

	let i = in_i as i32;
	let start_j = s_j as i32;
	let end_j = e_j as i32;

	let i_up = cmp::max(i-1, 0);
	let i_down = cmp::min(i+1, ((matrix.len() as u32)-1) as i32);

	let j_left = cmp::max(start_j-1, 0);
	let j_right = cmp::min(end_j, ((matrix[0].len())-1) as i32);

	for i in i_up..=i_down {
		for j in j_left..=j_right {
			if (matrix[i as usize][j as usize] != '.') 
			&& (!(matrix[i as usize][j as usize].is_digit(10))) {
				return true;
			}
		}
	}

	return false;
}

// for every number, check its bounds for a gear, 
// then check the gears bounds for another number
fn gear_ratio_sum(matrix: Vec<Vec<char>>) -> u32 {
	let width: usize = matrix[0].len();
	let height: usize = matrix.len();
	
	let mut sum: u32 = 0;

	for i in 0..height {
		for j in 0..width {
			if matrix[i][j] == '*' {
				//println!("gear: {},{}", i, j);	
				sum += validate_gear_ratio(matrix.clone(), i as i32, j as i32);
			}
		}
	}

	return sum;
}

fn validate_gear_ratio(matrix: Vec<Vec<char>>, i_gear: i32, j_gear: i32) -> u32 {
	let height: usize = matrix.len();
	let width: usize = matrix[0].len();

	let mut numbers: Vec<u32> = Vec::new();
	
	// left
	let mut num_l: Vec<char> = Vec::new();
	for j in (0..(j_gear as usize)).rev() {
		let c = matrix[i_gear as usize][j];
		if c.is_digit(10) {
			num_l.push(c);
		} else {
			break;
		}
	}
	if num_l.len() > 0 {
		num_l.reverse();
		numbers.push(num_l
					.into_iter()
					.collect::<String>()
					.parse::<u32>()
					.expect(""));
	}
	
	// right
	let mut num_r: Vec<char> = Vec::new();
	for j in ((j_gear as usize)+1)..width {
		let c = matrix[i_gear as usize][j];
		if c.is_digit(10) {
			num_r.push(c);
		} else {
			break;
		}
	}
	if num_r.len() > 0 {
		numbers.push(num_r
					.into_iter()
					.collect::<String>()
					.parse::<u32>()
					.expect(""));
	}
	
	// up
	if i_gear>0 {
		let mut num_up: Vec<char> = Vec::new();

		for j in 0..width {
			let c = matrix[i_gear as usize - 1][j];
			if c.is_digit(10) {
				num_up.push(c);
			}

			if (num_up.len()>0) && !c.is_digit(10) {
				if (j_gear-1 <= (j as i32)-1 && j_gear+1 >= (j as i32)-(num_up.len() as i32)) {
					numbers.push(num_up
								.into_iter()
								.collect::<String>()
								.parse::<u32>()
								.expect(""));
				}
				num_up = Vec::default();
			}
		}
	}
	
	// down
	if (i_gear as usize)+1 < height {
		let mut num_down: Vec<char> = Vec::new();

		for j in 0..width {
			let c = matrix[i_gear as usize + 1][j];
			if c.is_digit(10) {
				num_down.push(c);
			}

			if (num_down.len()>0) && !c.is_digit(10) {
				if (j_gear-1 <= (j as i32)-1 && j_gear+1 >= (j as i32)-(num_down.len() as i32)) {
					numbers.push(num_down
								.into_iter()
								.collect::<String>()
								.parse::<u32>()
								.expect(""));
				}
				num_down = Vec::default();
			}
		}
	}

	let mut ratio: u32 = 0;
	if numbers.len() == 2 {
		ratio = numbers[0]*numbers[1];
		//println!("{}*{} = {}", numbers[0], numbers[1], ratio);
	}

	return ratio;
}

pub fn part1() -> io::Result<u32> {
	let matrix: Vec<Vec<char>> = build_matrix()?;

	let sum: u32 = part_number_sum(matrix);

	Ok(sum)
}

pub fn part2() -> io::Result<u32> {
	let matrix: Vec<Vec<char>> = build_matrix()?;

	let sum: u32 = gear_ratio_sum(matrix);

	Ok(sum)
}