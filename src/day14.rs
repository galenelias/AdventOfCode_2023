use itertools::Itertools;
use std::collections::HashMap;

enum Dir {
	North,
	West,
	South,
	East,
}

fn total_load(grid: &Vec<Vec<char>>) -> usize {
	grid.iter()
		.enumerate()
		.map(|(r, row)| {
			row.iter().filter_map(move |&ch| {
				if ch == 'O' {
					Some(grid.len() - r)
				} else {
					None
				}
			})
		})
		.flatten()
		.sum::<usize>()
}

fn tilt_grid(grid: &mut Vec<Vec<char>>, rocks: &mut Vec<(usize, usize)>, dir: &Dir) {
	rocks.sort_by(|a, b| match dir {
		Dir::North => a.0.cmp(&b.0),
		Dir::West => a.1.cmp(&b.1),
		Dir::South => b.0.cmp(&a.0),
		Dir::East => b.1.cmp(&a.1),
	});

	for rock in rocks {
		grid[rock.0][rock.1] = '.';
		match dir {
			Dir::North => {
				while rock.0 > 0 && grid[rock.0 - 1][rock.1] == '.' {
					rock.0 -= 1;
				}
			}
			Dir::West => {
				while rock.1 > 0 && grid[rock.0][rock.1 - 1] == '.' {
					rock.1 -= 1;
				}
			}
			Dir::South => {
				while rock.0 < grid.len() - 1 && grid[rock.0 + 1][rock.1] == '.' {
					rock.0 += 1;
				}
			}
			Dir::East => {
				while rock.1 < grid[0].len() - 1 && grid[rock.0][rock.1 + 1] == '.' {
					rock.1 += 1;
				}
			}
		}

		grid[rock.0][rock.1] = 'O';
	}
}

pub fn solve(inputs: Vec<String>) {
	let mut grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let mut rocks = grid
		.iter()
		.enumerate()
		.map(|(r, row)| {
			row.iter().enumerate().filter_map(
				move |(c, &ch)| {
					if ch == 'O' {
						Some((r, c))
					} else {
						None
					}
				},
			)
		})
		.flatten()
		.collect_vec();

	let mut part1_grid = grid.clone();
	let mut part1_rocks = rocks.clone();
	tilt_grid(&mut part1_grid, &mut part1_rocks, &Dir::North);
	println!("Part 1: {}", total_load(&part1_grid));

	let directions = [Dir::North, Dir::West, Dir::South, Dir::East];
	let mut states = HashMap::new();

	for i in 1.. {
		for dir in &directions {
			tilt_grid(&mut grid, &mut rocks, dir);
		}

		if let Some(seen_at) = states.get(&grid) {
			// To make the math easier, just wait until our current grid modulo the cycle length perfectly aligns with the 1000000000 entry
			let cycle_len = i - seen_at;
			if (1000000000 % cycle_len) == *seen_at {
				println!("Part 2: {}", total_load(&grid));
				break;
			}
		} else {
			states.insert(grid.clone(), i);
		}
	}
}
