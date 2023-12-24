use itertools::Itertools;
use std::collections::{VecDeque, HashSet};

fn is_intersection(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
	let adjacent_paths = if grid[pos.0 - 1][pos.1] != '#' { 1 } else { 0 }
		+ if grid[pos.0 + 1][pos.1] != '#' { 1 } else { 0 }
		+ if grid[pos.0][pos.1 - 1] != '#' { 1 } else { 0 }
		+ if grid[pos.0][pos.1 + 1] != '#' { 1 } else { 0 };

	return grid[pos.0][pos.1] != '#' && adjacent_paths > 2;
}

// Run BFS from one intersection to another to calculate distances between them
fn bfs(grid: &Vec<Vec<char>>, start_pos: (usize, usize), end_pos: (usize, usize), part2: bool) -> Option<usize> {
	let mut visited = HashSet::new();
	let mut queue = VecDeque::new();

	queue.push_back((start_pos, 0));
	while !queue.is_empty() {
		let (pos, steps) = queue.pop_front().unwrap();
		if grid[pos.0][pos.1] == '#' {
			continue;
		} else if pos == end_pos {
			return Some(steps);
		} else if steps > 0 && is_intersection(grid, pos) {
			continue;
		}

		if !visited.insert(pos) {
			continue;
		}

		let ch = grid[pos.0][pos.1];

		if ch == '.' || (part2 && ch != '#') {
			queue.push_back(((pos.0 - 1, pos.1), steps + 1));
			queue.push_back(((pos.0 + 1, pos.1), steps + 1));
			queue.push_back(((pos.0, pos.1 - 1), steps + 1));
			queue.push_back(((pos.0, pos.1 + 1), steps + 1));
		} else if ch == '^' {
			queue.push_back(((pos.0 - 1, pos.1), steps + 1));
		} else if ch == 'v' {
			queue.push_back(((pos.0 + 1, pos.1), steps + 1));
		} else if ch == '<' {
			queue.push_back(((pos.0, pos.1 - 1), steps + 1));
		} else if ch == '>' {
			queue.push_back(((pos.0, pos.1 + 1), steps + 1));
		}
	}

	return None;
}

// Run DFS across the intersections to find the longest path
fn dfs(distances: &Vec<Vec<Option<usize>>>, start_index: usize, end_index: usize, visited: &mut Vec<bool>) -> Option<usize> {
	let mut result = None;

	for node in 0..distances.len() {
		if node == start_index || visited[node] {
			continue;
		}

		if let Some(dist) = distances[start_index][node] {
			if node == end_index {
				result = Some(std::cmp::max(result.unwrap_or(0), dist));
			} else {
				visited[node] = true;
				let remaining_dist = dfs(distances, node, end_index, visited);
				visited[node] = false;

				if let Some(remaining_dist) = remaining_dist {
					result = Some(std::cmp::max(result.unwrap_or(0), remaining_dist + dist));
				}
			}
		}
	}

	return result;
}

fn sub_solve(grid: Vec<Vec<char>>, initial_pos: (usize, usize), end_pos: &(usize, usize), part2: bool) -> usize {
	let mut intersections = grid[1..grid.len()-1]
		.iter()
		.enumerate()
		.map(|(r, line)| {
			line[1..line.len()-1].iter()
				.enumerate()
				.filter_map(|(c, _)| if is_intersection(&grid, (r+1, c+1)) { Some((r+1, c+1)) } else { None })
				.collect_vec()
		})
		.flatten()
		.collect_vec();

	intersections.push(initial_pos);
	intersections.push(*end_pos);

	let mut distances = vec![vec![None; intersections.len()]; intersections.len()];
	for i in 0..intersections.len() {
		for j in 0..intersections.len() {
			if i == j {
				continue;
			}

			if let Some(dist) = bfs(&grid, intersections[i], intersections[j], part2) {
				distances[i][j] = Some(dist);
			}
		}
	}

	let mut visited = vec![false; intersections.len()];
	return dfs(&distances, intersections.len() - 2, intersections.len() - 1, &mut visited).unwrap();
}

pub fn solve(inputs: Vec<String>) {
	let mut grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let start_pos = grid[0].iter().enumerate().find_map(|(c, &ch)| if ch == '.' { Some((0, c)) } else { None }).unwrap();
	let end_pos = grid.last().unwrap().iter().enumerate().find_map(|(c, &ch)| if ch == '.' { Some((grid.len()-1, c)) } else { None }).unwrap();

	// Fill in the starting point and start our DFS from the next row to avoid having to deal with boundary conditions. Also, add 2 to the result to account for the starting and ending points
	grid[start_pos.0][start_pos.1] = '#';
	grid[end_pos.0][end_pos.1] = '#';

	println!("Part 1: {}", sub_solve(grid.clone(), (start_pos.0 + 1, start_pos.1), &(end_pos.0 - 1, end_pos.1), /*part2=*/false) + 2);
	println!("Part 2: {}", sub_solve(grid.clone(), (start_pos.0 + 1, start_pos.1), &(end_pos.0 - 1, end_pos.1), /*part2=*/true) + 2);
}