mod utils;
use std::collections::{HashMap, HashSet};

use utils::Grid;

fn main() {
    let input = include_str!("../../puzzle_input/day10.txt");
    let result = process(&input);
    println!("{}", result);
}

type RelMap<'a> = HashMap<(usize, usize), (&'a Tile, HashSet<(usize, usize)>)>;

fn process(input: &str) -> String {
    let grid: Grid<Tile> = Grid::new(input);
    let mut scores = 0;
    let mut rels: RelMap = HashMap::new();
    println!("processing grid...");
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
    println!("calculating...");
    println!("{:?}", rels.keys());
    println!("{:?}", rels.values());
    for (tile, neighbours) in rels.values() {
        if tile.height() == 0 {
            for n in neighbours {
                let sc = calculate_score(&n, &rels);
                scores += sc;
            }
        }
    }
    scores.to_string()
}

fn calculate_score(coord: &(usize, usize), rels: &RelMap) -> u32 {
    if let Some((tile, neighbours)) = rels.get(&coord) {
        if tile.height() == 9 {
            1
        } else {
            let mut score = 0;
            for n in neighbours {
                let sc = calculate_score(n, rels);
                score += sc;
            }
            score
        }
    } else {
        0
    }
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

    // #[ignore]
    #[test]
    fn test_process() {
        assert_eq!("36".to_string(), process(EXAMPLE));
    }

    #[test]
    fn test_calculate_score() {
        let tile_0 = Tile::Trailhead;
        let tile_1 = Tile::Level(1);
        let tile_2 = Tile::Level(2);
        let tile_3 = Tile::Level(3);
        let tile_4 = Tile::Level(4);
        let tile_5 = Tile::Level(5);
        let tile_6 = Tile::Level(6);
        let tile_7 = Tile::Level(7);
        let tile_8 = Tile::Level(8);
        let tile_9 = Tile::Peak;
        let rels = HashMap::from([
            ((1, 1), (&tile_0, HashSet::from([(1, 2)]))),
            ((1, 2), (&tile_1, HashSet::from([(1, 3)]))),
            ((1, 3), (&tile_2, HashSet::from([(1, 4)]))),
            ((1, 4), (&tile_3, HashSet::from([(1, 5)]))),
            ((1, 5), (&tile_4, HashSet::from([(1, 6)]))),
            ((1, 6), (&tile_5, HashSet::from([(1, 7)]))),
            ((1, 7), (&tile_6, HashSet::from([(1, 8)]))),
            ((1, 8), (&tile_7, HashSet::from([(1, 9)]))),
            ((1, 9), (&tile_8, HashSet::from([(1, 10)]))),
            ((1, 10), (&tile_9, HashSet::new())),
        ]);
        assert_eq!(1, calculate_score(&(1, 1), &rels));
        for (tile, neighbours) in rels.values() {
            for n in neighbours {
                println!("{}", rels.get(&n).unwrap().0.height());
            }
        }
    }
}
