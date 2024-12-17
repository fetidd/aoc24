use std::collections::HashSet;

use aoc24::utils::{Grid, Point};

fn main() {
    let input = include_str!("../../puzzle_input/day6.txt");
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let mut grid = Grid::new(&input);
    let mut guard = find_guard(&grid);
    let mut route = HashSet::from([(guard.col, guard.row, guard.dir)]);
    while let Ok(()) = move_guard(&mut guard, &grid) {
        route.insert((guard.col, guard.row, guard.dir));
    }
    let mut obstacles = HashSet::new();
    for r in route {
        let mut guard = find_guard(&grid);
        let mut visited = HashSet::from([(guard.col, guard.row, guard.dir)]);
        let obst = match r.2 {
            Dir::N => Point {
                x: r.0 as i32,
                y: (r.1 as i32) - 1,
            },
            Dir::E => Point {
                x: (r.0 as i32) + 1,
                y: r.1 as i32,
            },
            Dir::S => Point {
                x: r.0 as i32,
                y: (r.1 as i32) + 1,
            },
            Dir::W => Point {
                x: (r.0 as i32) - 1,
                y: r.1 as i32,
            },
        };
        if !grid.check_oob(&obst) {
            let prev = grid.tiles[obst.y as usize][obst.x as usize];
            grid.tiles[obst.y as usize][obst.x as usize] = '#';
            while let Ok(()) = move_guard(&mut guard, &grid) {
                if visited.contains(&(guard.col, guard.row, guard.dir)) {
                    obstacles.insert((guard.col, guard.row));
                    break;
                }
                visited.insert((guard.col, guard.row, guard.dir));
            }
            grid.tiles[obst.y as usize][obst.x as usize] = prev;
        }
    }
    obstacles.len().to_string()
    // route.len().to_string()
}

// We can just panic if there's no guard!
fn find_guard(grid: &Grid) -> Guard {
    for (i, row) in grid.tiles.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if ['^', 'v', '<', '>'].contains(ch) {
                return Guard::new(*ch, i, j);
            }
        }
    }
    panic!("failed to find a guard!");
}

fn move_guard(guard: &mut Guard, grid: &Grid) -> Result<(), String> {
    let mut next = Point {
        x: guard.col as i32,
        y: guard.row as i32,
    };
    loop {
        match guard.dir {
            Dir::N => next.y = (guard.row as i32) - 1,
            Dir::E => next.x = (guard.col as i32) + 1,
            Dir::S => next.y = (guard.row as i32) + 1,
            Dir::W => next.x = (guard.col as i32) - 1,
        };
        if grid.check_oob(&next) {
            return Err("Out of bounds!".to_string());
        } else if grid.tiles[next.y as usize][next.x as usize] == '#' {
            guard.dir = next_dir(&guard.dir);
            next = Point {
                x: guard.col as i32,
                y: guard.row as i32,
            }
        } else {
            break;
        }
    }
    guard.col = next.x as usize;
    guard.row = next.y as usize;
    Ok(())
}

fn next_dir(curr: &Dir) -> Dir {
    match *curr {
        Dir::N => Dir::E,
        Dir::E => Dir::S,
        Dir::S => Dir::W,
        Dir::W => Dir::N,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::N,
            '>' => Self::E,
            'v' => Self::S,
            '<' => Self::W,
            _ => panic!("bad direction!"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Guard {
    row: usize,
    col: usize,
    dir: Dir,
}

impl Guard {
    fn new(dir: char, row: usize, col: usize) -> Self {
        Self {
            dir: dir.into(),
            row,
            col,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &'static str = "....#.....\n\
									.........#\n\
									..........\n\
									..#.......\n\
									.......#..\n\
									..........\n\
									.#..^.....\n\
									........#.\n\
									#.........\n\
									......#...";

    #[test]
    fn test_process() {
        // assert_eq!("41".to_string(), process(EXAMPLE));
        assert_eq!("6".to_string(), process(EXAMPLE));

        let tests = vec![
            (".#...\n....#\n.....\n.....\n.^.#.", 1),
            (".....\n>...#\n.....\n#....\n...#.", 1),
            (".#.v.\n.....\n.....\n#....\n...#.", 1),
            (".#...\n.....\n.....\n#...<\n...#.", 1),
            (
                ".#.v.\n\
                 .....\n\
                 .....\n\
                 #....\n\
                 ...#.",
                1,
            ),
        ];
        for (input, expected) in tests {
            assert_eq!(expected.to_string(), process(input));
        }
    }
}
