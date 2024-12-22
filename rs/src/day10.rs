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
            rels.insert((x, y), (&tile, HashSet::new()));
            if let Some(possible) = grid.tiles[y].get(x + 1) {
                if possible.height() > 0 && tile.height() == possible.height() + 1 {
                    rels.entry((x, y)).and_modify(|(_, climbs)| {
                        climbs.insert((x + 1, y));
                    });
                }
            }
            if let Some(valid_x) = x.checked_sub(1) {
                if grid.tiles[y][valid_x].height() > 0
                    && tile.height() == grid.tiles[y][valid_x].height() - 1
                {
                    rels.entry((x, y)).and_modify(|(_, climbs)| {
                        climbs.insert((valid_x, y));
                    });
                }
            }
            if let Some(possible) = grid.tiles.get(y + 1) {
                if possible[x].height() > 0 && tile.height() == possible[x].height() - 1 {
                    rels.entry((x, y)).and_modify(|(_, climbs)| {
                        climbs.insert((y + 1, x));
                    });
                }
            }
            if let Some(valid_y) = y.checked_sub(1) {
                if grid.tiles[valid_y][x].height() > 0
                    && tile.height() == grid.tiles[valid_y][x].height() - 1
                {
                    rels.entry((x, y)).and_modify(|(_, climbs)| {
                        climbs.insert((x, valid_y));
                    });
                }
            }
        }
    }
    for (x, y) in rels.keys() {}
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
