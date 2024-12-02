use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// Inner vectors required to have length 6
fn parse_input<P>(filename: P) -> Vec<Vec<u32>>
	where P: AsRef<Path>,
{
	let file = File::open(filename).unwrap();
	let lines = io::BufReader::new(file).lines();

	lines.filter_map(|line| {
		// println!("{:?}", line);
		let l = line.unwrap();
		if l.len() == 0 {
			return None;
		}
		let v = l.split(" ")
			.map(|n| n.parse::<u32>().unwrap())
			.collect::<Vec<u32>>();
		if v.len() == 0 { None } else { Some(v) }

	}).collect::<Vec<Vec<u32>>>()
}

fn safe(level: &Vec<u32>) -> bool {
	if level.len() == 0 {
		return true;
	}
	let diffs_ok = level.iter().skip(1).try_fold(level[0], |last, cur| {
		let d = last.abs_diff(*cur);
		if d > 0 && d <= 3 { Ok(*cur) } else { Err(*cur) }
	}).is_ok();
	let increasing = level.iter()
		.try_fold(0u32, |last, cur|
			if *cur > last { Ok(*cur) } else { Err(*cur) }
		).is_ok();
	if increasing {
		// println!("{:?} is increasing {}", level, diffs_ok);
		return diffs_ok;
	}
	let decreasing = level.iter()
		.try_fold(u32::MAX, |last, cur|
			if *cur < last { Ok(*cur) } else { Err(*cur) }
		).is_ok();
	if decreasing {
		// println!("{:?} is decreasing {}", level, diffs_ok);
		return diffs_ok;
	}
	false
}

fn safe_pt2(level: &Vec<u32>) -> bool {
	let s = safe(&level);
	if s {
		return true;
	}
	for i in 0..level.len() {
		let mut l_copy = level.clone();
		l_copy.remove(i);
		if safe(&l_copy) {
			return true;
		}
	}
	return false;
}

pub fn process_input<P>(filename: P)
	where P: AsRef<Path>,
{
	println!("========= DAY 2 ==============");
	let reports = parse_input(filename);
	// Part 1
	let part1_ans = reports.iter().filter(|report| safe(&report.to_vec())).count();
	println!("The answer for part 1 is {part1_ans}");
	let part2_ans = reports.iter().filter(|report| safe_pt2(&report.to_vec())).count();
	println!("The answer for part 2 is {part2_ans}");
}
