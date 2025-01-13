use aoc24::utils;

fn main() {
	let input = include_str!("../../puzzle_input/day14.txt");
	let result = process(&input);
	println!("{}", result);
}

fn process(input: &str) -> String {
	input.to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "";

	#[test]
	fn test_process() {
		assert_eq!("".to_string(), process(EXAMPLE));
	}
}


