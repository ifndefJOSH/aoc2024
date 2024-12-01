use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn create_lists<P>(filename: P) -> (Vec<u32>, Vec<u32>)
	where P: AsRef<Path>,
{
	let file = File::open(filename).unwrap();
	let re = Regex::new(r"(\d+)\s+(\d+)")
		.expect("Failed to create regex");

	let mut left = Vec::<u32>::new();
	let mut right = Vec::<u32>::new();
	for line in io::BufReader::new(file).lines() {
		let l = line.unwrap();
		if let Some(captures) = re.captures(&l) {
			// Only push if there are two numbers on the line
			let first = captures[1].parse::<u32>().unwrap();
			let second = captures[2].parse::<u32>().unwrap();
			left.push(first);
			right.push(second);
		}
	}
	(left, right)
}

fn process_input<P>(filename: P) 
	where P: AsRef<Path>,
{
	let (mut l, mut r) = create_lists(filename);
	l.sort();
	r.sort();
	// Part 1
	let answer_pt1: u32 = l.iter().zip(r.iter()).map(|(a, b)| a.abs_diff(*b)).sum();
	println!("The answer for part 1 is {answer_pt1}");
	// Part 2
	let answer_pt2: u32 = l.iter().map(|i| {
		// The number of times this occurs in the other vector
		let weight = r.iter().filter(|j| **j == *i).count() as u32;
		i * weight
	}).sum();
	println!("The answer for part 2 is {answer_pt2}");

}

fn main() {
	std::env::args().skip(1).for_each(|f| {
		process_input(f);
	});
}
