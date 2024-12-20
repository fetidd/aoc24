use std::fmt::Display;

use aoc24::utils;

fn main() {
    let input = include_str!("../../puzzle_input/day9.txt");
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let mut disk = Disk::new(input);
    for i in (0..disk.0.len()).rev() {
        if let Block::File { .. } = disk.0[i] {
            disk.move_file(i);
        }
    }
    println!("{}", disk);
    let disk = disk.flatten();
    // println!("{:?}", disk);
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

    fn id(&self) -> u32 {
        match self {
            Block::Free(_) => panic!("Free blocks have no id!"),
            Block::File { id, .. } => *id,
        }
    }

    fn shrink(&mut self, amount: usize) {
        match self {
            Block::Free(size) => *size -= amount,
            Block::File { size, .. } => *size -= amount,
        }
    }

    fn grow(&mut self, amount: usize) {
        match self {
            Block::Free(size) => *size += amount,
            Block::File { size, .. } => *size += amount,
        }
    }

    fn write(&mut self, id: u32) {
        if let Block::Free(size) = self {
            *self = Block::File { id, size: *size };
        }
    }

    fn free(&mut self) {
        if let Block::File { size, .. } = self {
            *self = Block::Free(*size);
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Free(size) => write!(f, "[{}]", ".".repeat(*size)),
            Block::File { id, size } => write!(f, "[{}]", id.to_string().repeat(*size)),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Disk(Vec<Block>);

impl Disk {
    fn new(input: &str) -> Self {
        Disk(
            input
                .chars()
                .filter(|ch| *ch != '\n')
                .enumerate()
                .map(Block::from_indexed_char)
                .collect(),
        )
    }

    fn move_file(&mut self, file: usize) {
        let mut i = 0;
        loop {
            // println!("loop");
            if let Some(free) = self.find_free(i, file) {
                let file_size = self.0[file].size();
                let free_size = self.0[free].size();
                // if free_size < file_size {
                //     // println!("1 {}", self);
                //     let file_id = self.0[file].id();
                //     self.0[free].write(file_id);
                //     // println!("1 {}", self);
                //     self.0[file].shrink(free_size);
                //     // println!("1 {}", self);
                //     self.0.insert(file + 1, Block::Free(free_size));
                //     // println!("1 {}", self);
                if free_size == file_size {
                    println!("2 {}", self);
                    self.0.swap(free, file);
                    println!("2 {}", self);
                    break; // whole file moved so we're done
                } else if free_size > file_size {
                    // the whole file chunk we're moving can sit in this free
                    println!("3 {}", self);
                    let file_id = self.0[file].id();
                    self.0[free].write(file_id);
                    self.0[free].shrink(free_size - file_size);
                    self.0.insert(free + 1, Block::Free(free_size - file_size));
                    self.0.remove(file + 1);
                    println!("3 {}", self);
                    break; // whole file moved so we're done
                } else {
                    break;
                }
            } else {
                // no free space to move file chunks to
                // println!("break");
                if i >= self.0.len() {
                    break;
                } else {
                    i += 1;
                }
            }
        }
    }

    fn find_free(&self, start: usize, bound: usize) -> Option<usize> {
        for i in start..std::cmp::min(bound, self.0.len()) {
            if let Block::Free(_) = self.0[i] {
                return Some(i);
            }
        }
        None
    }

    fn flatten(&mut self) -> Vec<usize> {
        let mut f = vec![];
        for i in 0..self.0.len() {
            if let Block::File { id, size } = self.0[i] {
                f.extend(vec![id as usize; size]);
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
            self.0.iter().map(|x| x.to_string()).collect::<String>()
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

    #[ignore]
    #[test]
    fn blocks_move_correctly() {
        let mut disk = Disk::new("4221342");
        assert_eq!(disk.to_string(), String::from("0000..11.222....33"));
        disk.move_file(6);
        assert_eq!(disk.to_string(), String::from("00003311.222......"));
        disk.move_file(4);
        assert_eq!(disk.to_string(), String::from("00003311222......."));
        disk.move_file(2);
    }
}
