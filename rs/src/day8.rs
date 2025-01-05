use std::{collections::HashSet, fmt::Display};
mod utils;
use utils::grid::{Grid, GridPos};

fn main() {
    let input = include_str!("../../puzzle_input/day8.txt");
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let grid: Grid<Tile> = Grid::new(input);
    let mut antinodes: HashSet<GridPos> = HashSet::new();
    for (pos, _) in grid.iter() {
        if let Some(Tile::Node(node)) = grid.at(pos) {
            antinodes.insert(pos);
            for other in grid.find_other_nodes(*node, pos) {
                antinodes.extend(grid.find_antinodes(pos, other));
            }
        }
    }
    antinodes.len().to_string()
}

impl Grid<Tile> {
    fn find_other_nodes(&self, node: char, from: GridPos) -> Vec<GridPos> {
        let mut nodes = vec![];
        for (pos, _) in self {
            if pos != from {
                if let Some(Tile::Node(other)) = self.at(pos) {
                    if *other == node {
                        nodes.push(pos);
                    }
                }
            }
        }
        nodes
    }

    fn find_antinodes(&self, (ax, ay): GridPos, (bx, by): GridPos) -> Vec<GridPos> {
        let (ax, ay, bx, by) = (ax as i32, ay as i32, bx as i32, by as i32);
        let (vx, vy) = (bx as i32 - ax as i32, by as i32 - ay as i32);
        let mut antinodes = vec![];
        // find antinodes before
        let (mut curr_x, mut curr_y) = (ax - vx, ay - vy);
        while self.in_bounds_i32((curr_x, curr_y)) {
            antinodes.push((curr_x as usize, curr_y as usize));
            curr_x -= vx;
            curr_y -= vy;
        }
        // find antinodes after
        let (mut curr_x, mut curr_y) = (bx + vx, by + vy);
        while self.in_bounds_i32((curr_x, curr_y)) {
            antinodes.push((curr_x as usize, curr_y as usize));
            curr_x += vx;
            curr_y += vy;
        }
        antinodes
    }
}

#[derive(Debug)]
enum Tile {
    Node(char),
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Node(n) => n.to_string(),
                // Tile::Antinode(n) => n.to_string(),
                Tile::Empty => ".".to_string(),
            }
        )
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' | '*' => Self::Empty,
            'a'..='z' | 'A'..='Z' | '0'..='9' => Self::Node(value),
            _ => panic!("invalid char"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &'static str = "............\n\
									........B...\n\
									.....B......\n\
									.......B....\n\
									....B.......\n\
									......A.....\n\
									............\n\
									............\n\
									........A...\n\
									.........A..\n\
									............\n\
									............";

    #[test]
    fn test_process() {
        assert_eq!(34.to_string(), process(EXAMPLE));
    }
}
