use itertools::Itertools;

fn sub_solve(grids: &Vec<Vec<Vec<char>>>, target_mismatches: usize) -> usize {
	grids
		.iter()
		.map(|grid| {
			let mut result = 0;
			for r in 1..grid.len() {
				let mut num_mismatch = 0;
				'outer: for dr in 0..std::cmp::min(r, grid.len() - r) {
					for c in 0..grid[0].len() {
						if grid[r - dr - 1][c] != grid[r + dr][c] {
							num_mismatch += 1;
							if num_mismatch > target_mismatches {
								break 'outer;
							}
						}
					}
				}

				if num_mismatch == target_mismatches {
					result += 100 * r;
					break;
				}
			}

			for c in 1..grid[0].len() {
				let mut num_mismatch = 0;
				'outer: for dc in 0..std::cmp::min(c, grid[0].len() - c) {
					for r in 0..grid.len() {
						if grid[r][c - dc - 1] != grid[r][c + dc] {
							num_mismatch += 1;
							if num_mismatch > target_mismatches {
								break 'outer;
							}
						}
					}
				}

				if num_mismatch == target_mismatches {
					result += c;
					break;
				}
			}
			return result;
		})
		.sum::<usize>()
}

pub fn solve(inputs: Vec<String>) {
	let grids = inputs
		.split(|i| i.is_empty())
		.map(|grid| {
			grid.iter()
				.map(|line| line.chars().collect_vec())
				.collect_vec()
		})
		.collect_vec();

	println!("Part 1: {}", sub_solve(&grids, 0));
	println!("Part 2: {}", sub_solve(&grids, 1));
}
