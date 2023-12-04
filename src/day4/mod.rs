use itertools::Itertools;
use std::collections::HashSet;

pub fn solve(inputs: Vec<String>) {
	let match_counts = inputs
		.iter()
		.map(|line| {
			let (_, numbers_str) = line.split_once(": ").unwrap();
			let (winning_numbers_str, my_numbers_str) = numbers_str.split_once(" | ").unwrap();

			let winning_numbers = winning_numbers_str
				.split_whitespace()
				.map(|n| n.parse::<u32>().unwrap())
				.collect::<HashSet<u32>>();
			let my_numbers = my_numbers_str
				.split_whitespace()
				.map(|n| n.parse::<u32>().unwrap())
				.collect::<HashSet<u32>>();

			return winning_numbers.intersection(&my_numbers).count();
		})
		.collect_vec();

	let part1 = match_counts
		.iter()
		.map(|matches| {
			if matches == &0 {
				0
			} else {
				1 << matches - 1
			}
		})
		.sum::<u32>();
	println!("Part 1: {}", part1);

	let mut card_counts = vec![1; match_counts.len()];

	for i in 0..match_counts.len() {
		for j in 1..=match_counts[i] {
			if i + j < match_counts.len() {
				card_counts[i + j] += card_counts[i];
			}
		}
	}
	println!("Part 2: {}", card_counts.iter().sum::<u32>());
}
