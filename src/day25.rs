use itertools::Itertools;
use std::collections::HashSet;

// Stoer-Wagner min cut algorithm
// Based on https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm#Example_code
// Input: mat - Adjacency matrix of graph
fn min_cut(mut mat: Vec<Vec<i32>>) -> (i32, Vec<usize>) {
	let n = mat.len();
	let mut best = (i32::MAX, Vec::new());
	let mut co = vec![Vec::new(); n];

	for i in 0..n {
		co[i].push(i);
	}

	for ph in 1..n {
		let mut w = mat[0].clone();
		let mut t = 0;
		let mut s = 0;
		for _it in 0..n - ph {
			w[t] = i32::MIN;
			s = t;
			t = w.iter().position_max().unwrap();

			for i in 0..n {
				w[i] += mat[t][i];
			}
		}

		best = std::cmp::min(best, (w[t] - mat[t][t], co[t].clone()));

		let co_t = co[t].clone();
		co[s].extend(co_t);
		for i in 0..n {
			mat[s][i] += mat[t][i];
		}

		for i in 0..n {
			mat[i][s] = mat[s][i];
		}

		mat[0][t] = i32::MIN;
	}

	return best;
}

pub fn solve(inputs: Vec<String>) {
	let mut all_connections = Vec::new();
	let mut nodes = HashSet::new();

	for line in &inputs {
		let (source, dests) = line.split_once(": ").unwrap();
		nodes.insert(source);
		for dest in dests.split_whitespace() {
			nodes.insert(dest);
			all_connections.push((source, dest));
		}
	}

	let nodes = nodes.into_iter().collect_vec();
	let mut mat = vec![vec![0i32; nodes.len()]; nodes.len()];

	for conn in &all_connections {
		let source_index = nodes.iter().position(|&x| x == conn.0).unwrap();
		let dest_index = nodes.iter().position(|&x| x == conn.1).unwrap();
		mat[source_index][dest_index] = 1;
		mat[dest_index][source_index] = 1;
	}

	let (_min_cut, min_cut_nodes) = min_cut(mat);
	println!(
		"Part 1: {}",
		min_cut_nodes.len() * (nodes.len() - min_cut_nodes.len())
	);
}
