use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
enum Destination {
	Accept,
	Reject,
	Workflow(String),
}

enum Operation {
	LessThan,
	GreaterThan,
}

struct Condition {
	category: char,
	op: Operation,
	value: i64,
}

struct Rule {
	condition: Option<Condition>,
	dest: Destination,
}

struct Workflow {
	rules: Vec<Rule>,
}

#[derive(Debug, Copy, Clone)]
struct Part {
	x: i64,
	m: i64,
	a: i64,
	s: i64,
}

impl Part {
	fn new() -> Self {
		Self {
			x: 0,
			m: 0,
			a: 0,
			s: 0,
		}
	}

	fn set_category(&mut self, category: char, value: i64) {
		match category {
			'x' => self.x = value,
			'm' => self.m = value,
			'a' => self.a = value,
			's' => self.s = value,
			_ => unreachable!("Unexpected category {category}"),
		}
	}

	fn get_category(&self, category: char) -> i64 {
		match category {
			'x' => self.x,
			'm' => self.m,
			'a' => self.a,
			's' => self.s,
			_ => unreachable!(),
		}
	}
}

pub fn solve(inputs: Vec<String>) {
	// name -> Workflow
	let mut workflows = HashMap::new();
	let mut i = 0;
	while !inputs[i].is_empty() {
		let input = &inputs[i];
		let (name, rules_str) = input.split_once('{').unwrap();
		let rules_str = &rules_str[..rules_str.len()-1];

		let rules = rules_str.split(',').map(|rule_str| {
			if let Some((cond_str, dest_str)) = rule_str.split_once(":") {
				let mut cond_iter = cond_str.chars();
				let category = cond_iter.next().unwrap();
				let op = match cond_iter.next().unwrap() {
					'<' => Operation::LessThan,
					'>' => Operation::GreaterThan,
					_ => unreachable!(),
				};
				let value = cond_str[2..].parse::<i64>().unwrap();

				let dest = match dest_str {
					"A" => Destination::Accept,
					"R" => Destination::Reject,
					_ => Destination::Workflow(dest_str.to_string()),
				};

				return Rule {
					condition: Some(Condition {
						category,
						op,
						value,
					}),
					dest,
				};	
			} else {
				let dest = match rule_str {
					"A" => Destination::Accept,
					"R" => Destination::Reject,
					_ => Destination::Workflow(rule_str.to_string()),
				};

				return Rule {
					condition: None,
					dest,
				};
			}
		}).collect_vec();

		workflows.insert(name.to_string(), Workflow { rules });
		i += 1;
	}

	i += 1;
	let parts = inputs[i..].iter().map(|line| {
		let line = &line[1..line.len()-1];
		let mut part = Part::new();

		for assignment in line.split(",") {
			let (category, value) = assignment.split_once("=").unwrap();
			part.set_category(category.chars().next().unwrap(), value.parse::<i64>().unwrap());
		}
		return part;
	}).collect_vec();

	let mut part1 = 0;
	for part in &parts {
		let mut workflow = String::from("in");

		loop {
			let rules = &workflows.get(&workflow).unwrap().rules;
			let mut dest = None;

			for rule in rules {
				if let Some(cond) = &rule.condition {
					let part_val = part.get_category(cond.category);
					let cond_val = cond.value;

					let accept = match cond.op {
						Operation::LessThan => part_val < cond_val,
						Operation::GreaterThan => part_val > cond_val,
					};

					if accept {
						dest = Some(rule.dest.clone());
						break;
					}
				} else {
					dest = Some(rule.dest.clone());
					break;
				}
			}

			match dest {
				Some(Destination::Accept) => {
					part1 += part.x + part.m + part.a + part.s;
					break;
				}
				Some(Destination::Reject) => { break; }
				Some(Destination::Workflow(name)) => workflow = name,
				None => panic!("No destination found"),
			}
		}
	}
	println!("Part 1: {}", part1);

	let mut queue = VecDeque::new();
	queue.push_back((Destination::Workflow(String::from("in")), Part{ x: 1, m: 1, a: 1, s: 1 }, Part{ x: 4000, m: 4000, a: 4000, s: 4000 }));

	let mut passing_parts = Vec::new();

	while !queue.is_empty() {
		let (dest, mut min_part, mut max_part) = queue.pop_front().unwrap();

		let workflow = match dest {
			Destination::Accept => {
				passing_parts.push((min_part, max_part));
				continue;
			}
			Destination::Reject => {
				continue;
			}
			Destination::Workflow(name) => {
				name
			}
		};

		let rules = &workflows.get(&workflow).unwrap().rules;

		for rule in rules {
			if let Some(cond) = &rule.condition {
				let cond_val = cond.value;

				// See if our current constraints always match the condition, at which point we should always branch
				// and not continue applying further rules
				let accept = match cond.op {
					Operation::LessThan => max_part.get_category(cond.category) < cond_val,
					Operation::GreaterThan => min_part.get_category(cond.category) > cond_val,
				};

				// If our current constraint never matches the condition, then skip this rule
				let reject = match cond.op {
					Operation::LessThan => min_part.get_category(cond.category) > cond_val,
					Operation::GreaterThan => max_part.get_category(cond.category) < cond_val,
				};

				if accept {
					queue.push_back((rule.dest.clone(), min_part.clone(), max_part.clone()));
					break;
				} else if reject {
					continue;
				} else {
					// Branch our potential values, one that satisfies the condition, and one that doesn't
					let mut pass_part_min = min_part.clone();
					let mut pass_part_max = max_part.clone();

					match cond.op {
						Operation::LessThan => {
							pass_part_max.set_category(cond.category, cond_val - 1);
							min_part.set_category(cond.category, cond_val);
						}
						Operation::GreaterThan => {
							pass_part_min.set_category(cond.category, cond_val + 1);
							max_part.set_category(cond.category, cond_val);
						}
					}

					queue.push_back((rule.dest.clone(), pass_part_min, pass_part_max));
				}
			} else {
				queue.push_back((rule.dest.clone(), min_part.clone(), max_part.clone()));
				break;
			}
		}
	}

	let part2 = passing_parts.iter().map(|(min, max)| 
		(max.x + 1 - min.x) * (max.m + 1 - min.m) * (max.a + 1 - min.a) * (max.s + 1 - min.s)
	).sum::<i64>();

	println!("Part 2: {}", part2);
}
