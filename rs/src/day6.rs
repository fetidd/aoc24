use std::collections::HashSet;

// use aoc24::utils;

fn main() {
    let input = include_bytes!("../../puzzle_input/day6.txt");
    let input = String::from_utf8_lossy(input);
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let cols = input
        .chars()
        .position(|ch| ch == '\n')
        .expect("input is single line!");
    let rows = input.len() / cols;
    let mut obstacles = HashSet::new();
    for i in 0..input.len() {
        let g = match input.chars().nth(i) {
            Some(dir @ ('^' | '>' | 'v' | '<')) => Some(Guard::new(dir, i)),
            _ => None,
        };
        if let Some(g) = g {
            let (visited, _) = run_guard(g, (&input, rows, cols), None);
            // let new_grid: String = input
            //     .char_indices()
            //     .map(|(i, ch)| {
            //         if visited
            //             .iter()
            //             .map(|(j, _)| *j)
            //             .collect::<HashSet<usize>>()
            //             .contains(&i)
            //         {
            //             'X'
            //         } else {
            //             ch
            //         }
            //     })
            //     .collect();
            // println!("\n{}", new_grid);
            for v in visited {
                let obst = match v.1 {
                    Dir::N => (v.0 as i32) - (cols as i32) - 1,
                    Dir::E => (v.0 as i32) + 1,
                    Dir::S => (v.0 as i32) + (cols as i32) + 1,
                    Dir::W => (v.0 as i32) - 1,
                };
                if obst >= 0 && obst < input.len() as i32 && input.chars().nth(i).unwrap() != '\n' {
                    if let (_, true) = run_guard(g, (&input, rows, cols), Some(obst as usize)) {
                        obstacles.insert(obst);
                    }
                }
            }
        }
    }
    let new_grid: String = input
        .char_indices()
        .map(|(i, ch)| {
            if obstacles.contains(&(i as i32)) {
                'O'
            } else {
                ch
            }
        })
        .collect();
    println!("\n{}", new_grid);
    obstacles.len().to_string()
}

fn run_guard(
    mut guard: Guard,
    (grid, _rows, cols): (&str, usize, usize),
    obst: Option<usize>,
) -> (HashSet<(usize, Dir)>, bool) {
    let mut visited = HashSet::from([(guard.idx, guard.dir)]);
    loop {
        let next_i: i32 = match guard.dir {
            Dir::N => (guard.idx as i32) - (cols as i32) - 1,
            Dir::E => (guard.idx as i32) + 1,
            Dir::S => (guard.idx as i32) + (cols as i32) + 1,
            Dir::W => (guard.idx as i32) - 1,
        };
        if visited.contains(&(next_i as usize, guard.dir)) {
            return (visited, true);
        };
        if match guard.dir {
            Dir::N => next_i < 0,
            Dir::S => next_i >= (grid.len() as i32),
            Dir::W | Dir::E => grid.chars().nth(next_i as usize).expect("bad idx") == '\n',
        } {
            return (visited, false); // left the grid
        } else if grid.chars().nth(next_i as usize).expect("bad idx") == '#' // hit a real obstacle
            || obst.is_some_and(|x| x == (next_i as usize))
        // hit a potential obstacle
        {
            guard.dir = next_dir(&guard.dir);
        } else {
            visited.insert((next_i as usize, guard.dir));
            guard.idx = next_i as usize;
        }
    }
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

#[derive(Clone, Copy)]
struct Guard {
    idx: usize,
    dir: Dir,
}

impl Guard {
    fn new(dir: char, idx: usize) -> Self {
        Self {
            dir: dir.into(),
            idx,
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
        println!("{}", EXAMPLE);
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
