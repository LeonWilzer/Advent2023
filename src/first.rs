use std::collections::HashMap;
use std::path::Path;
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn first(path: impl AsRef<Path>, part: u8) -> io::Result<usize>
{
	let part = if part==1 { false } else { true };

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
			Ok(s) => {

				if s.is_empty()
				{
					continue;
				}

				sum += digit_sum(&s, part);
			},
			Err(e) => return Err(e),
		}
	}

	return Ok(sum);
}

fn digit_sum(line: &str, part: bool) -> usize
{
	let mut patterns = vec![
	("1", 1),
	("2", 2),
	("3", 3),
	("4", 4),
	("5", 5),
	("6", 6),
	("7", 7),
	("8", 8),
	("9", 9),
	];
	if part
	{
		patterns.extend(vec![
			("one", 1),
			("two", 2),
			("three", 3),
			("four", 4),
			("five", 5),
			("six", 6),
			("seven", 7),
			("eight", 8),
			("nine", 9),
		]
		);
	}

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
