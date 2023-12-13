use itertools::Itertools;
use std::collections::HashMap;

// (report.len(), remaining_regions.len()) -> number of possible arrangements
type Memo = HashMap<(usize, usize), usize>;

fn possible_arrangements(report: &[char], remaining_regions: &[usize], memo: &mut Memo) -> usize {
	if let Some(memo_result) = memo.get(&(report.len(), remaining_regions.len())) {
		return *memo_result;
	}

	// No remaining damaged regions.  Valid config if rest of report can be working springs
	if remaining_regions.is_empty() {
		return report.iter().all(|&ch| ch == '.' || ch == '?') as usize;
	}

	// Can't fulfill remaining regions with the remaining report
	if report.is_empty() && !remaining_regions.is_empty() {
		return 0;
	}

	if report[0] == '.' {
		return possible_arrangements(&report[1..], remaining_regions, memo);
	}

	let mut result = 0;
	if report[0] == '?' {
		result += possible_arrangements(&report[1..], remaining_regions, memo);
	}

	// Now assume we're fulfilling the next region starting from here
	if report.len() >= remaining_regions[0]
		&& report[0..remaining_regions[0]]
			.iter()
			.all(|&ch| ch == '#' || ch == '?')
		&& (report.len() == remaining_regions[0] || report[remaining_regions[0]] != '#')
	{
		result += possible_arrangements(
			&report[remaining_regions[0] + 1..],
			&remaining_regions[1..],
			memo,
		);
	}

	memo.insert((report.len(), remaining_regions.len()), result);
	return result;
}

pub fn solve(inputs: Vec<String>) {
	let springs_and_regions = inputs
		.iter()
		.map(|line| {
			let (report, damaged_regions) = line.split_once(" ").unwrap();
			let damaged_regions = damaged_regions
				.split(",")
				.map(|s| s.parse::<usize>().unwrap())
				.collect_vec();

			(report, damaged_regions)
		})
		.collect_vec();

	let part1 = springs_and_regions
		.iter()
		.map(|(springs, regions)| {
			let mut springs = springs.chars().collect_vec();
			springs.push('.'); // Add a sentinel to the end of the report to make boundary conditions easier

			let mut memo = Memo::new();
			return possible_arrangements(&springs, &regions, &mut memo);
		})
		.sum::<usize>();

	let part2 = springs_and_regions
		.iter()
		.map(|(springs, regions)| {
			let mut unfolded_springs = vec![*springs; 5].join("?");
			unfolded_springs.push('.'); // Add a sentinel to the end of the report to make boundary conditions easier
			let unfolded_springs = unfolded_springs.chars().collect_vec();
			let regions_x5 = regions.repeat(5);

			let mut memo = Memo::new();
			return possible_arrangements(&unfolded_springs, &regions_x5, &mut memo);
		})
		.sum::<usize>();

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
