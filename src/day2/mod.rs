use itertools::Itertools;

struct Pull {
	green: u32,
	red: u32,
	blue: u32,
}

struct Game {
	id: u32,
	pulls: Vec<Pull>,
}

pub fn solve(inputs: Vec<String>) {
	let games = inputs
		.iter()
		.map(|line| {
			let (game_str, pulls_str) = line.split_once(": ").unwrap();
			let game_id = game_str.split_once(" ").unwrap().1.parse::<u32>().unwrap();

			let pulls = pulls_str
				.split("; ")
				.map(|pull| {
					let cubes = pull.split(", ").collect_vec();

					let mut pull = Pull {
						green: 0,
						red: 0,
						blue: 0,
					};

					for cube in cubes {
						let (amount_str, color_str) = cube.split_once(" ").unwrap();
						let amount = amount_str.parse::<u32>().unwrap();

						match color_str {
							"green" => pull.green = amount,
							"red" => pull.red = amount,
							"blue" => pull.blue = amount,
							_ => panic!("Unknown color"),
						}
					}

					pull
				})
				.collect_vec();

			Game { id: game_id, pulls }
		})
		.collect_vec();

	let part1 = games
		.iter()
		.filter(|game| {
			game.pulls
				.iter()
				.all(|pull| pull.red <= 12 && pull.green <= 13 && pull.blue <= 14)
		})
		.map(|game| game.id)
		.sum::<u32>();

	println!("Part 1: {}", part1);

	let powers = games
		.iter()
		.map(|game| {
			let max_red = game.pulls.iter().map(|pull| pull.red).max().unwrap();
			let max_green = game.pulls.iter().map(|pull| pull.green).max().unwrap();
			let max_blue = game.pulls.iter().map(|pull| pull.blue).max().unwrap();

			max_red * max_green * max_blue
		})
		.collect_vec();

	println!("Part 2: {}", powers.iter().sum::<u32>());
}
