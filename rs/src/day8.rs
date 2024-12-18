use std::{collections::HashSet, fmt::Display};

// use aoc24::utils;

fn main() {
    let input = include_str!("../../puzzle_input/day8.txt");
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let mut grid = Grid::new(input);
    let mut antinodes = HashSet::new();
    for x in 0..grid.width {
        for y in 0..grid.height {
            if let Tile::Node(node) = grid.tiles[y][x] {
                for other in grid.find_other_nodes(node, (x, y)) {
                    // calculate vector
                    let vector = (other.0.abs_diff(x), other.1.abs_diff(y));
                    // add antinode(s) to set
                    let antinode_1 = (x.checked_sub(vector.0), y.checked_sub(vector.1));
                    if antinode_1.0.is_some() && antinode_1.1.is_some() {
                        antinodes.insert((antinode_1.0.unwrap(), antinode_1.1.unwrap()));
                        if let Tile::Empty =
                            grid.tiles[antinode_1.1.unwrap()][antinode_1.0.unwrap()]
                        {
                            grid.tiles[antinode_1.1.unwrap()][antinode_1.0.unwrap()] =
                                Tile::Antinode(node.to_ascii_lowercase());
                        }
                    }
                    let antinode_2 = (other.0 + vector.0, other.1 + vector.1);
                    if antinode_2.0 < grid.width && antinode_2.1 < grid.height {
                        antinodes.insert(antinode_2);
                        if let Tile::Empty = grid.tiles[antinode_2.1][antinode_2.0] {
                            grid.tiles[antinode_2.1][antinode_2.0] =
                                Tile::Antinode(node.to_ascii_lowercase());
                        }
                    }
                }
            }
        }
    }
    // dbg!(&antinodes);
    println!(
        "{}",
        grid.tiles
            .into_iter()
            .map(|l| l.into_iter().map(|t| t.to_string()).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
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
        dbg!(nodes)
    }
}

#[derive(Debug)]
enum Tile {
    Node(char),
    Antinode(char),
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Node(n) => n.to_string(),
                Tile::Antinode(n) => n.to_string(),
                Tile::Empty => ".".to_string(),
            }
        )
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' | '*' => Self::Empty,
            'a'..'z' | 'A'..'Z' | '0'..'9' => Self::Node(value),
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
    static EXAMPLE_SOLVED: &'static str = "......b....b\n\
        								   ...a....B...\n\
        								   ....aB....b.\n\
        								   ..b....B....\n\
        								   ....B....b..\n\
        								   .b....A.....\n\
        								   ...b........\n\
        								   b......a....\n\
        								   ........A...\n\
        								   .........A..\n\
        								   ..........a.\n\
        								   ..........a.";

    #[test]
    fn test_process() {
        assert_eq!(14.to_string(), process(EXAMPLE));
    }
}
