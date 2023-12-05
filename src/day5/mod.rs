use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Mapping {
	dst_range_start: usize,
	src_range_start: usize,
	range_length: usize,
}

#[derive(Debug, PartialEq)]
struct Value {
	category: String,
	value: usize,
}

#[derive(Debug, PartialEq)]
struct Conversion {
	src_category: String,
	dst_category: String,
	mappings: Vec<Mapping>,
}

fn map_value(value: usize, mappings: &[Mapping]) -> usize {
	for mapping in mappings {
		if value >= mapping.src_range_start
			&& value < mapping.src_range_start + mapping.range_length
		{
			return mapping.dst_range_start + value - mapping.src_range_start;
		}
	}

	return value;
}

fn map_value_ranges(value: (usize, usize), mappings: &[Mapping]) -> Vec<(usize, usize)> {
	let mut result = Vec::new();

	let mut start = value.0;
	let mut len = value.1;

	while len > 0 {
		let mut found = false;
		for mapping in mappings {
			if start >= mapping.src_range_start
				&& start < mapping.src_range_start + mapping.range_length
			{
				let mapped_len =
					std::cmp::min(len, mapping.src_range_start + mapping.range_length - start);
				result.push((
					mapping.dst_range_start + start - mapping.src_range_start,
					mapped_len,
				));
				len -= mapped_len;
				start += mapped_len;
				found = true;
				break;
			}
		}

		if !found {
			let mut next_range = None;

			for mapping in mappings {
				if mapping.src_range_start > start
					&& (next_range.is_none() || mapping.src_range_start < next_range.unwrap())
				{
					next_range = Some(mapping.src_range_start);
				}
			}

			if let Some(next_range) = next_range {
				let len_before_next_range = std::cmp::min(len, next_range - start);
				result.push((start, len_before_next_range));
				len -= len_before_next_range;
				start += len_before_next_range;
			} else {
				result.push((start, len));
				len = 0;
			}
		}
	}

	return result;
}

pub fn solve(inputs: Vec<String>) {
	let mut seeds = Vec::new();
	let mut i = 0;
	let mut conversions = Vec::new();

	while i < inputs.len() {
		let line = &inputs[i];
		i += 1;

		if line.starts_with("seeds:") {
			seeds = line
				.split(": ")
				.nth(1)
				.unwrap()
				.split(" ")
				.map(|s| s.parse::<usize>().unwrap())
				.collect_vec();
		} else if line.ends_with(" map:") {
			let (src_cat, dst_cat) = line.split_once(" ").unwrap().0.split_once("-to-").unwrap();

			let mut mappings = Vec::new();
			while i < inputs.len() && !inputs[i].is_empty() {
				let values = inputs[i]
					.split(" ")
					.map(|s| s.parse::<usize>().unwrap())
					.collect_vec();
				mappings.push(Mapping {
					dst_range_start: values[0],
					src_range_start: values[1],
					range_length: values[2],
				});
				i += 1;
			}

			conversions.push(Conversion {
				src_category: src_cat.to_string(),
				dst_category: dst_cat.to_string(),
				mappings,
			});
		}
	}

	let mut category = String::from("seed");
	let mut values = seeds.clone();

	while category != "location" {
		let conversion = conversions
			.iter()
			.find(|c| c.src_category == category)
			.unwrap();

		values = values
			.iter()
			.map(|v| map_value(*v, &conversion.mappings))
			.collect_vec();
		category = conversion.dst_category.clone();
	}

	println!("Part 1: {}", values.iter().min().unwrap());

	let mut value_ranges = Vec::new();
	for i in 0..seeds.len() / 2 {
		value_ranges.push((seeds[i * 2], seeds[i * 2 + 1]));
	}

	let mut category = String::from("seed");

	while category != "location" {
		let conversion = conversions
			.iter()
			.find(|c| c.src_category == category)
			.unwrap();

		let mut new_value_ranges = Vec::new();
		for value_range in value_ranges {
			new_value_ranges.append(&mut map_value_ranges(value_range, &conversion.mappings));
		}
		value_ranges = new_value_ranges;
		category = conversion.dst_category.clone();
	}

	println!(
		"Part 2: {}",
		value_ranges.iter().map(|range| range.0).min().unwrap()
	);
}
