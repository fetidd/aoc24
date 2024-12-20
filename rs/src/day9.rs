use std::{collections::HashSet, fmt::Display, io::Write};

fn main() {
    let input = include_str!("../../puzzle_input/day9.txt");
    let result = process(&input);
    println!("{result}");
}

fn process(input: &str) -> String {
    let mut disk = Disk::new(input);
    let mut moved = HashSet::new();
    for i in (0..disk.blocks.len()).rev() {
        if let Block::File { id, .. } = disk.blocks[i] {
            if !moved.contains(&id) {
                disk.move_file(i);
                moved.insert(id);
            }
        }
    }
    if let Ok(mut file) = std::fs::File::create("./day9_output.txt") {
        file.write(&disk.to_string().as_bytes()).unwrap();
    }
    let disk = disk.flatten();
    disk.into_iter()
        .enumerate()
        .reduce(|(_, acc), (i, v)| (i, acc + (v * i)))
        .unwrap()
        .1
        .to_string()
}

#[derive(Debug, PartialEq, Clone)]
enum Block {
    Free(usize),
    File { id: u32, size: usize },
}

impl Block {
    fn from_indexed_char((i, ch): (usize, char)) -> Self {
        let size = ch.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            let id = (i as u32) / 2;
            Self::File { id, size }
        } else {
            Self::Free(size)
        }
    }

    fn size(&self) -> usize {
        match self {
            Block::Free(size) => *size,
            Block::File { size, .. } => *size,
        }
    }

    fn shrink(&mut self, amount: usize) {
        match self {
            Block::Free(size) => *size -= amount,
            Block::File { size, .. } => *size -= amount,
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Free(size) => write!(f, "{}", ".".repeat(*size)),
            Block::File { id, size } => write!(f, "{}", id.to_string().repeat(*size)),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Disk{
    blocks: Vec<Block>,
}

impl Disk {
    fn new(input: &str) -> Self {
        let mut blocks = vec![];
        let mut frees = vec![];
        for (i, ch) in input.chars().enumerate() {
            if ch != '\n' {
                let block = Block::from_indexed_char((i, ch));
                if let Block::Free(_) = block {
                    frees.push(i);
                }
                blocks.push(block);
            }
        }
        Disk { blocks }
    }

    fn move_file(&mut self, file: usize) {
        let file_size = self.blocks[file].size();
        if let Some(free) = self.find_free(file, Some(file_size)) {
            let free_size = self.blocks[free].size();
            let diff = free_size - file_size;
            self.blocks.swap(free, file);
            if diff > 0 {
                self.blocks[file].shrink(diff);
                self.blocks.insert(free + 1, Block::Free(diff));
            }
        }
    }

    fn find_free(&self, bound: usize, required: Option<usize>) -> Option<usize> {
        for i in 0..self.blocks.len() {
            if i > bound {
                return None;
            }
            if let Block::Free(size) = self.blocks[i] {
                if required.is_some_and(|req| req <= size) || required.is_none() {
                    return Some(i);
                }
            }
        }
        None
    }

    fn flatten(&mut self) -> Vec<usize> {
        let mut f = vec![];
        for i in 0..self.blocks.len() {
            match self.blocks[i] {
                Block::Free(size) => f.extend(vec![0; size]),
                Block::File { id, size } => f.extend(vec![id as usize; size]),
            }
        }
        f
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.blocks.iter().map(Block::to_string).collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!("2858".to_string(), process("2333133121414131402"));
    }
}
