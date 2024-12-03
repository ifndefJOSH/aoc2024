use std::fs::{self};
use std::path::Path;

use regex::Regex;

fn parse_input<P>(filename: &P, part1: bool) -> Vec<(i64, i64)>
	where P: AsRef<Path>,
{
	let input: String = fs::read_to_string(filename).unwrap();
	let re = if part1 {
		Regex::new(r"mul\((\d+),(\d+)\)")
			.expect("Cannot create regex")
	} else {
		Regex::new(r"(do\(\)|don't\(\)|mul\(\d+,\d+\))")
			.expect("Cannot create regex")
	};

	let (i, j) = (1, 2);
	if part1 {
		re.captures_iter(&input).map(|cap| {
			(cap[i].parse::<i64>().unwrap(), cap[j].parse::<i64>().unwrap())
		}).collect::<Vec<(i64, i64)>>()
	} else {
		let mut enabled: bool = true;
		re.captures_iter(&input).filter_map(|cap| {
			// println!("{:?}", cap);
			if let Some(mtch) = cap.get(1) {
				let mstr = mtch.as_str();
				if mstr == "do()" {
					enabled = true;
					None
				} else if mstr == "don't()" {
				    enabled = false;
					None
				} else if enabled {
					let tuple_vec = mstr
						.replace("mul(", "")
						.replace(")", "")
						.split(",")
						.map(|i| i.parse::<i64>().unwrap())
						.collect::<Vec<_>>();
					Some((tuple_vec[0], tuple_vec[1]))
				} else {
					None
				}
			} else {
				None
			}
		}).collect::<Vec<(i64, i64)>>()
	}

}

pub fn process_input<P>(filename: P)
	where P: AsRef<Path>,
{
	println!("========= DAY 3 ==============");
	let data = parse_input(&filename, true);
	let part1_ans: i64 = data.iter().map(|t| {
		t.1 * t.0
	}).sum();
	println!("Part 1 answer is {part1_ans}");
	let data2 = parse_input(&filename, false);
	let part2_ans: i64 = data2.iter().map(|t| {
		println!("{:?}", t);
		t.1 * t.0
	}).sum();
	println!("Part 2 answer is {part2_ans}");

}

