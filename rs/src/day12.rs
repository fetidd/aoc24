use aoc24::utils;

fn main() {
	let input = include_str!("../../puzzle_input/day12.txt");
	let result = process(&input);
	println!("{}", result);
}

fn process(input: &str) -> String {
	input.to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "RRRRIICCFF\n\
						   RRRRIICCCF\n\
						   VVRRRCCFFF\n\
						   VVRCCCJFFF\n\
						   VVVVCJJCFE\n\
						   VVIVCCJJEE\n\
						   VVIIICJJEE\n\
						   MIIIIIJJEE\n\
						   MIIISIJEEE\n\
						   MMMISSJEEE";

	#[test]
	fn test_process() {
		assert_eq!("1930".to_string(), process(EXAMPLE));
	}

	#[test]
	fn test_calculate_region() {
		let tests = [
			("RRRR.\n\
			  RRRR.\n\
			  ..RRR\n\
			  ..R..", 216),
		];
		for (region, expected) in tests {
			assert_eq!(expected, calculate_region(region));
		}
	}
}

