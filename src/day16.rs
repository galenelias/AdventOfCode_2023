use itertools::Itertools;
use std::collections::HashSet;

fn sub_solve(grid: &Vec<Vec<char>>, r: isize, c: isize, dr: isize, dc: isize) -> usize {
	let mut beams: HashSet<(isize, isize, isize, isize)> = HashSet::new();
	cast_beam(&grid, r, c, dr, dc, &mut beams);
	return beams
		.iter()
		.map(|(r, c, _, _)| (r, c))
		.collect::<HashSet<_>>()
		.len();
}

fn cast_beam(
	grid: &Vec<Vec<char>>,
	mut r: isize,
	mut c: isize,
	dr: isize,
	dc: isize,
	beams: &mut HashSet<(isize, isize, isize, isize)>,
) {
	while r >= 0 && r < grid.len() as isize && c >= 0 && c < grid[0].len() as isize {
		if !beams.insert((r, c, dr, dc)) {
			return;
		}

		let ch = grid[r as usize][c as usize];

		match ch {
			'.' => (),
			'/' => {
				let (dr, dc) = (dc * -1, dr * -1);
				return cast_beam(grid, r + dr, c + dc, dr, dc, beams);
			}
			'\\' => {
				let (dr, dc) = (dc, dr);
				return cast_beam(grid, r + dr, c + dc, dr, dc, beams);
			}
			'|' => match (dr, dc) {
				(-1, 0) | (1, 0) => (),
				(0, -1) | (0, 1) => {
					cast_beam(grid, r - 1, c, -1, 0, beams);
					return cast_beam(grid, r + 1, c, 1, 0, beams);
				}
				_ => unreachable!("Unexpected direction: {} {}", dr, dc),
			},
			'-' => match (dr, dc) {
				(0, -1) | (0, 1) => (),
				(-1, 0) | (1, 0) => {
					cast_beam(grid, r, c - 1, 0, -1, beams);
					return cast_beam(grid, r, c + 1, 0, 1, beams);
				}
				_ => unreachable!("Unexpected direction: {} {}", dr, dc),
			},
			_ => unreachable!("Unexpected grid character: {}", ch),
		}

		r += dr;
		c += dc;
	}
}

pub fn solve(inputs: Vec<String>) {
	let grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let part1 = sub_solve(&grid, 0, 0, 0, 1);
	println!("Part 1: {}", part1);

	let mut part2 = 0;

	// Left and right edges
	for r in 0..grid.len() {
		let left = sub_solve(&grid, r as isize, 0, 0, 1);
		let right = sub_solve(&grid, r as isize, grid[0].len() as isize - 1, 0, -1);
		part2 = std::cmp::max(part2, left);
		part2 = std::cmp::max(part2, right);
	}

	// Top and bottom edges
	for c in 0..grid[0].len() {
		let top = sub_solve(&grid, 0, c as isize, 1, 0);
		let bottom = sub_solve(&grid, grid.len() as isize - 1, c as isize, -1, 0);
		part2 = std::cmp::max(part2, top);
		part2 = std::cmp::max(part2, bottom);
	}

	println!("Part 2: {}", part2);
}
