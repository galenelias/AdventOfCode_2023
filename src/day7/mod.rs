use std::cmp;
use itertools::Itertools;

struct Entry {
	hand: Vec<char>,
	bid: u64,
}

fn card_to_index(card: &char) -> usize {
	let cards = [
		'2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
	];
	cards.iter().position(|c| c == card).unwrap()
}

fn card_to_index_part2(card: &char) -> usize {
	let cards = [
		'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
	];
	cards.iter().position(|c| c == card).unwrap()
}

fn build_frequencies(hand: &[char]) -> [u32; 13] {
	let mut freqs = [0; 13];
	for card in hand {
		freqs[card_to_index(card)] += 1;
	}
	freqs
}

fn build_frequencies_part2(hand: &[char]) -> Vec<usize> {
	let mut freqs = [0; 13];
	let jokers = hand.iter().filter(|&c| c == &'J').count();
	for card in hand.iter().filter(|&c| c != &'J') {
		freqs[card_to_index(card)] += 1;
	}

	let mut sorted_freqs = freqs.into_iter().sorted().rev().collect_vec();
	sorted_freqs[0] += jokers;
	return sorted_freqs;
}

fn compare_hands_part1(hand1: &[char], hand2: &[char]) -> std::cmp::Ordering {
	let freqs1 = build_frequencies(hand1);
	let freqs2 = build_frequencies(hand2);

	let sorted_freqs1 = freqs1.iter().sorted();
	let sorted_freqs2 = freqs2.iter().sorted();

	let freq_cmp = sorted_freqs1.cmp(sorted_freqs2);
	if freq_cmp != std::cmp::Ordering::Equal {
		return freq_cmp.reverse();
	}

	for (c1, c2) in hand1.iter().zip(hand2.iter()) {
		if c1 != c2 {
			return card_to_index(c1).cmp(&card_to_index(c2));
		}
	}

	cmp::Ordering::Equal
}

fn compare_hands_part2(hand1: &[char], hand2: &[char]) -> std::cmp::Ordering {
	let freqs1 = build_frequencies_part2(hand1);
	let freqs2 = build_frequencies_part2(hand2);

	let sorted_freqs1 = freqs1.iter().sorted();
	let sorted_freqs2 = freqs2.iter().sorted();

	let freq_cmp = sorted_freqs1.cmp(sorted_freqs2);
	if freq_cmp != std::cmp::Ordering::Equal {
		return freq_cmp.reverse();
	}

	for (c1, c2) in hand1.iter().zip(hand2.iter()) {
		if c1 != c2 {
			return card_to_index_part2(c1).cmp(&card_to_index_part2(c2));
		}
	}

	cmp::Ordering::Equal
}

pub fn solve(inputs: Vec<String>) {
	let entries = inputs
		.iter()
		.map(|line| {
			let (hand, bid) = line.split_once(" ").unwrap();
			Entry {
				hand: hand.chars().collect_vec(),
				bid: bid.parse::<u64>().unwrap(),
			}
		})
		.collect_vec();

	// -- Part 1 --
	let entries_sorted_part1 = entries
		.iter()
		.sorted_by(|e1, e2| compare_hands_part1(&e1.hand, &e2.hand))
		.collect_vec();

	let part1 = entries_sorted_part1
		.iter()
		.enumerate()
		.map(|(i, entry)| entry.bid * (i as u64 + 1))
		.sum::<u64>();

	println!("Part 1: {}", part1);

	// -- Part 2 --
	let entries_sorted_part2 = entries
		.iter()
		.sorted_by(|e1, e2| compare_hands_part2(&e1.hand, &e2.hand))
		.collect_vec();

	let part2 = entries_sorted_part2
		.iter()
		.enumerate()
		.map(|(i, entry)| entry.bid * (i as u64 + 1))
		.sum::<u64>();

	println!("Part 2: {}", part2);
}
