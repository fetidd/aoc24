mod utils;
use std::collections::{HashMap, HashSet};

use utils::Grid;

fn main() {
    let input = include_str!("../../puzzle_input/day10.txt");
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let mut grid: Grid<Tile> = Grid::new(input);
    let mut scores = 0;
    let mut rels: HashMap<(usize, usize), (&Tile, HashSet<(usize, usize)>)> = HashMap::new();
    for (y, inner) in grid.tiles.iter().enumerate() {
        for (x, tile) in inner.iter().enumerate() {
            dbg!(&rels);
            rels.insert((x, y), (&tile, HashSet::new()));
            let entry = rels.entry((x, y));
            if let Some(right) = grid.tiles[y].get(x + 1) {
                entry.and_modify(|(_, climbs)| {
                    if tile.height() == right.height() + 1 {
                        climbs.insert((x + 1, y));
                    }
                });
            }
        }
    }
    todo!();
}

impl Grid<Tile> {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Tile {
    Trailhead,
    Level(u32),
    Peak,
}

impl Tile {
    fn height(&self) -> u32 {
        match self {
            Tile::Trailhead => 0,
            Tile::Level(height) => *height,
            Tile::Peak => 9,
        }
    }
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
