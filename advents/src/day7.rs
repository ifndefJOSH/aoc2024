use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Mul,
    Add,
	Cat,
}

fn parse_input<P>(filename: P) -> Vec<(u128, Vec<u32>)>
	where P: AsRef<Path>,
{
	let file = File::open(filename).unwrap();
	let lines = io::BufReader::new(file).lines();

	lines.filter_map(|line| {
		let l = line.unwrap();
		if l.len() == 0 {
			return None;
		}
		let res_and_eq = l.split(":").collect::<Vec<_>>();
		let res = res_and_eq[0].parse::<u128>().unwrap();
		let eq = res_and_eq[1]
			.trim()
			.split(" ")
			.map(|n| n.parse::<u32>().unwrap())
			.collect::<Vec<_>>();
		Some((res, eq))
	}).collect::<Vec<_>>()
}

fn apply_eq(flag: u128, eq: &Vec<u32>) -> u128 {
	let mut idx: usize = 0;
	eq.iter().skip(1).fold(eq[0] as u128, |last, cur|{
		let op = if flag >> idx & 0x1 != 0 { Op::Mul } else { Op::Add };
		idx += 1;
		if op == Op::Mul { last * (*cur as u128) } else { last + (*cur as u128) }
	})
}

fn equation_works(res: u128, eq: &Vec<u32>) -> bool {
	let spots = eq.len() - 1;
	let base: u128 = 2;
	// We'll cycle through all combinations by treating 0 like add and 1
	// like multiply in a binary number
	for flag in 0..=base.pow(spots as u32) {
		if apply_eq(flag, eq) == res {
			return true;
		}
	}
	return false;
}

fn next_flag_pt2(flag: &mut Vec<Op>) -> bool {
	let mut i = 0;
	while i < flag.len() && flag[i] == Op::Cat {
		flag[i] = Op::Mul;
		i += 1;
	}
	if i == flag.len() {
		// We've reached the end
		false
	} else {
		if flag[i] == Op::Mul {
			flag[i] = Op::Add;
		} else if flag[i] == Op::Add {
			flag[i] = Op::Cat;
		} else {
			panic!("Ya done messed up!");
		}
		true
	}
}

fn apply_eq_part2(flag: &Vec<Op>, eq: &Vec<u32>) -> u128 {
	let mut idx: usize = 0;
	let mut flag_idx = 0;
	eq.iter().skip(1).fold(eq[0] as u128, |last, cur|{
		let op = &flag[flag_idx];
		flag_idx += 1;
		if *op == Op::Mul {
			last * (*cur as u128)
		} else if *op == Op::Add {
			last + (*cur as u128)
		} else if *op == Op::Cat {
			let mut concat = last.to_string();
			concat.push_str(&cur.to_string());
			concat.parse::<u128>().unwrap()
		} else {
			last
		}
	})
}

fn equation_works_part2(res: u128, eq: &Vec<u32>) -> bool {
	let spots = eq.len() - 1;
	// We'll cycle through all combinations by treating 0 like add and 1
	// like multiply in a binary number
	let mut flag = (0..spots).map(|_| Op::Mul).collect::<Vec<_>>();
	loop {
		if apply_eq_part2(&flag, eq) == res {
			return true;
		}
		let has_next_flag = next_flag_pt2(&mut flag);
		// println!("{:?}", flag);
		if !has_next_flag {
			return false;
		}
	}
}

pub fn process_input<P>(filename: P)
	where P: AsRef<Path>,
{
	println!("========= DAY 7 ==============");
	// Part 1
	let equations = parse_input(filename);
	let (working, not_working): (Vec<_>, Vec<_>) = equations.iter().partition(|res_eq| {
		let res = &res_eq.0;
		let eq = &res_eq.1;
		equation_works(*res, eq)

	});
	let answer_part1: u128 = working.iter().map(|t| t.0).sum();
	println!("The answer for part 1 is {answer_part1}");
	let addl_sum: u128 = not_working.iter().filter_map(|res_eq| {
		let res = &res_eq.0;
		let eq = &res_eq.1;
		if equation_works_part2(*res, eq) {
			Some(res)
		} else {
			None
		}
	}).sum();
	let answer_part2 = answer_part1 + addl_sum;
	println!("The answer for part 2 is {answer_part2}");
}
