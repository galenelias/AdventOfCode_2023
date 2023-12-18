use itertools::Itertools;

fn sub_solve(inputs: &[(char, i64)]) -> i64 {
	let mut r: i64 = 0;
	let mut area: i64 = 0;
	let mut last_dir = inputs.last().unwrap().0;

	// Trace the contour of the shape, keeping track of the area via
	// the shoelace formula
	for (dir, dist) in inputs {
		let dr = match dir {
			'U' => -1,
			'D' => 1,
			'L' | 'R' => 0,
			_ => unreachable!(),
		};

		area += match (dir, last_dir) {
			('R', 'D') => -r * (dist-1),
			('R', 'U') => -r * dist,
			('D', 'L') => 0,
			('D', 'R') => -r,
			('L', 'D') => (r+1) * dist,
			('L', 'U') => (r+1) * (dist-1),
			('U', 'L') => r+1,
			('U', 'R') => 0,
			_ => unreachable!("Unhandled case: {} {}", dir, last_dir),
		};

		r += dr * dist;
		last_dir = *dir;
	}

	return area;
}
pub fn solve(inputs: Vec<String>) {

	let part1_inputs = inputs.iter().map(|line| {
		let parts = line.split_whitespace().collect_vec();
		let dir = parts[0].chars().next().unwrap();
		let dist = parts[1].parse::<i64>().unwrap();
		(dir, dist)
	}).collect_vec();

	let part2_inputs = inputs.iter().map(|line| {
		let parts = line.split_whitespace().collect_vec();
		let color = &parts[2][2..parts[2].len()-1];
		let dir = match color.chars().last().unwrap() {
			'0' => 'R',
			'1' => 'D',
			'2' => 'L',
			'3' => 'U',
			_ => unreachable!(),
		};
		
		let dist = i64::from_str_radix(&color[..color.len()-1], 16).unwrap();
		(dir, dist)
	}).collect_vec();

	println!("Part 1: {}", sub_solve(&part1_inputs));
	println!("Part 2: {}", sub_solve(&part2_inputs));
}
