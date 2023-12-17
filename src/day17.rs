use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashSet, BinaryHeap};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Dir {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Debug, Eq, PartialEq)]
struct State {
	r: isize,
	c: isize,
	dir: Dir,
	steps: usize, // steps so far in the current direction
	heat_loss: u32,
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> Ordering {
		self.heat_loss.cmp(&(other.heat_loss))
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn opposite_dir(dir: &Dir) -> Dir {
	match dir {
		Dir::Up => Dir::Down,
		Dir::Down => Dir::Up,
		Dir::Left => Dir::Right,
		Dir::Right => Dir::Left,
	}
}

fn sub_solve(grid: &Vec<Vec<u32>>, dist_to_turn: usize, max_dist: usize) -> u32 {
	let mut queue = BinaryHeap::new();
	queue.push(State{ r: 0, c: 1, dir: Dir::Right, steps: 1, heat_loss: grid[0][1] });
	queue.push(State{ r: 1, c: 0, dir: Dir::Down, steps: 1, heat_loss: grid[1][0] });

	let mut seen = HashSet::new();

	while !queue.is_empty() {
		let state = queue.pop().unwrap();
		let r = state.r;
		let c = state.c;
		let steps = state.steps;
		let heat_loss = state.heat_loss;

		if state.steps > max_dist {
			continue;
		}

		if !seen.insert((r, c, state.dir.clone(), steps)) {
			continue;
		}

		// Terminating condition
		if r as usize == grid.len() - 1 && c as usize == grid[0].len() - 1 && state.steps >= dist_to_turn {
			return heat_loss;
		}

		let mut try_dir = |dir| {
			// If we try to go in the opposite direction, we can't turn yet due to part 2 constraints, return.
			if dir == opposite_dir(&state.dir) || (dir != state.dir && state.steps < dist_to_turn) {
				return;
			}

			let (r, c) = match dir {
				Dir::Up => (state.r - 1, state.c),
				Dir::Down => (state.r + 1, state.c),
				Dir::Left => (state.r, state.c - 1),
				Dir::Right => (state.r, state.c + 1),
			};

			let steps = if dir == state.dir { state.steps + 1 } else { 1 };

			if r >= 0 && r < grid.len() as isize && c >= 0 && c < grid[0].len() as isize {
				let heat_loss = state.heat_loss + grid[r as usize][c as usize];
				queue.push(State{ r, c, dir, steps, heat_loss });
			}			
		};

		try_dir(Dir::Up);
		try_dir(Dir::Down);
		try_dir(Dir::Left);
		try_dir(Dir::Right);
	}

	panic!("Didn't find a solution");
}

pub fn solve(inputs: Vec<String>) {

	let grid = inputs
		.iter()
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
		.collect_vec();

	let part1 = sub_solve(&grid, /*dist_to_turn=*/ 1, /*max_dist=*/ 3 );
	let part2 = sub_solve(&grid, /*dist_to_turn=*/ 4, /*max_dist=*/ 10 );

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
