use std::collections::HashSet;

use aoc24::utils;

fn main() {
    let input = include_str!("../../puzzle_input/day6.txt");
    let result = process(&input);
    println!("{}", result);
}


#[derive(Debug, PartialEq)]
pub struct Grid {
    pub tiles: Vec<Vec<char>>,
    pub rows: usize,
    pub cols: usize,
}

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Grid {
    pub fn new(input: &str) -> Grid {
        let tiles: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows = tiles[0].len();
        let cols = tiles.len();
        Grid { tiles, rows, cols }
    }

    pub fn get_next_pos(
        &self,
        (curr_row, curr_col): (usize, usize),
        direction: &mut Dir,
    ) -> Option<(usize, usize)> {
        let (next_row, next_col) = match direction {
            Dir::Up => (curr_row.checked_sub(1)?, curr_col),
            Dir::Right => (curr_row, curr_col + 1),
            Dir::Down => (curr_row + 1, curr_col),
            Dir::Left => (curr_row, curr_col.checked_sub(1)?),
        };

        let char = self.tiles.get(next_row).and_then(|row| row.get(next_col))?;

        if char == &'#' {
            direction.turn_right();
            return Some((curr_row, curr_col));
        }

        Some((next_row, next_col))
    }
}
fn process(input: &str) -> String {
    let mut grid = Grid::new(input);
    let (mut guard_row, mut guard_col) = find_guard(&grid);
    let mut direction = Dir::Up;

    let mut visited = HashSet::new();
    let mut count = 0;

    while let Some((next_row, next_col)) = grid.get_next_pos((guard_row, guard_col), &mut direction)
    {
        visited.insert((guard_row, guard_col));

        if !visited.contains(&(next_row, next_col)) {
            grid.tiles[next_row][next_col] = '#';
            if gets_in_loop(&grid, (guard_row, guard_col), direction) {
                count += 1;
            }
            grid.tiles[next_row][next_col] = '.';
        }

        (guard_row, guard_col) = (next_row, next_col);
    }

    count.to_string()
}

fn gets_in_loop(
    grid: &Grid,
    (start_row, start_col): (usize, usize),
    start_direction: Dir,
) -> bool {
    // only need to keep track of the times the guard turned
    // if the guard made the same turn at the same obstacle twice then we have a cycle
    let mut visited_obstacles: Vec<(usize, usize, Dir)> = Vec::new();

    let mut direction = start_direction;
    let (mut guard_row, mut guard_col) = (start_row, start_col);

    while let Some((next_row, next_col)) = grid.get_next_pos((guard_row, guard_col), &mut direction)
    {
        if (guard_row, guard_col) == (next_row, next_col) {
            if visited_obstacles.contains(&(guard_row, guard_col, direction)) {
                return true;
            }

            visited_obstacles.push((guard_row, guard_col, direction));
        }

        (guard_row, guard_col) = (next_row, next_col);
    }

    false
}

// We can just panic if there's no guard!
fn find_guard(grid: &Grid) -> (usize, usize) {
    for (i, row) in grid.tiles.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if ['^', 'v', '<', '>'].contains(ch) {
                return (i, j);
            }
        }
    }
    panic!("failed to find a guard!");
}

fn next_dir(curr: &Dir) -> Dir {
    match *curr {
        Dir::Up => Dir::Right,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn_right(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        };
    }
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!("bad direction!"),
        }
    }
}
