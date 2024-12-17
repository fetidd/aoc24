use std::str::FromStr;

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

    pub fn check_oob(&self, point: &Point) -> bool {
        point.x >= (self.cols as i32) || point.y >= (self.rows as i32) || point.x < 0 || point.y < 0
    }
}

pub fn parse_space_list<T: FromStr>(list: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    Ok(list
        .split(' ')
        .map(|x| x.trim_start().trim_end())
        .filter(|x| *x != "")
        .map(|x| x.parse::<T>())
        .collect::<Result<Vec<T>, <T as FromStr>::Err>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let input = ".....\n.....\n.....\n.....\n.....";
        let actual = Grid::new(&input);
        let expected = Grid {
            tiles: vec![
                vec!['.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.'],
            ],
            rows: 5_usize,
            cols: 5_usize,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_oob() {
        let tests = vec![
            (Point { x: 0, y: -1 }, true),
            (Point { x: -1, y: 0 }, true),
            (Point { x: 0, y: 0 }, false),
            (Point { x: 2, y: 2 }, false),
            (Point { x: 3, y: 2 }, true),
            (Point { x: 2, y: 3 }, true),
        ];
        for (point, expected) in tests {
            let grid = Grid {
                tiles: vec![],
                cols: 3,
                rows: 3,
            };
            assert_eq!(expected, grid.check_oob(&point));
        }
    }
}
