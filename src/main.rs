mod first;

use std::io::BufRead;
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
		First => println!("Calibration: {}", first::first(format!("{}/res/1/calibration", args.directory)).unwrap()),
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