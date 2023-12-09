use itertools::Itertools;
use std::collections::VecDeque;

fn extrapolate_sequence(sequence: &VecDeque<i64>, part2: bool) -> i64 {
	let mut derivatives = Vec::new();
	derivatives.push(sequence.clone());

	while !derivatives.last().unwrap().iter().all(|&d| d == 0) {
		let last = derivatives.last().unwrap();
		let mut next = VecDeque::new();
		for i in 0..last.len() - 1 {
			next.push_back(last[i+1] - last[i]);
		}
		derivatives.push(next);
	}

	if !part2 {
		derivatives.last_mut().unwrap().push_back(0);
		for i in (0..derivatives.len()-1).rev() {
			let new_value = derivatives[i].back().unwrap() + derivatives[i+1].back().unwrap();
			derivatives[i].push_back(new_value);
		}

		derivatives[0].back().unwrap().clone()
	} else {
		derivatives.last_mut().unwrap().push_front(0);
		for i in (0..derivatives.len()-1).rev() {
			let new_value = derivatives[i].front().unwrap() - derivatives[i+1].front().unwrap();
			derivatives[i].push_front(new_value);
		}
	
		derivatives[0].front().unwrap().clone()
	}
}

pub fn solve(inputs: Vec<String>) {
	let sequences = inputs.iter().map(|line| line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<VecDeque<_>>()).collect_vec();

	let part1 = sequences.iter().map(|sequence| {
		extrapolate_sequence(sequence, /*part2=*/false)
	}).sum::<i64>();

	println!("Part 1: {}", part1);

	let part2 = sequences.iter().map(|sequence| {
		extrapolate_sequence(sequence, /*part2=*/true)
	}).sum::<i64>();

	println!("Part 2: {}", part2);
}