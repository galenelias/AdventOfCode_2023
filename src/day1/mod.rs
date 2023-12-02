use itertools::Itertools;

fn str_to_digit(s: &str, include_strings: bool) -> Option<u32> {
	let number_strings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

	if let Some(digit) = s.chars().next().unwrap().to_digit(10) {
		Some(digit)
	} else if include_strings {
		for i in 0..number_strings.len() {
			if s.starts_with(number_strings[i]) {
				return Some(i as u32 + 1);
			}
		}
		None
	} else {
		None
	}
}

pub fn solve(inputs: Vec<String>) {
	for part in 0..2 {
		let digits = inputs.iter()
			.map(|line|  {
				let line_len = line.len();
				(0..line_len)
					.map(|i| &line[i..])
					.filter_map(|substr| str_to_digit(substr, /*include_strings=*/part == 1))
					.collect_vec()
			})
			.collect_vec();

		let numbers = digits.iter()
			.map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
			.collect_vec();

		println!("Part {}: {}", part + 1, numbers.iter().sum::<u32>());
	}
}
