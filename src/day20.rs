use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Clone, Copy)]
enum ModuleType {
	FlipFlop,
	Conjunction,
	Broadcast,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Pulse {
	High,
	Low,
}

#[derive(Debug)]
struct Module<'a> {
	module_type: ModuleType,
	name: &'a str,
	outputs: Vec<&'a str>,
}

pub fn solve(inputs: Vec<String>) {
	let mut modules = HashMap::new();
	let mut module_inputs: HashMap<&str, Vec<&str>> = HashMap::new();
	let mut flipflop_states: HashMap<&str, bool> = HashMap::new();
	let mut conjunction_states: HashMap<&str, HashMap<&str, Pulse>> = HashMap::new();

	for line in &inputs {
		let (name, outputs) = line.split_once(" -> ").unwrap();

		let (name, module_type) = if name == "broadcaster" {
			(name, ModuleType::Broadcast)
		} else if name.starts_with("%") {
			(&name[1..], ModuleType::FlipFlop)
		} else if name.starts_with("&") {
			(&name[1..], ModuleType::Conjunction)
		} else {
			unreachable!();
		};

		let outputs = outputs.split(", ").collect_vec();

		for output in outputs.iter() {
			module_inputs.entry(output).or_default().push(name);
		}

		modules.insert(
			name,
			Module {
				module_type,
				name,
				outputs,
			},
		);

		match module_type {
			ModuleType::FlipFlop => {
				flipflop_states.insert(name, false);
			}
			ModuleType::Conjunction => {
				conjunction_states.insert(name, HashMap::new());
			}
			_ => {}
		}
	}

	// Initialize conjection states
	for (name, module) in &modules {
		for output in &module.outputs {
			if let Some(output_module) = modules.get(output) {
				if output_module.module_type == ModuleType::Conjunction {
					conjunction_states
						.get_mut(output)
						.unwrap()
						.insert(name, Pulse::Low);
				}
			}
		}
	}

	let mut signals = VecDeque::new();
	let mut low_pulses = 0;
	let mut high_pulses = 0;

	let mut last_activation_pushes = HashMap::new();
	let mut conjuction_period = HashMap::new();

	let num_conjunctions = modules
		.values()
		.filter(|m| m.module_type == ModuleType::Conjunction)
		.count();

	for pushes in 1i64.. {
		// Push button signal
		signals.push_back(("button", "broadcaster", Pulse::Low));

		while !signals.is_empty() {
			let (source, dest, pulse) = signals.pop_front().unwrap();

			if pulse == Pulse::Low {
				low_pulses += 1;
			} else {
				high_pulses += 1;
			}

			// Unknown module destinations just fizzle
			if let Some(module) = modules.get(dest) {
				match module.module_type {
					ModuleType::FlipFlop => {
						if pulse == Pulse::Low {
							let status = !flipflop_states.get(module.name).unwrap();
							flipflop_states.insert(module.name, status);
							let out_pulse = if status { Pulse::High } else { Pulse::Low };
							for output in module.outputs.iter() {
								signals.push_back((module.name, output, out_pulse));
							}
						}
					}
					ModuleType::Conjunction => {
						let conjunction_state = conjunction_states.get_mut(module.name).unwrap();
						conjunction_state.insert(source, pulse);

						let out_pulse = if conjunction_state.values().all(|&p| p == Pulse::High) {
							if let Some(last_push) = last_activation_pushes.get(module.name) {
								let period = pushes - last_push;
								if let Some(last_period) = conjuction_period.get(module.name) {
									if period != 0 && period != *last_period {
										println!(
											"{}: Period changed from {} to {}!!",
											module.name, last_period, period
										);
									}
								} else {
									conjuction_period.insert(module.name, period);

									// We don't want to wait for the final conjunction to activate, as that will trigger the 'rx' module
									// But the LCM of the periods of all the previous conjunctions will determine the period of the final
									// conjunction and hence the final output
									if conjuction_period.len() == num_conjunctions - 1 {
										// Should use LCM, but this is likely good enough
										println!(
											"Part 2: {}",
											conjuction_period.values().fold(1, |acc, &p| acc * p)
										);
										return;
									}
								}
							}
							last_activation_pushes.insert(module.name, pushes);

							Pulse::Low
						} else {
							Pulse::High
						};

						for output in module.outputs.iter() {
							signals.push_back((module.name, output, out_pulse));
						}
					}
					ModuleType::Broadcast => {
						for output in module.outputs.iter() {
							signals.push_back((module.name, output, pulse));
						}
					}
				}
			}
		}

		if pushes == 1000 {
			println!("Part 1: {}", low_pulses * high_pulses);
		}
	}
}
