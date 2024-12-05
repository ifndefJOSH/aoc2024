use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_input<P>(filename: P) -> (Vec<Vec<u16>>, Vec<Vec<u16>>)
	where P: AsRef<Path>,
{
	let file = File::open(filename).unwrap();
	let lines = io::BufReader::new(file).lines();

	let mut ordering_rules = Vec::<Vec<u16>>::new();
	let mut updates = Vec::<Vec<u16>>::new();
	let mut in_ordering = true;
	for line in lines {
		let line_str = line.unwrap();
		if line_str.len() == 0 {
			in_ordering = false;
			continue;
		} else if in_ordering {
			ordering_rules.push(
				line_str.split("|")
				.map(|n| n.parse::<u16>()
					.unwrap()).collect::<Vec<u16>>());
		} else {
			updates.push(
				line_str.split(",")
				.map(|n| n.parse::<u16>()
					.unwrap()).collect::<Vec<u16>>());

		}
	}
	(ordering_rules, updates)
}

// Option so I can use try_fold to short circuit
fn satisfies_order(order: &Vec<u16>, update: &Vec<u16>) -> Option<bool> {
	assert!(order.len() == 2);
	for i in 1..update.len() {
		if update[i] == order[0] {
			for j in 0..i {
				if update[j] == order[1] {
					return None;
				}
			}
		}
	}
	Some(true)
}

fn swap_order(order: &[u16], update: &mut [u16]) {
	assert!(order.len() == 2);
	for i in 1..update.len() {
		if update[i] == order[0] {
			for j in 0..i {
				if update[j] == order[1] {
					// Swap update[i] and update[j]
					update.swap(i, j);
				}
			}
		}
	}
}


pub fn process_input<P>(filename: P)
	where P: AsRef<Path>,
{
	println!("========= DAY 5 ==============");
	let (ordering_rules, updates) = parse_input(filename);
	// Part 1
	let part1_ans: u16 = updates.iter().filter(|update| {
		ordering_rules.iter()
			.try_fold(true, |_, order| satisfies_order(order, update)).is_some()
	})
	.map(|succeeding_update| {
		// println!("{:?}", succeeding_update);
		// println!("{}", succeeding_update[succeeding_update.len() / 2]);
		succeeding_update[succeeding_update.len() / 2]
	}).sum();
	println!("Part 1 answer is {part1_ans}");
	let part2_ans: u16 = updates.iter().filter(|update| {
		// only get the failing ones
		ordering_rules.iter()
			.try_fold(true, |_, order| satisfies_order(order, update)).is_none()
	})
	.map(|failing_update| {
		let mut modified_update = failing_update.clone();
		while ordering_rules.iter()
			.try_fold(true, |_, order| satisfies_order(order, &modified_update)).is_none() {
			for rule in ordering_rules.clone() {
				swap_order(&rule, &mut modified_update);
			}
		}
		assert!(ordering_rules.iter()
			.try_fold(true, |_, order| satisfies_order(order, &modified_update)).is_some());
		modified_update[modified_update.len() / 2]
	}).sum();
	println!("Part 2 answer is {part2_ans}");
}
