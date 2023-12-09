use itertools::Itertools;

fn extrapolate_sequence(sequence: Vec<i64>) -> i64 {
	let mut derivatives = vec![sequence];

	while !derivatives.last().unwrap().iter().all(|&d| d == 0) {
		let last = derivatives.last().unwrap();
		let mut next = Vec::new();
		for i in 0..last.len() - 1 {
			next.push(last[i+1] - last[i]);
		}
		derivatives.push(next);
	}

	derivatives.last_mut().unwrap().push(0);
	for i in (0..derivatives.len()-1).rev() {
		let new_value = derivatives[i].last().unwrap() + derivatives[i+1].last().unwrap();
		derivatives[i].push(new_value);
	}

	derivatives[0].last().unwrap().clone()
}

pub fn solve(inputs: Vec<String>) {
	let sequences = inputs.iter().map(|line| line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect_vec()).collect_vec();

	let part1 = sequences.iter().map(|sequence| {
		extrapolate_sequence(sequence.clone())
	}).sum::<i64>();

	let part2 = sequences.iter().map(|sequence| {
		extrapolate_sequence(sequence.iter().cloned().rev().collect_vec())
	}).sum::<i64>();

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}