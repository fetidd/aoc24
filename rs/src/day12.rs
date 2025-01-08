use std::collections::{HashMap, HashSet};

use aoc24::utils::{
    self,
    grid::{Grid, GridCursor, GridPeekFn, GridPos}, Dir,
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
    for (pos, char) in &grid {
        if !assigned.contains(&pos) {
            let region = Region::plot(*char, pos, &grid);
            assigned.extend(&region.content);
            // total_cost += region.area() * region.perimeter();
            total_cost += region.area() * region.sides();
        }
    }
    total_cost.to_string()
}

#[derive(Debug, Clone)]
struct Region<'grid> {
    kind: char,
    content: Vec<GridPos>,
    grid: &'grid Grid<char>
}

impl<'grid> Region<'grid> {
    fn plot(kind: char, pos: GridPos, grid: &'grid Grid<char>) -> Self {
        let mut visited = HashSet::new();
        let mut plotted = vec![];
        plot_region(&mut plotted, pos, &grid, &mut visited, &kind);
        Self {
            kind,
            content: plotted,
            grid
        }
    }
    
    fn area(&self) -> usize {
        self.content.len()
    }

    fn sides(&self) -> usize {
        todo!()
    }

    fn get_plot_fences(&self, pos: GridPos) -> Vec<Dir> {
        let mut fences: Vec<Dir> = vec![];
        let mut cursor = self.grid.cursor();
        cursor.goto_unchecked(pos);    
        for (peek, dir) in [
            (GridCursor::north as GridPeekFn<char>, Dir::Up),
            (GridCursor::east as GridPeekFn<char>, Dir::Right),
            (GridCursor::south as GridPeekFn<char>, Dir::Down),
            (GridCursor::west as GridPeekFn<char>, Dir::Left),
        ] {
            if let Some((_, peeked)) = peek(&cursor) {
                if *peeked != self.kind {
                    fences.push(dir);
                }
            } else {
                fences.push(dir); // off the edge of the grid counts as fence!
            }
        }
        fences
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = 0;
        for pos in &self.content {
            perimeter += self.get_plot_fences(*pos).len();
        }
        perimeter
    }
}

fn plot_region(mut plotted: &mut Vec<GridPos>, pos: GridPos, grid: &Grid<char>, mut visited: &mut HashSet<GridPos>, kind: &char) {
    visited.insert(pos);
    plotted.push(pos);
    let mut cursor = grid.cursor();
    cursor.goto_unchecked(pos);
    for peek in [
        GridCursor::north,
        GridCursor::east,
        GridCursor::south,
        GridCursor::west,
    ] {
        if let Some((possible_pos, possible)) = peek(&cursor) {
            if !visited.contains(&possible_pos) && possible == kind {
                plot_region(&mut plotted, possible_pos, &grid, &mut visited, kind);
            }
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
