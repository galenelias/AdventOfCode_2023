use itertools::Itertools;

#[derive(Debug)]
struct Hailstone {
	pos: (f64, f64, f64),
	vel: (f64, f64, f64),
}

fn get_intersection(hs1: &Hailstone, hs2: &Hailstone) -> Option<(f64, f64)> {
	let dx = hs2.pos.0 - hs1.pos.0;
	let dy = hs2.pos.1 - hs1.pos.1;
	let det = hs2.vel.0 * hs1.vel.1 - hs2.vel.1 * hs1.vel.0;

	if det == 0.0 {
		return None;
	}

	let t1 = (dy * hs2.vel.0 - dx * hs2.vel.1) / det;
	let t2 = (dy * hs1.vel.0 - dx * hs1.vel.1) / det;

	if t1 < 0.0 || t2 < 0.0 {
		return None;
	}

	let px = hs1.pos.0 + t1 * hs1.vel.0;
	let py = hs1.pos.1 + t1 * hs1.vel.1;

	return Some((px, py));
}

pub fn solve(inputs: Vec<String>) {

	let hailstones = inputs
		.iter()
		.map(|line| {
			let (pos_str, vel_str) = line.split_once(" @ ").unwrap();
			let pos = pos_str.split(", ").map(|s| s.trim().parse::<f64>().unwrap()).collect_vec();
			let vel = vel_str.split(", ").map(|s| s.trim().parse::<f64>().unwrap()).collect_vec();

			Hailstone {
				pos: (pos[0], pos[1], pos[2]),
				vel: (vel[0], vel[1], vel[2]),
			}
		})
		.collect_vec();

	let mut part1 = 0;
	for i in 0..hailstones.len() {
		for j in i..hailstones.len() {
			if i == j {
				continue;
			}

			let hs1 = &hailstones[i];
			let hs2 = &hailstones[j];

			if let Some((px, py)) = get_intersection(hs1, hs2) {
				if px >= 200000000000000f64 && px <= 400000000000000f64 && py >= 200000000000000f64 && py <= 400000000000000f64 {
					part1 += 1;
				}
			} else {
			}
		}
	}

	println!("Part 1: {}", part1);

	println!("Part 2 - equations to be plugged into Z3:");
	for (i, hs) in hailstones.iter().enumerate().take(3) {
		println!("{} + {} * t{} == px + vx * t{}", hs.pos.0, hs.vel.0, i+1, i+1);
		println!("{} + {} * t{} == py + vy * t{}", hs.pos.1, hs.vel.1, i+1, i+1);
		println!("{} + {} * t{} == pz + vz * t{}", hs.pos.2, hs.vel.2, i+1, i+1);
	}
}