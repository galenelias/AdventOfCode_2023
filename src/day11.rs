use itertools::Itertools;
use std::cmp;

pub fn solve(inputs: Vec<String>) {
	let grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let horizontal_gaps = grid
		.iter()
		.enumerate()
		.filter_map(|(r, line)| {
			if line.iter().all(|&ch| ch == '.') {
				Some(r)
			} else {
				None
			}
		})
		.collect_vec();

	let vertical_gaps = (0..grid[0].len())
		.filter_map(|c| {
			if grid.iter().all(|row| row[c] == '.') {
				Some(c)
			} else {
				None
			}
		})
		.collect_vec();

	let mut galaxies = Vec::new();
	for r in 0..grid.len() {
		for c in 0..grid[0].len() {
			if grid[r][c] == '#' {
				galaxies.push((r, c));
			}
		}
	}

	let sub_solve = |gap_factor: usize| {
		let mut sum: usize = 0;
		for i in 0..galaxies.len() {
			for j in i + 1..galaxies.len() {
				let (r1, c1) = (
					cmp::min(galaxies[i].0, galaxies[j].0),
					cmp::min(galaxies[i].1, galaxies[j].1),
				);
				let (r2, c2) = (
					cmp::max(galaxies[i].0, galaxies[j].0),
					cmp::max(galaxies[i].1, galaxies[j].1),
				);
				sum += (r2 - r1) + (c2 - c1);

				sum += horizontal_gaps
					.iter()
					.filter(|&&r| r > r1 && r < r2)
					.count() * (gap_factor - 1);
				sum +=
					vertical_gaps.iter().filter(|&&c| c > c1 && c < c2).count() * (gap_factor - 1);
			}
		}
		return sum;
	};

	println!("Part 1: {}", sub_solve(2));
	println!("Part 2: {}", sub_solve(1000000));
}
