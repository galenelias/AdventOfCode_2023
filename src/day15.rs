use itertools::Itertools;

fn hash(s: &str) -> u8 {
	s.chars().fold(0u8, |hash, ch| {
		hash.wrapping_add(ch as u8).wrapping_mul(17)
	})
}

pub fn solve(inputs: Vec<String>) {
	let parts = inputs[0].split(',').collect_vec();

	let part1 = parts.iter().map(|line| hash(line) as u64).sum::<u64>();
	println!("Part 1: {}", part1);

	let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];

	for instruction in parts {
		if instruction.contains('=') {
			let (id, lense) = instruction.split_once('=').unwrap();
			let lense = lense.parse::<usize>().unwrap();
			let box_num = hash(id);

			let dest_box = &mut boxes[box_num as usize];
			if let Some(i) = dest_box.iter().position(|item| item.0 == id) {
				dest_box[i].1 = lense;
			} else {
				dest_box.push((id, lense));
			}
		} else {
			let (id, _) = instruction.split_once('-').unwrap();

			let box_num = hash(id);
			let dest_box = &mut boxes[box_num as usize];
			if let Some(i) = dest_box.iter().position(|item| item.0 == id) {
				dest_box.remove(i);
			}
		}
	}

	let mut part2 = 0;
	for i in 0..256 {
		for b in 0..boxes[i].len() {
			part2 += (i + 1) * (1 + b) * boxes[i][b].1;
		}
	}

	println!("Part 2: {}", part2);
}
