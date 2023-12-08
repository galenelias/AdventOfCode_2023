// #[macro_use] extern crate lazy_static;
extern crate clap;
extern crate emergence;
extern crate itertools;
extern crate num;
extern crate regex;

use clap::Parser;
use std::io::{self, BufRead};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
	/// Reads puzzle input from the specified file
	#[arg(short, long)]
	file: Option<String>,

	/// Reads puzzle input from standard in
	#[arg(short, long)]
	stdin: bool,

	/// Specifies which day's challenge to run
	day: u32,
}

fn main() {
	let cli = Cli::parse();

	let input;
	if let Some(file_name) = cli.file {
		let contents = std::fs::read_to_string(file_name).expect("Can't read input file");
		input = contents.lines().map(String::from).collect();
	} else if cli.stdin {
		let stdin = io::stdin();
		input = stdin
			.lock()
			.lines()
			.filter_map(|l| l.ok())
			.collect::<Vec<String>>();
	} else {
		let aoc_fetcher = emergence::AoC::new(2023).expect("Couldn't instantiate AoC object");
		let prob_input = aoc_fetcher
			.read_or_fetch(cli.day as usize)
			.expect("Couldn't fetch problem input");
		input = prob_input
			.trim_end_matches('\n')
			.split('\n')
			.map(String::from)
			.collect::<Vec<String>>();
	}

	match cli.day {
		1 => day1::solve(input),
		2 => day2::solve(input),
		3 => day3::solve(input),
		4 => day4::solve(input),
		5 => day5::solve(input),
		6 => day6::solve(input),
		7 => day7::solve(input),
		8 => day8::solve(input),

		_ => println!("Oops! Day {} isn't implemented yet!", cli.day),
	}
}
