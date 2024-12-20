mod utils;
use utils::Grid;

fn main() {
	let input = include_str!("../../puzzle_input/day10.txt");
	let result = process(&input);
	println!("{}", result);
}

fn process(input: &str) -> String {
	let mut grid: Grid<Tile> = Grid::new(input);
	let trailheads = grid.find_trailheads();
	let mut scores = 0;
	for trailhead in trailheads {
		
	}
	todo!();
}

impl Grid<Tile> {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Tile {
	Trailhead,
	Level(u32),
	Peak
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value.to_digit(10) {
            Some(0) => Self::Trailhead,
            Some(9) => Self::Peak,
            Some(height) => Self::Level(height),
            None => panic!("bad char"),
        }
    }
}


#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "89010123\n\
						   78121874\n\
						   87430965\n\
						   96549874\n\
						   45678903\n\
						   32019012\n\
						   01329801\n\
						   10456732";
	
	#[test]
	fn test_process() {
		assert_eq!("36".to_string(), process(EXAMPLE));
	}
}


