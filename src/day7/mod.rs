use itertools::Itertools;
use std::cmp;

struct Entry {
	hand: Vec<char>,
	bid: u64,
}

#[derive(PartialEq)]
enum Part {
	Part1,
	Part2,
}

fn card_to_index(card: &char, part: &Part) -> usize {
	let cards = if part == &Part::Part1 {
		[
			'2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
		]
	} else {
		[
			'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
		]
	};
	cards.iter().position(|c| c == card).unwrap()
}

fn build_frequencies(hand: &[char], part: &Part) -> Vec<usize> {
	let mut freqs = [0; 13];
	for card in hand {
		freqs[card_to_index(card, part)] += 1;
	}

	if part == &Part::Part1 {
		return freqs.into_iter().sorted().rev().collect_vec();
	} else {
		let jokers = freqs[card_to_index(&'J', part)];
		freqs[card_to_index(&'J', part)] = 0;

		let mut sorted_freqs = freqs.into_iter().sorted().rev().collect_vec();
		sorted_freqs[0] += jokers;
		return sorted_freqs;
	}
}

fn compare_hands(hand1: &[char], hand2: &[char], part: &Part) -> std::cmp::Ordering {
	let freqs1 = build_frequencies(hand1, part);
	let freqs2 = build_frequencies(hand2, part);

	let freq_cmp = freqs1.cmp(&freqs2);
	if freq_cmp != std::cmp::Ordering::Equal {
		return freq_cmp;
	}

	for (c1, c2) in hand1.iter().zip(hand2.iter()) {
		if c1 != c2 {
			return card_to_index(c1, part).cmp(&card_to_index(c2, part));
		}
	}

	return cmp::Ordering::Equal;
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

	for part in &[Part::Part1, Part::Part2] {
		let entries_sorted = entries
			.iter()
			.sorted_by(|e1, e2| compare_hands(&e1.hand, &e2.hand, part))
			.collect_vec();

		let winnings = entries_sorted
			.iter()
			.enumerate()
			.map(|(i, entry)| entry.bid * (i as u64 + 1))
			.sum::<u64>();

		println!(
			"Part {}: {}",
			if part == &Part::Part1 { 1 } else { 2 },
			winnings
		);
	}
}
