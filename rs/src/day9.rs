use std::fmt::Display;

use aoc24::utils;

fn main() {
    let input = include_str!("../../puzzle_input/day9.txt");
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let mut disk: Disk = Disk(
        input
            .chars()
            .enumerate()
            .map(Block::from_indexed_char)
            .collect(),
    );
    println!(
        "{}",
        disk.0
            .into_iter()
            .map(|x| x.to_string())
            .collect::<String>()
    );
    "todo".into()
}

#[derive(Debug, PartialEq)]
enum Block {
    Free { size: usize },
    File { id: u32, size: usize },
}

impl Block {
    fn from_indexed_char((i, ch): (usize, char)) -> Self {
        let size = ch.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            let id = (i as u32) / 2;
            Self::File { id, size }
        } else {
            Self::Free { size }
        }
    }

    fn size(&self) -> usize {
        match self {
            Block::Free { size } => *size,
            Block::File { size, .. } => *size,
        }
    }
}

struct Disk(Vec<Block>);

impl Disk {
    fn move_file(&mut self, loc: usize) {
        let mut file = &mut self.0[loc];
        while file.size() > 0 {
            if let Some((i, &mut free)) = self.find_free() {
                if free.size() < file.size() {
                    // free is replaced by new file with same id as file, file shrunk
                } else if free.size() == file.size() {
                    // free swaps with file
                } else {
                    // free squashed behind new_file, file becomes free
                }
            }
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Free { size } => write!(f, "{}", ".".repeat(*size)),
            Block::File { id, size } => write!(f, "{}", id.to_string().repeat(*size)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!("1928".to_string(), process("2333133121414131402"));
    }
}
