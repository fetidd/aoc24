use std::collections::{HashMap, HashSet};

use aoc24::utils::{
    grid::{cmp_grid_pos, Grid, GridCursor, GridPeekFn, GridPos},
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Fences {
    top: bool,
    left: bool,
    right: bool,
    bottom: bool,
}

impl Fences {
    fn count(&self) -> usize {
        let mut count = 0;
        if self.top {
            count += 1;
        }
        if self.left {
            count += 1;
        }
        if self.right {
            count += 1;
        }
        if self.bottom {
            count += 1;
        }
        count
    }

    fn add(&mut self, dir: Dir) {
        match dir {
            Dir::Up => self.top = true,
            Dir::Left => self.left = true,
            Dir::Down => self.bottom = true,
            Dir::Right => self.right = true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Plot {
    kind: char,
    pos: GridPos,
    fences: Fences,
}

impl Plot {
    fn new(kind: char, pos: GridPos) -> Self {
        Self {
            kind,
            pos,
            fences: Fences::default(),
        }
    }

    fn is_edge(&self) -> bool {
        self.fences.count() > 0
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
        println!("{self}");
        let mut to_go: HashMap<GridPos, Plot> = self
            .content
            .iter()
            .map(|(pos, p)| (*pos, p.to_owned()))
            .filter(|(_, p)| p.is_edge())
            .collect();
        for pos in self.content.keys() {
            if to_go.contains(pos) {
                let c = self.grid.cursor_at(pos).unwrap();
                if let Some(plot) = to_go.get_mut(plot) {
                    
                }
            }
        }
        sides
    }

    fn perimeter(&self) -> usize {
        self.content.iter().map(|(_, p)| p.fences.count()).sum()
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
                mapped.entry(pos).and_modify(|p| p.fences.add(dir));
            }
        } else {
            mapped.entry(pos).and_modify(|p| p.fences.add(dir));
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PlotDisplay {
    top: (char, char, char),
    mid: (char, char, char),
    bot: (char, char, char),
}

impl PlotDisplay {
    fn new(plot: &Plot) -> Self {
        let row = (' ', ' ', ' ');
        Self {
            top: row,
            mid: (' ', plot.kind, ' '),
            bot: row,
        }
    }
}

impl From<&Plot> for PlotDisplay {
    fn from(value: &Plot) -> Self {
        let mut output = Self::new(value);
        if value.fences.top {
            output.top = ('.', '.', '.');
        }
        if value.fences.left {
            output.top.0 = if value.fences.top { '.' } else { ':' };
            output.mid.0 = ':';
            output.bot.0 = if value.fences.bottom { '.' } else { ':' };
        }
        if value.fences.bottom {
            output.bot = ('.', '.', '.');
        }
        if value.fences.right {
            output.top.2 = if value.fences.top { '.' } else { ':' };
            output.mid.2 = ':';
            output.bot.2 = if value.fences.bottom { '.' } else { ':' };
        }
        output
    }
}

impl<'grid> std::fmt::Display for Region<'grid> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        let mut sorted_positions: Vec<_> = self.content.keys().clone().collect();
        sorted_positions.sort_by(|a, b| cmp_grid_pos(a, b));
        let mut i = 0;
        while i < sorted_positions.len() {
            let mut to_process = vec![];
            let mut j = i;
            while j < sorted_positions.len() && sorted_positions[j].1 == sorted_positions[i].1 {
                to_process.push(sorted_positions[j]);
                j += 1;
            }
            let plots: Vec<&Plot> = to_process
                .iter()
                .map(|pos| self.content.get(pos).unwrap())
                .collect();
            let mut top_line = String::new();
            let mut mid_line = String::new();
            let mut bot_line = String::new();
            let mut plots_iter = plots.iter();
            let mut curr_plot = plots_iter.next();
            for indent in 0..(plots.iter().map(|p| p.pos.0).max().unwrap() + 1) {
                if let Some(plot) = curr_plot {
                    if plot.pos.0 == indent {
                        let display = PlotDisplay::from(*plot);
                        top_line.push_str(&format!(
                            "{}{}{}",
                            display.top.0, display.top.1, display.top.2
                        ));
                        mid_line.push_str(&format!(
                            "{}{}{}",
                            display.mid.0, display.mid.1, display.mid.2
                        ));
                        bot_line.push_str(&format!(
                            "{}{}{}",
                            display.bot.0, display.bot.1, display.bot.2
                        ));
                        curr_plot = plots_iter.next();
                    } else {
                        top_line.push_str("   ");
                        mid_line.push_str("   ");
                        bot_line.push_str("   ");
                    }
                }
            }
            let line = [top_line, mid_line, bot_line, "".into()].join("\n");
            output.push_str(&line);
            i = j;
        }
        write!(f, "{output}")
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
        let grid = Grid::new(EXAMPLE);
        let region = Region::map_out('R', (0, 0), &grid);
        assert_eq!(10, region.sides());
        let region = Region::map_out('R', (4, 0), &grid);
        assert_eq!(4, region.sides());
    }
}
