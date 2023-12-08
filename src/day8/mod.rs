use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
	left: String,
	right: String,
}

pub fn solve(inputs: Vec<String>) {

	let mut node_map = HashMap::new();

	let directions = &inputs[0].chars().collect_vec();

	for line in inputs[2..].iter() {
		let (node_str, left_right_str) = line.split_once(" = ").unwrap();
		let left_right_str = &left_right_str[1..left_right_str.len() - 1];
		let (left, right) = left_right_str.split_once(", ").unwrap();

		node_map.insert(node_str, Node { left: left.to_string(), right: right.to_string() });
	}

	let mut dir_index = 0;
	let mut node = "AAA";
	let mut steps = 0;
	while node != "ZZZ" {
		let direction = directions[dir_index];
		let next_node = match direction {
			'L' => &node_map.get(node).unwrap().left,
			'R' => &node_map.get(node).unwrap().right,
			_ => panic!("Unknown direction"),
		};

		node = next_node;
		dir_index = (dir_index + 1) % directions.len();
		steps += 1;
	}

	println!("Part 1: {}", steps);

	let mut ghost_nodes = node_map.keys().filter(|node| node.ends_with("A")).copied().collect_vec();
	let mut dir_index = 0;
	let mut steps: u64 = 0;
	let mut last_z = vec![None; ghost_nodes.len()];
	let mut intervals = vec![None; ghost_nodes.len()];

	while !ghost_nodes.iter().all(|node| node.ends_with("Z")) {
		let direction = directions[dir_index];
		dir_index = (dir_index + 1) % directions.len();
		steps += 1;
		
		for i in 0..ghost_nodes.len() {
			let next_node = match direction {
				'L' => &node_map.get(ghost_nodes[i]).unwrap().left,
				'R' => &node_map.get(ghost_nodes[i]).unwrap().right,
				_ => panic!("Unknown direction"),
			};
			ghost_nodes[i] = next_node;

			if next_node.ends_with("Z") {
				if last_z[i] != None {
					let interval = steps - last_z[i].unwrap();
					if intervals[i] == None {
						println!("{}: Found Z at interval {} (steps = {})", i, interval, steps);
						intervals[i] = Some(interval);

						if intervals.iter().all(|interval| interval != &None) {
							let mut sim_steps = steps;

							// Walk by largest interval until all ghosts are at their finish node.
							// Could be done with math, but that sounds harder...
							loop {
								if (0..ghost_nodes.len()).all(|i| {
									let interval = intervals[i].unwrap();
									(sim_steps - last_z[i].unwrap()) % interval == 0
								}) {
									println!("Part 2: {}", sim_steps);
									return;
								}

								sim_steps += interval;
							}
						}

					} else if intervals[i] != Some(interval) {
						panic!("Intervals don't match: {}: {} vs {}", i, intervals[i].unwrap(), interval);
					}
				}
				last_z[i] = Some(steps);
			}
		}
	}

	println!("Part 2: {}", steps);
}