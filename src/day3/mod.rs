use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

type Grid = Vec<Vec<char>>;

fn get_adjacent_points((r, c): (usize, usize), grid: &Grid) -> Vec<(usize, usize)> {
	let mut result = Vec::new();
	if r > 0 && c > 0 {
		result.push((r - 1, c - 1));
	}
	if r > 0 {
		result.push((r - 1, c));
	}
	if r > 0 && c < grid[r - 1].len() - 1 {
		result.push((r - 1, c + 1));
	}
	if c > 0 {
		result.push((r, c - 1));
	}
	if c < grid[r].len() - 1 {
		result.push((r, c + 1));
	}
	if r < grid.len() - 1 && c > 0 {
		result.push((r + 1, c - 1));
	}
	if r < grid.len() - 1 {
		result.push((r + 1, c));
	}
	if r < grid.len() - 1 && c < grid[r + 1].len() - 1 {
		result.push((r + 1, c + 1));
	}
	return result;
}

fn adjacent_to_symbol(grid: &Grid, r: usize, c: usize) -> bool {
	let mut visited = HashSet::new();
	let mut q: VecDeque<(usize, usize)> = VecDeque::new();
	q.push_back((r, c));

	while !q.is_empty() {
		let (r, c) = q.pop_front().unwrap();
		let ch = grid[r][c];

		if !visited.insert((r, c)) {
			continue;
		}

		if !ch.is_ascii_digit() && ch != '.' {
			return true;
		}

		for adjacent in get_adjacent_points((r, c), grid) {
			if grid[adjacent.0][adjacent.1] != '.' {
				q.push_back(adjacent);
			}
		}
	}

	return false;
}

pub fn solve(inputs: Vec<String>) {
	let grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let mut part1 = 0;

	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			if grid[r][c].is_ascii_digit() && (c == 0 || !grid[r][c - 1].is_ascii_digit()) {
				// Start of word, check if adjacent to symbol
				if adjacent_to_symbol(&grid, r, c) {
					part1 += grid[r][c..]
						.iter()
						.take_while(|c| c.is_ascii_digit())
						.collect::<String>()
						.parse::<u32>()
						.unwrap();
				}
			}
		}
	}

	println!("Part 1: {}", part1);

	let mut part2: u64 = 0;

	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			if grid[r][c] != '*' {
				continue;
			}

			println!("Checking {}, {}", r, c);
			let mut adjacent_numbers = Vec::<(usize, usize)>::new();
			for adjacent in get_adjacent_points((r, c), &grid) {
				if grid[adjacent.0][adjacent.1].is_ascii_digit() {
					if adjacent.1 == c - 1 || !grid[adjacent.0][adjacent.1 - 1].is_ascii_digit() {
						adjacent_numbers.push(adjacent);
					}
				}
			}

			println!("Adjacent numbers: {:?}", adjacent_numbers);
			if adjacent_numbers.len() == 2 {
				println!("Gear at {}, {}", r, c);
				let mut product: u64 = 1;
				for num in adjacent_numbers {
					let mut num_start = num.1;
					while num_start > 0 && grid[num.0][num_start - 1].is_ascii_digit() {
						num_start -= 1;
					}
					product *= grid[num.0][num_start..]
						.iter()
						.take_while(|c| c.is_ascii_digit())
						.collect::<String>()
						.parse::<u64>()
						.unwrap();
				}
				part2 += product;
			}
		}
	}

	println!("Part 2: {}", part2);
}
