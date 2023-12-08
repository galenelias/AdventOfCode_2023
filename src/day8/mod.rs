use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
	left: String,
	right: String,
}

// Returns (steps_a_to_z, steps_z_to_z)
fn get_steps(
	node_map: &HashMap<&str, Node>,
	directions: &[char],
	start: &str,
	part2: bool,
) -> (usize, usize) {
	let mut dir_index = 0;
	let mut node = start;
	let mut steps = 0;

	let mut steps_a_to_z = None;
	let mut steps_z_to_z = None;

	loop {
		let direction = directions[dir_index];
		let next_node = match direction {
			'L' => &node_map.get(node).unwrap().left,
			'R' => &node_map.get(node).unwrap().right,
			_ => panic!("Unknown direction"),
		};

		node = next_node;
		dir_index = (dir_index + 1) % directions.len();
		steps += 1;

		if node == "ZZZ" || (part2 && node.ends_with("Z")) {
			if steps_a_to_z == None {
				steps_a_to_z = Some(steps);
			} else if steps_z_to_z == None {
				steps_z_to_z = Some(steps - steps_a_to_z.unwrap());
			} else {
				if steps - steps_z_to_z.unwrap() - steps_a_to_z.unwrap() != steps_a_to_z.unwrap() {
					panic!("Z -> Z interval is not consistent!");
				}
				return (steps_a_to_z.unwrap(), steps_z_to_z.unwrap());
			}
		}
	}
}

pub fn solve(inputs: Vec<String>) {
	let directions = &inputs[0].chars().collect_vec();

	let mut node_map = HashMap::new();
	for line in inputs[2..].iter() {
		let (node_str, left_right_str) = line.split_once(" = ").unwrap();
		let left_right_str = &left_right_str[1..left_right_str.len() - 1];
		let (left, right) = left_right_str.split_once(", ").unwrap();

		node_map.insert(
			node_str,
			Node {
				left: left.to_string(),
				right: right.to_string(),
			},
		);
	}

	println!(
		"Part 1: {}",
		get_steps(&node_map, directions, "AAA", /*part2=*/ false).0
	);

	let ghost_nodes = node_map
		.keys()
		.filter(|node| node.ends_with("A"))
		.copied()
		.collect_vec();

	let ghost_distances = ghost_nodes
		.iter()
		.map(|start| {
			get_steps(&node_map, directions, start, /*part2=*/ true)
		})
		.sorted_by_key(|distances| distances.1)
		.collect_vec();

	// Walk our steps at the interval of the ghost with the longest interval, checking if all
	// ghosts are at their finish node.
	let &(start_steps, interval) = ghost_distances.last().unwrap();
	for steps in (start_steps..).step_by(interval) {
		if ghost_distances
			.iter()
			.all(|(a_to_z, z_to_z)| ((steps - a_to_z) % z_to_z) == 0)
		{
			println!("Part 2: {}", steps);
			break;
		}
	}
}
