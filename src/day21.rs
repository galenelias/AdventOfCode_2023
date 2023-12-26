use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

fn count_squares_in_steps(
	distances_map: &HashMap<(isize, isize), Vec<Vec<Option<usize>>>>,
	row_count: usize,
	col_count: usize,
	distance: usize,
) -> usize {
	let mut result = 0;
	for distances in distances_map.values() {
		for r in 0..row_count {
			for c in 0..col_count {
				if let Some(dist) = distances[r][c] {
					if dist == distance || (dist < distance && dist % 2 == distance % 2) {
						result += 1;
					}
				}
			}
		}
	}

	return result;
}

fn sub_solve(
	grid: &Vec<Vec<char>>,
	start_pos: (isize, isize),
	distance: usize,
	part2: bool,
) -> usize {
	// Map of (grid_r, grid_c) -> (distance grid)
	let mut distances_map = HashMap::new();

	let mut queue = VecDeque::new();
	queue.push_back((
		(start_pos.0 as isize, start_pos.1 as isize),
		(0isize, 0isize),
		0,
	));

	let mut report_dist = 1;
	let mut values = Vec::new();

	while !queue.is_empty() {
		let (pos, grid_num, steps) = queue.pop_front().unwrap();

		if part2 && steps > report_dist {
			if report_dist >= 65 && ((report_dist - 65) % 131) == 0 {
				let count =
					count_squares_in_steps(&distances_map, grid.len(), grid[0].len(), report_dist);
				values.push(count);

				// Fit a quadratic equation to the first 3 values:  a*i^2 + b*i + c
				if values.len() == 3 {
					let c = values[0];
					for a in 0..values[1] {
						for b in 0..values[1] {
							if a + b + c == values[1] && a * 4 + b * 2 + c == values[2] {
								let x = (26501365 - 65) / 131;
								return a * x * x + b * x + c;
							}
						}
					}
				}
			}
			report_dist += 1;
		}

		let distances = distances_map
			.entry(grid_num)
			.or_insert(vec![vec![None; grid[0].len()]; grid.len()]);

		if !distances[pos.0 as usize][pos.1 as usize].is_none() {
			continue;
		}

		distances[pos.0 as usize][pos.1 as usize] = Some(steps);

		if steps == distance {
			continue;
		}

		let adjacents = [
			(pos.0 - 1, pos.1),
			(pos.0 + 1, pos.1),
			(pos.0, pos.1 - 1),
			(pos.0, pos.1 + 1),
		];

		for mut adj in adjacents.into_iter() {
			let mut new_grid_num = grid_num;

			// For part 2, simulate an infinite grid by wrapping our grid coordinates and then having a 'grid number'
			if part2 {
				if adj.0 < 0 {
					adj.0 = grid.len() as isize - 1;
					new_grid_num.0 -= 1;
				}
				if adj.0 >= grid.len() as isize {
					adj.0 = 0;
					new_grid_num.0 += 1;
				}
				if adj.1 < 0 {
					adj.1 = grid[0].len() as isize - 1;
					new_grid_num.1 -= 1;
				}
				if adj.1 >= grid[0].len() as isize {
					adj.1 = 0;
					new_grid_num.1 += 1;
				}
			}

			if adj.0 >= 0
				&& adj.0 < grid.len() as isize
				&& adj.1 >= 0 && adj.1 < grid[0].len() as isize
			{
				let ch = grid[adj.0 as usize][adj.1 as usize];
				if ch == '.' {
					queue.push_back((adj, new_grid_num, steps + 1));
				}
			}
		}
	}

	return count_squares_in_steps(&distances_map, grid.len(), grid[0].len(), distance);
}

pub fn solve(inputs: Vec<String>) {
	let mut grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let start_pos = grid
		.iter()
		.enumerate()
		.find_map(|(r, line)| {
			line.iter().enumerate().find_map(|(c, &ch)| {
				if ch == 'S' {
					Some((r as isize, c as isize))
				} else {
					None
				}
			})
		})
		.unwrap();

	grid[start_pos.0 as usize][start_pos.1 as usize] = '.';

	let part1 = sub_solve(&grid, start_pos, 64, /*part2=*/ false);
	let part2 = sub_solve(&grid, start_pos, 501, /*part2=*/ true);

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
