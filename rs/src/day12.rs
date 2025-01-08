use std::collections::HashMap;

use aoc24::utils::{
    self,
    grid::{Grid, GridCursor, GridPos},
};

fn main() {
    let input = include_str!("../../puzzle_input/day12.txt");
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let grid: Grid<char> = Grid::new(input);
    let mut region_ids: HashMap<char, usize> = HashMap::new();
    let mut regions: Vec<Region> = vec![];
    for (pos, char) in &grid {
        let mut cursor = grid.cursor();
        cursor.goto(pos);
        let mut region = Region::new(*char, &mut region_ids).build(cursor);
    }
    todo!()
}

struct Region {
    id: usize,
    kind: char,
    content: Option<Vec<GridPos>>,
}

impl Region {
    fn new(kind: char, ids: &mut HashMap<char, usize>) -> Self {
        ids.entry(kind).and_modify(|e| *e += 1).or_insert(0);
        Self {
            kind,
            id: ids[&kind],
            content: None,
        }
    }

    fn build(self, cursor: GridCursor<char>) -> Self {
        let content = vec![];
        let content = Some(content);
        Self { content, ..self }
    }
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

    // #[test]
    // fn test_calculate_region() {
    //     let tests = [
    //         ("RRRR.\nRRRR.\n..RRR\n..R..", 216),
    //         ("....\n.II.\n.II.\n....", 32),
    //     ];
    //     for (region, expected) in tests {
    //         assert_eq!(expected, calculate_region(region));
    //     }
    // }
}
