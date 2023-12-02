use std::fs::read_to_string;
use std::path::Path;

const RED_CUBE_LIMIT: u8 = 12;
const GREEN_CUBE_LIMIT: u8 = 13;
const BLUE_CUBE_LIMIT: u8 = 14;

pub(crate) fn second(path: impl AsRef<Path>, part: u8) -> usize
{
	return if part==1 { second_one(path) } else { second_two(path) }
}
pub(crate) fn second_one(path: impl AsRef<Path>) -> usize
{
	let input = read_to_string(path).unwrap();

	let mut sum = 0;

	'gameloop: for (line_num, line) in input.lines().enumerate()
	{
		let line = line.trim_start_matches("Game ").trim_start_matches(char::is_numeric).trim_start_matches(": ");
		for grab in line.split("; ")
		{
			for kvp in grab.split(", ")
			{
				match kvp.to_string().strip_suffix(" red")
				{
					Some(us) => if RED_CUBE_LIMIT<us.parse().unwrap() { continue 'gameloop } else { continue }
					None => {},
				}

				match kvp.to_string().strip_suffix(" green")
				{
					Some(us) => if GREEN_CUBE_LIMIT<us.parse().unwrap() { continue 'gameloop } else { continue }
					None => {},
				}

				match kvp.to_string().strip_suffix(" blue")
				{
					Some(us) => if BLUE_CUBE_LIMIT<us.parse().unwrap() { continue 'gameloop } else { continue }
					None => {},
				}
			}
		}
		sum += line_num+1;
	}

	sum
}

pub(crate) fn second_two(path: impl AsRef<Path>) -> usize
{
	let input = read_to_string(path).unwrap();

	let mut sum = 0;

	for line in input.lines()
	{
		let mut min_red_cubes = 0;
		let mut min_green_cubes = 0;
		let mut min_blue_cubes = 0;

		let line = line.trim_start_matches("Game ").trim_start_matches(char::is_numeric).trim_start_matches(": ");
		for grab in line.split("; ")
		{
			for kvp in grab.split(", ")
			{
				match kvp.to_string().strip_suffix(" red")
				{
					Some(us) =>
						{
							let num = us.parse().unwrap();
							if min_red_cubes<num { min_red_cubes = num } else { continue }
						}
					None => {},
				}

				match kvp.to_string().strip_suffix(" green")
				{
					Some(us) =>
						{
							let num = us.parse().unwrap();
							if min_green_cubes<num { min_green_cubes = num } else { continue }
						}
					None => {},
				}

				match kvp.to_string().strip_suffix(" blue")
				{
					Some(us) =>
						{
							let num = us.parse().unwrap();
							if min_blue_cubes<num { min_blue_cubes = num } else { continue }
						}
					None => {},
				}
			}
		}
		sum += min_red_cubes*min_green_cubes*min_blue_cubes;
	}

	sum
}