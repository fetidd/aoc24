use std::{collections::HashSet, fmt::Display};

// use aoc24::utils;

fn main() {
    let input = include_str!("../../puzzle_input/day8.txt");
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let grid = Grid::new(input);
    let mut antinodes = HashSet::new();
    for x in 0..grid.width {
        for y in 0..grid.height {
            if let Tile::Node(node) = grid.tiles[y][x] {
                antinodes.insert((x as i32, y as i32));
                for other in grid.find_other_nodes(node, (x, y)) {
                    antinodes.extend(grid.find_antinodes((x, y), other));
                }
            }
        }
    }
    // dbg!(&antinodes);
    // println!(
    //     "{}",
    //     grid.tiles
    //         .into_iter()
    //         .map(|l| l.into_iter().map(|t| t.to_string()).collect::<String>())
    //         .collect::<Vec<String>>()
    //         .join("\n")
    // );
    antinodes.len().to_string()
}


struct Grid {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let tiles: Vec<Vec<Tile>> = input
            .lines()
            .map(|l| l.chars().map(Tile::from).collect())
            .collect();
        let height = tiles.len();
        let width = tiles[0].len();
        Self {
            tiles,
            height,
            width,
        }
    }

    fn find_other_nodes(&self, node: char, from: (usize, usize)) -> Vec<(usize, usize)> {
        let mut nodes = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                if (x, y) != from {
                    if let Tile::Node(other) = self.tiles[y][x] {
                        if other == node {
                            nodes.push((x, y));
                        }
                    }
                }
            }
        }
        // dbg!(nodes)
        nodes
    }
    
    fn find_antinodes(&self, (ax, ay): (usize, usize), (bx, by): (usize, usize)) -> Vec<(i32, i32)> {
        let (ax, ay, bx, by) = (ax as i32, ay as i32, bx as i32, by as i32);
        let (vx, vy) = (bx as i32 - ax as i32, by as i32 - ay as i32);
        let mut antinodes = vec![];
        // find antinodes before
        let (mut curr_x, mut curr_y) = (ax - vx, ay - vy);
        while self.in_bounds((curr_x, curr_y)) {
            antinodes.push((curr_x, curr_y));
            curr_x -= vx;
            curr_y -= vy;
        }
        // find antinodes after
        let (mut curr_x, mut curr_y) = (bx + vx, by + vy);
        while self.in_bounds((curr_x, curr_y)) {
            antinodes.push((curr_x, curr_y));
            curr_x += vx;
            curr_y += vy;
        }
        antinodes
    }

    fn in_bounds(&self, (x, y): (i32, i32)) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
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
