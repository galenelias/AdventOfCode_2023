use itertools::Itertools;

fn dfs(mut grid: Vec<Vec<char>>, initial_pos: (usize, usize), initial_steps: usize, end_pos: &(usize, usize), part2: bool) -> usize {
	// Can't use recursion since we'll blow out our stack, so use manual stack instead
	let mut stack = Vec::new();
	let mut history: Vec<((usize, usize), char)> = Vec::new();
	let mut last_steps = 0;

	stack.push((initial_pos, initial_steps));

	let mut max_answer = 0;

	while !stack.is_empty() {
		let (pos, steps) = stack.pop().unwrap();

		if &pos == end_pos {
			if steps > max_answer {
				println!("Possible solution: {}", steps);
				max_answer = steps;
			}
			continue;
		}

		while steps <= last_steps {
			last_steps -= 1;
			let (restore_pos, restore_ch) = history.pop().unwrap();
			grid[restore_pos.0][restore_pos.1] = restore_ch;
		}

		let prev_ch = grid[pos.0][pos.1];
		history.push((pos, prev_ch));
		grid[pos.0][pos.1] = '#';
		last_steps = steps;

		let mut try_pos = |new_pos: (usize, usize)| {
			if grid[new_pos.0][new_pos.1] != '#' {
				stack.push((new_pos, steps + 1));
			}
		};

		if prev_ch == '.' || (part2 && prev_ch != '#') {
			try_pos((pos.0 - 1, pos.1));
			try_pos((pos.0 + 1, pos.1));
			try_pos((pos.0, pos.1 - 1));
			try_pos((pos.0, pos.1 + 1));
		} else if prev_ch == '^' {
			try_pos((pos.0 - 1, pos.1));
		} else if prev_ch == 'v' {
			try_pos((pos.0 + 1, pos.1));
		} else if prev_ch == '<' {
			try_pos((pos.0, pos.1 - 1));
		} else if prev_ch == '>' {
			try_pos((pos.0, pos.1 + 1));
		}
	}

	return max_answer;
}

pub fn solve(inputs: Vec<String>) {
	let mut grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let start_pos = grid[0].iter().enumerate().find_map(|(c, &ch)| if ch == '.' { Some((0, c)) } else { None }).unwrap();
	let end_pos = grid.last().unwrap().iter().enumerate().find_map(|(c, &ch)| if ch == '.' { Some((grid.len()-1, c)) } else { None }).unwrap();

	// Fill in the starting point and start our DFS from the next row to avoid having to deal with boundary conditions
	grid[start_pos.0][start_pos.1] = '#';

	println!("Part 1: {}", dfs(grid.clone(), (start_pos.0 + 1, start_pos.1), 1, &end_pos, /*part2=*/false));
	println!("Part 2: {}", dfs(grid.clone(), (start_pos.0 + 1, start_pos.1), 1, &end_pos, /*part2=*/true));
}