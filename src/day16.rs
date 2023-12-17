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
	mut dr: isize,
	mut dc: isize,
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
				(dr, dc) = (dc * -1, dr * -1);
			}
			'\\' => {
				(dr, dc) = (dc, dr);
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

	// == Part 1 ==
	let part1 = sub_solve(&grid, 0, 0, 0, 1);
	println!("Part 1: {}", part1);

	// == Part 2 ==
	// Left and right edges
	let lr_edges_max = (0..grid.len()).map(|r| {
		let left = sub_solve(&grid, r as isize, 0, 0, 1);
		let right = sub_solve(&grid, r as isize, grid[0].len() as isize - 1, 0, -1);
		return std::cmp::max(left, right);
	}).max().unwrap();

	// Top and bottom edges
	let tb_edges_max = (0..grid[0].len()).map(|c| {
		let top = sub_solve(&grid, 0, c as isize, 1, 0);
		let bottom = sub_solve(&grid, grid.len() as isize - 1, c as isize, -1, 0);
		return std::cmp::max(top, bottom);
	}).max().unwrap();

	let part2 = std::cmp::max(lr_edges_max, tb_edges_max);
	println!("Part 2: {part2}");
}
