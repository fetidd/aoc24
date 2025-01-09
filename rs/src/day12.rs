use std::collections::{HashMap, HashSet};

use aoc24::utils::{
    grid::{Grid, GridCursor, GridPeekFn, GridPos},
    Dir,
};

fn main() {
    let input = include_str!("../../puzzle_input/day12.txt");
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let grid: Grid<char> = Grid::new(input);
    let mut total_cost = 0;
    let mut assigned: HashSet<GridPos> = HashSet::new();
    for (pos, kind) in &grid {
        if !assigned.contains(&pos) {
            let region = Region::map_out(*kind, pos, &grid);
            assigned.extend(region.content.keys());
            // total_cost += region.area() * region.perimeter();
            total_cost += region.area() * region.sides();
        }
    }
    total_cost.to_string()
}

#[derive(Debug, Clone, PartialEq)]
struct Plot {
    kind: char,
    pos: GridPos,
    fences: Vec<Dir>,
}

impl Plot {
    fn new(kind: char, pos: GridPos) -> Self {
        Self {
            kind,
            pos,
            fences: vec![],
        }
    }
}

#[derive(Debug, Clone)]
struct Region<'grid> {
    kind: char,
    content: HashMap<GridPos, Plot>,
    grid: &'grid Grid<char>,
}

impl<'grid> Region<'grid> {
    fn map_out(kind: char, pos: GridPos, grid: &'grid Grid<char>) -> Self {
        let mut content = HashMap::new();
        map_out_region(&mut content, pos, &grid, kind);
        Self {
            kind,
            content,
            grid,
        }
    }

    fn area(&self) -> usize {
        self.content.len()
    }

    fn sides(&self) -> usize {
        let mut sides = 0;
        for (pos, plot) in &self.content {
            println!("({}, {}) -> {plot:?}", pos.0, pos.1);
        }
        sides
    }

    fn perimeter(&self) -> usize {
        self.content.iter().map(|(_, p)| p.fences.len()).sum()
    }
}

fn map_out_region(
    mut mapped: &mut HashMap<GridPos, Plot>,
    pos: GridPos,
    grid: &Grid<char>,
    kind: char,
) {
    let plot = Plot::new(kind, pos);
    mapped.insert(pos, plot);
    let mut cursor = grid.cursor();
    cursor.goto_unchecked(pos);
    for (peek, dir) in [
        (GridCursor::north as GridPeekFn<char>, Dir::Up),
        (GridCursor::east as GridPeekFn<char>, Dir::Right),
        (GridCursor::south as GridPeekFn<char>, Dir::Down),
        (GridCursor::west as GridPeekFn<char>, Dir::Left),
    ] {
        if let Some((peeked_pos, peeked)) = peek(&cursor) {
            if *peeked == kind {
                if !mapped.contains_key(&peeked_pos) {
                    map_out_region(&mut mapped, peeked_pos, &grid, kind);
                }
            } else {
                mapped.entry(pos).and_modify(|p| p.fences.push(dir));
            }
        } else {
            mapped.entry(pos).and_modify(|p| p.fences.push(dir));
        }
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
        // assert_eq!("1930".to_string(), process(EXAMPLE));
        assert_eq!("1206".to_string(), process(EXAMPLE));
    }

    #[test]
    fn test_sides() {
        let tests = [("RRRR.\nRRRR.\n..RRR\n..R..", 216)];
        for (region, expected) in tests {
            println!("{region}");
            let grid = Grid::new(&region);
            let region = Region::map_out('R', (0, 0), &grid);
            assert_eq!(expected, region.sides());
        }
    }
}
