use itertools::Itertools;

fn get_adjacents(pos: &(usize, usize)) -> [(usize, usize); 4] {
	[
		(pos.0 - 1, pos.1),
		(pos.0 + 1, pos.1),
		(pos.0, pos.1 - 1),
		(pos.0, pos.1 + 1),
	]
}

fn get_pipe_ends(pipe: &char, pos: &(usize, usize)) -> [(usize, usize); 2] {
	match pipe {
		'|' => { [(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)] }
		'-' => { [(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)] }
		'L' => { [(pos.0 - 1, pos.1), (pos.0, pos.1 + 1)] }
		'J' => { [(pos.0 - 1, pos.1), (pos.0, pos.1 - 1)] }
		'7' => { [(pos.0 + 1, pos.1), (pos.0, pos.1 - 1)] }
		'F' => { [(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)] }
		_ => panic!("Unknown pipe type: {}", pipe),
	}
}

fn get_next_pipe(grid: &Vec<Vec<char>>, pos: &(usize, usize), prev_pos: (usize, usize)) -> (usize, usize) {
	let ends = get_pipe_ends(&grid[pos.0][pos.1], pos);

	if ends[0] == prev_pos {
		ends[1]
	} else {
		ends[0]
	}
}

pub fn solve(inputs: Vec<String>) {
	let mut grid = inputs.iter().map(|line| line.chars().collect_vec()).collect_vec();

	// Pad grid with empty dots so we don't have to deal with boundary conditions
	for line in grid.iter_mut() {
		line.insert(0, '.');
		line.push('.');
	}
	grid.insert(0, vec!['.'; grid[0].len()]);
	grid.push(vec!['.'; grid[0].len()]);

	let start_pos = grid.iter().enumerate().find_map(|(r, line)| {
		line.iter().enumerate().find_map(|(c, &ch)| {
			if ch == 'S' {
				Some((r, c))
			} else {
				None
			}
		})
	}).unwrap();

	println!("Start pos: {:?}", start_pos);

	let start_pipe_neighbors = get_adjacents(&start_pos).into_iter().filter(|p| {
		let ch = grid[p.0][p.1];
		ch != '.' && get_pipe_ends(&ch, &p).iter().any(|&p| p == start_pos)
	}).collect_vec();

	let start_pipe_piece = ['|', '-', 'L', 'J', '7', 'F'].iter().find(|&ch| {
		let pipe_ends = get_pipe_ends(ch, &start_pos);
		return (pipe_ends[0] == start_pipe_neighbors[0] && pipe_ends[1] == start_pipe_neighbors[1])
		|| (pipe_ends[1] == start_pipe_neighbors[0] && pipe_ends[0] == start_pipe_neighbors[1]);
	}).unwrap();

	println!("Start pipe piece: {}", start_pipe_piece);
	grid[start_pos.0][start_pos.1] = *start_pipe_piece;

	let mut area: i64 = 0;

	let mut steps = 1;
	let mut prev_pos = start_pos;
	let mut pos = start_pipe_neighbors[1].clone();

	println!("Initial neighbors: {:?}", start_pipe_neighbors);

	loop {
		let ch = grid[pos.0][pos.1];
		if pos.1 == prev_pos.1 - 1 { // moving left
			if ch != 'F' {
				area += pos.0 as i64 + 1;
			}
		} else if pos.1 == prev_pos.1 + 1 { // moving right
			if ch != 'J' {
				area -= pos.0 as i64;
			}  // '7' OK
		} else if grid[pos.0][pos.1] != '|' {
			if pos.0 == prev_pos.0 - 1 { // moving up
				match ch {
					'7' => { /* turning left, no-op */ },
					'F' => { // turning right
						area -= pos.0 as i64;
					},
					_ => { panic!("Unknown pipe type: {}", ch); }
				}
			} else if pos.0 == prev_pos.0 + 1 { // Moving down
				match ch {
					'J' => { // turning left
						area += pos.0 as i64 + 1;
					},
					'L' => { /* turning right, no-op */ },
					_ => { panic!("Unknown pipe type: {}", ch); }
				}
			}
		}

		if pos == start_pos {
			break;
		}

		let next_pos = get_next_pipe(&grid, &pos, prev_pos);
		steps += 1;
		prev_pos = pos;
		pos = next_pos;
	}

	println!("Part 1: {}", steps / 2);
	println!("Part 2: {}", area - steps);
}
