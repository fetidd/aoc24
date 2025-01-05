mod utils;
use std::collections::{HashMap, HashSet};

use utils::grid::{Grid, GridCursor, GridPos};

fn main() {
    let input = include_str!("../../puzzle_input/day10.txt");
    let result = process(&input);
    println!("{}", result);
}

type RelMap<'a> = HashMap<GridPos, (&'a Tile, HashSet<GridPos>)>;

fn process(input: &str) -> String {
    let grid: Grid<Tile> = Grid::new(input);
    let mut rels: RelMap = HashMap::new();
    for (pos, tile) in &grid {
        let mut nexts = HashSet::new();
        let mut c = grid.cursor();
        c.goto(pos).expect("this should always be in bounds");
        for peek in [
            GridCursor::north,
            GridCursor::east,
            GridCursor::south,
            GridCursor::west,
        ] {
            if let Some((possible_pos, possible)) = peek(&c) {
                if possible.height() == tile.height() + 1 {
                    nexts.insert(possible_pos);
                }
            }
        }
        rels.insert(c.pos(), (tile, nexts));
    }
    let mut scores = 0;
    for (_, (tile, neighbours)) in &rels {
        if tile.height() == 0 {
            let mut trails = HashSet::new();
            for n in neighbours {
                let mut trail = vec![];
                find_trails(&n, &rels, &mut trail, &mut trails);
            }
            scores += trails.len();
        }
    }
    scores.to_string()
}

fn find_trails(
    coord: &GridPos,
    rels: &RelMap,
    mut trail: &mut Vec<GridPos>,
    mut trails: &mut HashSet<Vec<GridPos>>,
) {
    if let Some((tile, neighbours)) = rels.get(&coord) {
        trail.push(*coord);
        if tile.height() == 9 {
            trails.insert(trail.to_owned());
        } else {
            for n in neighbours {
                find_trails(n, rels, &mut trail, &mut trails);
            }
        }
    }
}

// fn find_peaks(coord: &GridPos, rels: &RelMap, mut peaks: &mut HashSet<GridPos>) {
//     if let Some((tile, neighbours)) = rels.get(&coord) {
//         if tile.height() == 9 {
//             peaks.insert(*coord);
//         } else {
//             for n in neighbours {
//                 find_peaks(n, rels, &mut peaks);
//             }
//         }
//     }
// }

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

    const SMALL_EXAMPLE: &str = "0123\n\
    						     9854\n\
    						     8765";

    #[test]
    fn test_process() {
        assert_eq!("1".to_string(), process(SMALL_EXAMPLE));
        assert_eq!("36".to_string(), process(EXAMPLE));
    }

    #[test]
    fn test_fine_peaks() {
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
        let mut peaks = HashSet::new();
        find_peaks(&(1, 1), &rels, &mut peaks);
        assert_eq!(1, peaks.len());
    }
}
