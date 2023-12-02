use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use clap::{Error, Parser, ValueEnum};
use clap::builder::TypedValueParser;
use clap::error::ErrorKind;
use crate::Advent::{Eighteenth, Eighth, Eleventh, Fifteenth, Fifth, First, Fourteenth, Fourth, Nineteenth, Ninth, Second, Seventeenth, Seventh, Sixteenth, Sixth, Tenth, Third, Thirteenth, Twelfth, Twentieth, TwentyFifth, TwentyFirst, TwentyFourth, TwentySecond, TwentyThird};

/// Bundled Advent of Code 2023 executable
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
	/// Day of advent
	#[arg(short, long)]
	advent: Advent,

	/// Working directory
	#[arg(short, long, default_value = ".")]
	directory: String
}

fn main() {
	let args = Args::parse();

	match args.advent
	{
		First => println!("Calibration: {}", first(format!("{}/res/1/calibration", args.directory)).unwrap()),
		Second => {}
		Third => {}
		Fourth => {}
		Fifth => {}
		Sixth => {}
		Seventh => {}
		Eighth => {}
		Ninth => {}
		Tenth => {}
		Eleventh => {}
		Twelfth => {}
		Thirteenth => {}
		Fourteenth => {}
		Fifteenth => {}
		Sixteenth => {}
		Seventeenth => {}
		Eighteenth => {}
		Nineteenth => {}
		Twentieth => {}
		TwentyFirst => {}
		TwentySecond => {}
		TwentyThird => {}
		TwentyFourth => {}
		TwentyFifth => {}
	}
}

#[derive(Copy, Clone,ValueEnum, Debug)]
enum Advent
{
	First,
	Second,
	Third,
	Fourth,
	Fifth,
	Sixth,
	Seventh,
	Eighth,
	Ninth,
	Tenth,
	Eleventh,
	Twelfth,
	Thirteenth,
	Fourteenth,
	Fifteenth,
	Sixteenth,
	Seventeenth,
	Eighteenth,
	Nineteenth,
	Twentieth,
	TwentyFirst,
	TwentySecond,
	TwentyThird,
	TwentyFourth,
	TwentyFifth
}

impl FromStr for Advent
{
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		return match s.to_lowercase().as_str() {
			"first" => Ok(First),
			"second" => Ok(Second),
			"third" => Ok(Third),
			"fourth" => Ok(Fourth),
			"fifth" => Ok(Fifth),
			"sixth" => Ok(Sixth),
			"seventh" => Ok(Seventh),
			"eighth" => Ok(Eighth),
			"ninth" => Ok(Ninth),
			"tenth" => Ok(Tenth),
			"eleventh" => Ok(Eleventh),
			"twelfth" => Ok(Twelfth),
			"thirteenth" => Ok(Thirteenth),
			"fourteenth" => Ok(Fourteenth),
			"fifteenth" => Ok(Fifteenth),
			"sixteenth" => Ok(Sixteenth),
			"seventeenth" => Ok(Seventeenth),
			"eighteenth" => Ok(Eighteenth),
			"nineteenth" => Ok(Nineteenth),
			"twentieth" => Ok(Twentieth),
			"twenty-first" => Ok(TwentyFirst),
			"twenty-second" => Ok(TwentySecond),
			"twenty-third" => Ok(TwentyThird),
			"twenty-fourth" => Ok(TwentyFourth),
			"twenty-fifth" => Ok(TwentyFifth),
			_ => Err(Self::Err::new(ErrorKind::InvalidValue))
		}
	}
}

fn first(path: impl AsRef<Path>) -> io::Result<usize>
{
	let file = match File::open(path)
	{
		Ok(f) => f,
		Err(e) => return Err(e)
	};

	let mut sum = 0;

	for line in BufReader::new(file).lines()
	{
		match line
		{
			Ok(mut s) => {

				if s.is_empty()
				{
					continue;
				}

				sum += digit_sum(&s);
			},
			Err(e) => return Err(e),
		}
	}

	return Ok(sum);
}
fn digit_sum(line: &str) -> usize
{
	let patterns = vec![
		//("0", 0), ("zero", 0),
		("1", 1), ("one", 1),
		("2", 2), ("two", 2),
		("3", 3), ("three", 3),
		("4", 4), ("four", 4),
		("5", 5), ("five", 5),
		("6", 6), ("six", 6),
		("7", 7), ("seven", 7),
		("8", 8), ("eight", 8),
		("9", 9), ("nine", 9)
	];

	let mut min = usize::MAX;
	let mut max = 0;

	let mut map = HashMap::new();

	for s in patterns
	{
		match line.to_lowercase().find(s.0)
		{
			Some(f) => {
				map.insert(f, s.1);
				if min > f
				{
					min = f;
				}
				if max < f
				{
					max = f;
				}
			},
			None => continue
		};
		match line.to_lowercase().rfind(s.0)
		{
			Some(f) => {
				map.insert(f, s.1);
				if min > f
				{
					min = f;
				}
				if max < f
				{
					max = f;
				}
			},
			None => continue
		};
	}

	if map.is_empty()
	{
		return 0;
	}

	format!("{}{}", map.get(&min).unwrap(), map.get(&max).unwrap()).parse().unwrap()
}