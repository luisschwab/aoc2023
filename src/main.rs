use aoc2023::{
    day01,
};

use std::collections::HashSet;
use std::env;

fn main() {
		let args = env::args().skip(1).collect::<HashSet<_>>();

  	if args.is_empty() || args.contains("1") {
		println!("day 01, part 1: {}", day01::part1().unwrap());
		println!("day 01, part 2: {}", day01::part2().unwrap());
		println!("");
	}
}