use std::fmt::Debug;

use super::*;

#[derive(Debug)]
pub struct Grid<Tile: From<char>> {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl<Tile: From<char>> Grid<Tile> {
    pub fn new(input: &str) -> Self {
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

    pub fn cursor<'a>(&'a self) -> GridCursor<'a, Tile> {
        GridCursor {
            grid: self,
            pos: (0, 0),
        }
    }

    fn in_bounds(&self, (x, y): (usize, usize)) -> bool {
        x < self.width && y < self.height
    }

    pub fn in_bounds_i32(&self, (x, y): (i32, i32)) -> bool {
        // generics!
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }

    pub fn at(&self, (x, y): (usize, usize)) -> Option<&Tile> {
        if !self.in_bounds((x, y)) {
            return None;
        }
        Some(&self.tiles[y][x])
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn iter(&self) -> GridIter<Tile> {
        GridIter {
            grid: self,
            pos: (0, 0),
        }
    }

    pub fn iter_mut(&mut self) -> GridIter<Tile> {
        GridIter {
            grid: self,
            pos: (0, 0),
        }
    }
}

pub type GridPos = (usize, usize);

pub struct GridCursor<'grid, Tile: From<char>> {
    grid: &'grid Grid<Tile>,
    pos: GridPos,
}

impl<'grid, Tile: From<char>> GridCursor<'grid, Tile> {
    pub fn goto(&mut self, (x, y): GridPos) -> Result<(), String> {
        if self.grid.in_bounds((x, y)) {
            self.pos = (x, y);
            Ok(())
        } else {
            Err(String::from("Out of bounds!"))
        }
    }

    pub fn up(&mut self) -> Result<(), String> {
        let y = self.pos.1.checked_sub(1);
        if let None = y {
            return Err("Out of bounds!".into());
        }
        self.goto((self.pos.0, y.unwrap()))
    }

    pub fn right(&mut self) -> Result<(), String> {
        self.goto((self.pos.0 + 1, self.pos.1))
    }

    pub fn left(&mut self) -> Result<(), String> {
        let x = self.pos.0.checked_sub(1);
        if let None = x {
            return Err("Out of bounds!".into());
        }
        self.goto((x.unwrap(), self.pos.1))
    }

    pub fn down(&mut self) -> Result<(), String> {
        self.goto((self.pos.0, self.pos.1 + 1))
    }

    pub fn read(&self) -> Option<&'grid Tile> {
        self.grid.at(self.pos)
    }

    pub fn reset(&mut self) {
        self.pos = (0, 0);
    }

    pub fn follow(&mut self, route: &[Dir]) -> Result<(), String> {
        let curr = self.pos;
        for d in route {
            let res = match d {
                Dir::Up => self.up(),
                Dir::Left => self.left(),
                Dir::Down => self.down(),
                Dir::Right => self.right(),
            };
            if res.is_err() {
                self.goto(curr)
                    .expect("somehow trying to reset to out of bounds after follow failed...");
                return res;
            }
        }
        Ok(())
    }

    pub fn pos(&self) -> GridPos {
        self.pos
    }

    pub fn peek(&self, pos: GridPos) -> Option<(GridPos, &'grid Tile)> {
        self.grid.at(pos).map(|t| (pos, t))
    }

    pub fn north(&self) -> Option<(GridPos, &'grid Tile)> {
        match self.pos.1.checked_sub(1) {
            Some(y) => self.peek((self.pos.0, y)),
            None => None,
        }
    }

    pub fn east(&self) -> Option<(GridPos, &'grid Tile)> {
        match self.pos.0.checked_sub(1) {
            Some(x) => self.peek((x, self.pos.1)),
            None => None,
        }
    }

    pub fn west(&self) -> Option<(GridPos, &'grid Tile)> {
        self.peek((self.pos.0 + 1, self.pos.1))
    }

    pub fn south(&self) -> Option<(GridPos, &'grid Tile)> {
        self.peek((self.pos.0, self.pos.1 + 1))
    }

    pub fn next(&mut self) -> Option<(GridPos, &'grid Tile)> {
        if let Some(tile) = self.grid.at(self.pos) {
            let curr_pos = self.pos;
            if self.grid.in_bounds((curr_pos.0 + 1, curr_pos.1)) {
                self.pos = (curr_pos.0 + 1, curr_pos.1);
            } else if self.grid.in_bounds((curr_pos.0, curr_pos.1 + 1)) {
                self.pos = (0, curr_pos.1 + 1);
            } else {
                self.pos = (curr_pos.0 + 1, curr_pos.1);
            }
            Some((curr_pos, tile))
        } else {
            None
        }
    }
}

impl<'grid, Tile: From<char>> IntoIterator for &'grid Grid<Tile> {
    type Item = (GridPos, &'grid Tile);

    type IntoIter = GridIter<'grid, Tile>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            grid: self,
            pos: (0, 0),
        }
    }
}

pub struct GridIter<'grid, Tile: From<char>> {
    grid: &'grid Grid<Tile>,
    pos: GridPos,
}

impl<'grid, Tile: From<char>> Iterator for GridIter<'grid, Tile> {
    type Item = (GridPos, &'grid Tile);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tile) = self.grid.at(self.pos) {
            let curr_pos = self.pos;
            if self.grid.in_bounds((curr_pos.0 + 1, curr_pos.1)) {
                self.pos = (curr_pos.0 + 1, curr_pos.1);
            } else if self.grid.in_bounds((curr_pos.0, curr_pos.1 + 1)) {
                self.pos = (0, curr_pos.1 + 1);
            } else {
                self.pos = (curr_pos.0 + 1, curr_pos.1);
            }
            Some((curr_pos, tile))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Dir;
    use super::*;

    #[test]
    fn test_char_grid() {
        let input = "abcde\n\
                     fghij\n\
                     klmno";
        let grid: Grid<char> = Grid::new(&input);
        assert_eq!(Some(&'a'), grid.at((0, 0)));
        assert_eq!(Some(&'h'), grid.at((2, 1)));
        assert_eq!(Some(&'o'), grid.at((4, 2)));
        assert_eq!(None, grid.at((5, 2)));
        let mut cur = grid.cursor();
        cur.goto((4, 1)).unwrap();
        assert_eq!(&'j', cur.read().unwrap());
        cur.reset();
        assert_eq!(&'a', cur.read().unwrap());
        assert_eq!(Err(String::from("Out of bounds!")), cur.goto((5, 2)));
        cur.goto((2, 1)).unwrap(); // middle
        cur.up().unwrap();
        cur.right().unwrap();
        assert_eq!(&'d', cur.read().unwrap());
        cur.follow(&[Dir::Left, Dir::Left, Dir::Down]).unwrap();
        assert_eq!(&'g', cur.read().unwrap());
        cur.goto((4, 0)).unwrap();
        assert_eq!(&'e', cur.read().unwrap());
        cur.next();
        assert_eq!(&'f', cur.read().unwrap());
        assert_eq!((0, 1), cur.pos());
        cur.next();
        assert_eq!(&'g', cur.read().unwrap());
        cur.goto((4, 2)).unwrap();
        assert_eq!(Some(((4, 2), &'o')), cur.next());
        assert_eq!(None, cur.next());
        assert_eq!(None, cur.read());
    }

    #[test]
    fn test_grid_cursor_iteration() {
        let input = "abcde\n\
                     fghij\n\
                     klmno";
        let grid: Grid<char> = Grid::new(&input);
        assert_eq!(
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o'],
            grid.iter().map(|x| *x.1).collect::<Vec<char>>()
        );
    }
}
