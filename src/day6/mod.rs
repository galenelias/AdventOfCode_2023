use itertools::Itertools;

fn ways_to_solve(time: u64, record_distance: u64) -> u64 {
	(1..time)
		.map(|hold_time| (time - hold_time) * hold_time)
		.filter(|&distance| distance > record_distance)
		.count() as u64
}

pub fn solve(inputs: Vec<String>) {
	let times_str = inputs[0].split_once(":").unwrap().1;
	let distances_str = inputs[1].split_once(":").unwrap().1;

	let times = times_str
		.split_whitespace()
		.map(|s| s.parse::<u64>().unwrap())
		.collect_vec();
	let distances = distances_str
		.split_whitespace()
		.map(|s| s.parse::<u64>().unwrap())
		.collect_vec();

	let part1 = times.iter().zip(distances.iter())
		.map(|(&dist, &time)| ways_to_solve(dist, time))
		.product::<u64>();
	println!("Part 1: {}", part1);

	let part2_time = times_str
		.chars()
		.filter(|c| c.is_ascii_digit())
		.collect::<String>()
		.parse::<u64>()
		.unwrap();
	let part2_distance = distances_str
		.chars()
		.filter(|c| c.is_ascii_digit())
		.collect::<String>()
		.parse::<u64>()
		.unwrap();
	println!("Part 2: {}", ways_to_solve(part2_time, part2_distance));
}
