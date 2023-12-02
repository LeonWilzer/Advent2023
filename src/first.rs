use std::collections::HashMap;
use std::path::Path;
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn first(path: impl AsRef<Path>) -> io::Result<usize>
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
