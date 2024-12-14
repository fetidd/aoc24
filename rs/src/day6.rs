use std::{collections::HashSet, rc::Rc};

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
    let mut visited = HashSet::new();
    for i in 0..input.len() {
        let g = match input.chars().nth(i) {
            Some(dir @ ('^' | '>' | 'v' | '<')) => Some(Guard::new(dir, i)),
            Some(_) => None,
            None => None,
        };
        if let Some(g) = g {
            // run guard path
            let new_visited = run_guard(g, (&input, rows, cols));
            visited.extend(new_visited);
        }
    }
    visited.len().to_string()
}

fn run_guard(mut guard: Guard, (grid, _rows, cols): (&str, usize, usize)) -> HashSet<usize> {
    let mut visited = HashSet::new();
    visited.insert(guard.idx);
    let next_n = Rc::new(|i: usize| (i as i32) - (cols as i32) - 1);
    let next_e = Rc::new(|i: usize| (i as i32) + 1);
    let next_s = Rc::new(|i: usize| (i as i32) + (cols as i32) + 1);
    let next_w = Rc::new(|i: usize| (i as i32) - 1);
    let out_n = Rc::new(|i: i32| i < 0);
    let out_s = Rc::new(|i: i32| i > (grid.len() as i32));
    let out_we = Rc::new(|i: i32| grid.chars().nth(i as usize).expect("bad idx") == '\n');
    loop {
        let (get_next, will_leave): (Rc<dyn Fn(usize) -> i32>, Rc<dyn Fn(i32) -> bool>) =
            match guard.dir {
                Direction::N => (next_n.clone(), out_n.clone()),
                Direction::E => (next_e.clone(), out_we.clone()),
                Direction::S => (next_s.clone(), out_s.clone()),
                Direction::W => (next_w.clone(), out_we.clone()),
            };
        let next_i: i32 = get_next(guard.idx);
        if will_leave(next_i) {
            return visited;
        } else if grid.chars().nth(next_i as usize).expect("bad idx") == '#' {
            guard.dir = next_dir(&guard.dir);
        } else {
            guard.idx = next_i as usize;
            visited.insert(guard.idx);
        }
    }
}

fn next_dir(curr: &Direction) -> Direction {
    match *curr {
        Direction::N => Direction::E,
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
    }
}

#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl From<char> for Direction {
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

struct Guard {
    idx: usize,
    dir: Direction,
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

    // static SOLVED: &'static str = "....#.....\n\
    //                                ....XXXXX#\n\
    //                                ....X...X.\n\
    //                                ..#.X...X.\n\
    //                                ..XXXXX#X.\n\
    //                                ..X.X.X.X.\n\
    //                                .#XXXXXXX.\n\
    //                                .XXXXXXX#.\n\
    //                                #XXXXXXX..\n\
    //                                ......#X..";

    #[test]
    fn test_process() {
        assert_eq!("41".to_string(), process(EXAMPLE));
    }
}
