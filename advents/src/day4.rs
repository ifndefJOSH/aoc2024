use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// Inner vectors required to have length 6
fn parse_input<P>(filename: P) -> Vec<Vec<char>>
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
		Some(l.chars().collect())
	}).collect::<Vec<Vec<char>>>()
}

fn is_word(a : char, b : char, c : char, d : char) -> bool {
	(a == 'X' && b == 'M' && c == 'A' && d == 'S')
		|| (a == 'S' && b == 'A' && c == 'M' && d == 'X')
}

fn is_mas_forward(a : char, b : char, c : char) -> bool {
	a == 'M' && b == 'A' && c == 'S'
}

fn is_mas_backward(a : char, b : char, c : char) -> bool {
	a == 'S' && b == 'A' && c == 'M'
}

fn part_1(input : &Vec<Vec<char>>) -> u64 {
	let mut cnt : u64 = 0;
	for i in 0..input.len() {
		let row = &input[i];
		for j in 0..row.len() {
			// Horizontal forward
			if j < row.len() - 3
				&& is_word(row[j], row[j + 1], row[j + 2], row[j + 3]) {
				cnt += 1;
			}
			// Vertical forward
			if i < input.len() - 3
				&& is_word(input[i][j], input[i + 1][j], input[i + 2][j], input[i + 3][j]) {
				cnt += 1;
			}
			// Forward-down diagonal
			if i < input.len() - 3 && j < row.len() - 3 // all rows are the same length, right?
				&& is_word(input[i][j], input[i + 1][j + 1], input[i + 2][j + 2], input[i + 3][j + 3]) {
				cnt += 1;
			}
			// Backward-down diagonal
			if i < input.len() - 3 && j >= 3
				&& is_word(input[i][j], input[i + 1][j - 1], input[i + 2][j - 2], input[i + 3][j - 3]) {
				cnt += 1;
			}
		}
	}
	cnt
}

fn part_2(input : &Vec<Vec<char>>) -> u64 {
	let mut cnt : u64 = 0;
	for i in 0..input.len() - 2 {
		for j in 0..input.len() - 2 {
			if (is_mas_forward(input[i][j], input[i + 1][j + 1], input[i + 2][j + 2])
					&& is_mas_forward(input[i][j + 2], input[i + 1][j + 1], input[i + 2][j]))
			|| (is_mas_backward(input[i][j], input[i + 1][j + 1], input[i + 2][j + 2])
					&& is_mas_backward(input[i][j + 2], input[i + 1][j + 1], input[i + 2][j]))
			|| (is_mas_forward(input[i][j], input[i + 1][j + 1], input[i + 2][j + 2])
					&& is_mas_backward(input[i][j + 2], input[i + 1][j + 1], input[i + 2][j]))
			|| (is_mas_backward(input[i][j], input[i + 1][j + 1], input[i + 2][j + 2])
					&& is_mas_forward(input[i][j + 2], input[i + 1][j + 1], input[i + 2][j]))
{
				cnt += 1;
			}
		}
	}
	cnt
}

pub fn process_input<P>(filename: P)
	where P: AsRef<Path>,
{
	println!("========= DAY 4 ==============");
	let grid = parse_input(filename);
	// Part 1
	let part1_ans = part_1(&grid);
	println!("The answer for part 1 is {part1_ans}");
	let part2_ans = part_2(&grid);
	println!("The answer for part 2 is {part2_ans}");
}
