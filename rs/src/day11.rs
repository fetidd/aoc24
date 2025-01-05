use aoc24::utils;

fn main() {
	let input = include_str!("../../puzzle_input/day11.txt");
	let result = process(&input);
	println!("{}", result);
}

fn process(input: &str) -> String {
	input.to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() {
		assert_eq!("input".to_string(), process("input"));
	}
}


