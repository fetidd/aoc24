#[derive(Debug, PartialEq)]
pub struct Grid {
    pub tiles: Vec<Vec<char>>,
    pub rows: usize,
    pub cols: usize
}

pub struct Point{pub x: i32, pub y: i32}

impl Grid {
    pub fn new(input: &str) -> Grid {
        let tiles: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows = tiles[0].len();
        let cols = tiles.len();
        Grid {
            tiles, rows, cols
        }
    }

    pub fn check_oob(&self, point: &Point) -> bool {
        point.x >= (self.cols as i32) || point.y >= (self.rows as i32) || point.x <= 0 || point.y <= 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let input = ".....\n.....\n.....\n.....\n.....";
        let actual = Grid::new(&input);
        let expected = Grid {
            tiles: vec![vec!['.','.','.','.','.'],vec!['.','.','.','.','.'],vec!['.','.','.','.','.'],vec!['.','.','.','.','.'],vec!['.','.','.','.','.'],],
            rows: 5_usize,
            cols: 5_usize
        };
        assert_eq!(expected, actual);
    }
}
