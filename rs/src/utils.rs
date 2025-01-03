use std::str::FromStr;

pub fn parse_space_list<T: FromStr>(list: &str) -> Result<Vec<T>, String> {
    list
        .split(' ')
        .map(|x| x.trim_start().trim_end())
        .filter(|x| *x != "")
        .map(|x| x.parse::<T>())
        .collect::<Result<Vec<T>, <T as FromStr>::Err>>()
        .map_err(|_| format!("failed to parse '{list}'"))
}

#[derive(Debug)]
pub struct Grid<Tile: From<char>> {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
    pub cursor: (usize, usize)
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
            cursor: (0, 0)
        }
    }

    pub fn at(&self, (x, y): (usize, usize)) -> Option<&Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&self.tiles[y][x])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_space_list() {
        let tests = vec![
            ("", Ok(vec![])),
            ("1 2 3 4 5", Ok(vec![1, 2, 3, 4, 5])),
            (" 1 2   3 4  5 ", Ok(vec![1, 2, 3, 4, 5])),
            ("1 2 3 b 5", Err("failed to parse '1 2 3 b 5'".to_string())),
        ];
        for (input, expected) in tests {
            assert_eq!(expected, parse_space_list::<i32>(input));
        }
        
        let tests = vec![
            ("", Ok(vec![])),
            ("a b c d e", Ok(vec!['a', 'b', 'c', 'd', 'e'])),
            (" a   b c  d e ", Ok(vec!['a', 'b', 'c', 'd', 'e'])),
        ];
        for (input, expected) in tests {
            assert_eq!(expected, parse_space_list::<char>(input));
        }
    }

    #[test]
    fn test_grid() {
        let input = "abcde\n\
                     fghij\n\
                     klmno";
        
        let mut grid: Grid<char> = Grid::new(&input);
    }

}
