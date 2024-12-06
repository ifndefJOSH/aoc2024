use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Clone, Copy)]
enum GridSquare {
    Obstacle,
	Visited,
	Unvisited,
	Undefined,
}

// Contains slightly more info needed for part 2
#[derive(PartialEq, Clone, Copy)]
enum RichGridSquare {
    Obstacle,
	Unvisited,
	Undefined,
	VisitedNorth,
	VisitedEast,
	VisitedSouth,
	VisitedWest,
}

#[derive(Clone)]
enum Direction {
	North,
	South,
	East,
	West,
}

fn parse_input<P>(filename: P) -> (Vec<Vec<GridSquare>>, (i32, i32))
	where P: AsRef<Path>,
{
	let file = File::open(filename).unwrap();
	let lines = io::BufReader::new(file).lines();

	let (mut row, mut col) = (0i32, 0i32);
	let (mut cur_row, mut cur_col) = (0i32, 0i32);

	let grid = lines.filter_map(|line| {
		cur_col = 0;
		let l = line.unwrap();
		if l.len() == 0 {
			return None;
		}
		let v = l.chars().map(|c| {
			let sq = match c {
			   '.' => GridSquare::Unvisited,
			   '#' => GridSquare::Obstacle,
			   // We always start with moving north
			   '^' => GridSquare::Visited,
			   _ => GridSquare::Undefined,
			};
			if c == '^' {
				col = cur_col;
				row = cur_row;
			}
			cur_col += 1;
			sq
		}).collect::<Vec<_>>();
		cur_row += 1;
		Some(v)
	}).collect::<Vec<_>>();
	(grid, (row, col))
}

fn in_grid(row: i32, col: i32, row_bound: usize, col_bound: usize) -> bool {
	row >= 0 && col >= 0 && (row as usize) < row_bound && (col as usize) < col_bound
}

fn turn_90(d: Direction) -> Direction {
	match d {
	    Direction::North => Direction::East,
		Direction::East => Direction::South,
		Direction::South => Direction::West,
		Direction::West => Direction::North,
	}
}

fn next_idx(row: i32, col: i32, d: &Direction) -> (i32, i32) {
	match d {
	   Direction::North => (row - 1, col),
	   Direction::East => (row, col + 1),
	   Direction::South => (row + 1, col),
	   Direction::West => (row, col - 1),
	}
}

fn dir_to_visited(dir: &Direction) -> RichGridSquare {
	match dir {
		Direction::North => RichGridSquare::VisitedNorth,
		Direction::West => RichGridSquare::VisitedWest,
		Direction::East => RichGridSquare::VisitedEast,
		Direction::South => RichGridSquare::VisitedSouth,
	}
}

fn creates_loop(grid: &mut Vec<Vec<RichGridSquare>>, init_pos: (i32, i32)) -> bool {
	let (mut row, mut col) = init_pos;
	let mut dir = Direction::North;
	loop {
		// Mark current grid square as explored
		grid[row as usize][col as usize] = dir_to_visited(&dir);
		// Select next square
		let (mut next_row, mut next_col) = next_idx(row, col, &dir);
		if !in_grid(next_row, next_col, grid.len(), grid[0].len()) {
			return false;
		}
		while grid[next_row as usize][next_col as usize] == RichGridSquare::Obstacle {
			dir = turn_90(dir);
			(next_row, next_col) = next_idx(row, col, &dir);
		}
		// Make move
		(row, col) = (next_row, next_col);
		if grid[row as usize][col as usize] == dir_to_visited(&dir) {
			// println!("Found loop placing obstacle at {row},{col}");
			// We found a loop!
			return true;
		}
	}
}

pub fn process_input<P>(filename: P)
	where P: AsRef<Path>,
{
	println!("========= DAY 5 ==============");
	// Part 1
	let (mut grid, (mut row, mut col)) = parse_input(filename);
	let init_grid = grid.clone();
	let init_pos = (row, col);
	let mut dir = Direction::North;
	loop {
		// Mark current grid square as explored
		grid[row as usize][col as usize] = GridSquare::Visited;
		// Select next square
		let (mut next_row, mut next_col) = next_idx(row, col, &dir);
		if !in_grid(next_row, next_col, grid.len(), grid[0].len()) {
			break;
		}
		while grid[next_row as usize][next_col as usize] == GridSquare::Obstacle {
			dir = turn_90(dir);
			(next_row, next_col) = next_idx(row, col, &dir);
		}
		// Make move
		(row, col) = (next_row, next_col);
	}
	let part1_ans: usize = grid.iter().map(|row| {
		row.iter().filter(|sq| **sq == GridSquare::Visited).count()
	}).sum();
	println!("The answer for part 1 is {part1_ans}");
	// Part 2: try placing a single obstacle in every spot until we get a loop.
	// we get a loop if we reach the same square going the same direction as we did last time we
	// were there
	// println!("Init pos {:?}", init_pos);
	let part2_ans: usize = (0..init_grid.len()).map(|r| {
		let cur_row = &init_grid[r];
		(0..cur_row.len()).filter(|c| {
			if r == (init_pos.0 - 1) as usize && *c == init_pos.1 as usize {
				false
			} else if cur_row[*c] == GridSquare::Unvisited {
				let mut grid_cpy = grid.iter().map(|rw| {
					rw.iter().map(|sq| {
						match sq {
						   GridSquare::Unvisited => RichGridSquare::Unvisited,
						   // We always start by moving north, but we want
						   // to let the other function handl the placing
						   GridSquare::Visited => RichGridSquare::Unvisited,
						   GridSquare::Obstacle => RichGridSquare::Obstacle,
						   GridSquare::Undefined => RichGridSquare::Undefined,
						}
					}).collect::<Vec<_>>()
				}).collect::<Vec<_>>();
				grid_cpy[r][*c] = RichGridSquare::Obstacle;
				let cl = creates_loop(&mut grid_cpy, init_pos);
				// println!("Position {r},{c} creates a loop? {cl}");
				cl
			} else {
				false
			}
		}).count()
	}).sum();
	println!("The answer for part 2 is {part2_ans}");
}
