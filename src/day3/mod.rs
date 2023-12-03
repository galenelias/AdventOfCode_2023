use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum NodeType {
	Number(u64),
	Symbol(char),
}

#[derive(Debug)]
struct Node {
	node_type: NodeType,
	r: usize,
	c: usize,
	width: usize,
	adjacent_nodes: Vec<usize>,
}

impl Node {
	fn is_other_node_type(&self, other: &Node) -> bool {
		match (&self.node_type, &other.node_type) {
			(NodeType::Number(_), NodeType::Symbol(_)) => true,
			(NodeType::Symbol(_), NodeType::Number(_)) => true,
			_ => false,
		}
	}

	fn is_adjacent(&self, other: &Node) -> bool {
		let dr = if self.r >= other.r {
			self.r - other.r
		} else {
			other.r - self.r
		};

		let dc = if other.c >= self.c + self.width - 1 {
			other.c - (self.c + self.width - 1)
		} else if self.c >= other.c + other.width - 1 {
			self.c - (other.c + other.width - 1)
		} else {
			0
		};

		return dr <= 1 && dc <= 1;
	}
}

pub fn solve(inputs: Vec<String>) {
	let grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let mut nodes = Vec::new();

	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			if grid[r][c].is_ascii_digit() {
				// Check for start of word
				if c == 0 || !grid[r][c - 1].is_ascii_digit() {
					let number_str = grid[r][c..]
						.iter()
						.take_while(|c| c.is_ascii_digit())
						.collect::<String>();

					let number = number_str.parse::<u64>().unwrap();

					nodes.push(Node {
						r,
						c,
						node_type: NodeType::Number(number),
						width: number_str.len(),
						adjacent_nodes: Vec::new(),
					});
				}
			} else if grid[r][c] == '.' {
				continue;
			} else {
				nodes.push(Node {
					r,
					c,
					node_type: NodeType::Symbol(grid[r][c]),
					width: 1,
					adjacent_nodes: Vec::new(),
				});
			}
		}
	}

	// Calculate node adjacency
	let nodes_len = nodes.len();
	for i in 0..nodes_len {
		for j in 0..nodes_len {
			if i == j {
				continue;
			}

			if nodes[i].is_other_node_type(&nodes[j]) && nodes[i].is_adjacent(&nodes[j]) {
				nodes[i].adjacent_nodes.push(j);
			}
		}
	}

	let part1 = nodes
		.iter()
		.filter(|node| !node.adjacent_nodes.is_empty())
		.filter_map(|node| match node.node_type {
			NodeType::Number(value) => Some(value),
			NodeType::Symbol(_) => None,
		})
		.sum::<u64>();
	println!("Part 1: {}", part1);

	let part2 = nodes
		.iter()
		.filter(|node| node.node_type == NodeType::Symbol('*') && node.adjacent_nodes.len() == 2)
		.filter_map(|node| {
			match (
				&nodes[node.adjacent_nodes[0]].node_type,
				&nodes[node.adjacent_nodes[1]].node_type,
			) {
				(NodeType::Number(num1), NodeType::Number(num2)) => Some(num1 * num2),
				_ => None,
			}
		})
		.sum::<u64>();
	println!("Part 2: {}", part2);
}
