use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Brick {
	pt1: [usize; 3],
	pt2: [usize; 3],
}

fn bricks_overlap_xy(b1: &Brick, b2: &Brick) -> bool {
	let x_overlap = b1.pt2[0] >= b2.pt1[0] && b1.pt1[0] <= b2.pt2[0];
	let y_overlap = b1.pt2[1] >= b2.pt1[1] && b1.pt1[1] <= b2.pt2[1];
	x_overlap && y_overlap
}

fn drop_bricks(bricks: &mut Vec<Brick>) -> usize {
	let mut dropped_count = 0;

	for i in 0..bricks.len() {
		let mut z_below = 0; // Ground

		// Make a copy of the brick so we can modify it without borrowing issues
		let mut brick = bricks[i].clone();
		for j in 0..i {
			let other_brick = &bricks[j];
			if bricks_overlap_xy(&brick, other_brick) {
				z_below = std::cmp::max(z_below, other_brick.pt2[2]);
			}
		}

		let z_drop = brick.pt1[2] - z_below - 1;
		if z_drop > 0 {
			brick.pt1[2] -= z_drop;
			brick.pt2[2] -= z_drop;

			bricks[i] = brick;
			dropped_count += 1;
		}
	}
	return dropped_count;
}

pub fn solve(inputs: Vec<String>) {
	let mut bricks = inputs
		.iter()
		.map(|line| {
			let (pt1, pt2) = line.split_once("~").unwrap();
			let pt1 = pt1
				.split(",")
				.map(|s| s.parse::<usize>().unwrap())
				.collect_vec();
			let pt2 = pt2
				.split(",")
				.map(|s| s.parse::<usize>().unwrap())
				.collect_vec();
			Brick {
				pt1: [pt1[0], pt1[1], pt1[2]],
				pt2: [pt2[0], pt2[1], pt2[2]],
			}
		})
		.collect_vec();

	loop {
		bricks.sort_by_key(|brick| brick.pt1[2]); // Sort by z

		if drop_bricks(&mut bricks) == 0 {
			break;
		}
	}

	let bricks = bricks; // Make immutable
	let mut part1 = 0;
	let mut part2 = 0;
	for i in 0..bricks.len() {
		let mut tmp_bricks = bricks.clone();
		tmp_bricks.remove(i);
		let bricks_fall = drop_bricks(&mut tmp_bricks);
		if bricks_fall == 0 {
			part1 += 1;
		}
		part2 += bricks_fall;
	}

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
