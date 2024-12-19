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
    println!("{}", disk);
    "todo".into()
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
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Free(size) => write!(f, "[{}]", "-".repeat(*size)),
            Block::File { id, size } => write!(f, "[{}]", id.to_string().repeat(*size)),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Disk(Vec<Block>);

impl Disk {
    fn move_file(&mut self, file: usize) {
        loop {
            if let Some(free) = self.find_free() {
                if self.0[free].size() < self.0[file].size() {
                    // [66666][---][77777][8888] -> [66666][888][77777][8][---]
                    let new_file = Block::File { id: self.0[file].id(), size: self.0[free].size() }; // [888]
                    let free_size = self.0[free].size();
                    self.0[file].shrink(free_size); // [8888] - [---] = [8]
                    self.0[free] = new_file; // [66666][888][77777][8]
                    self.0.insert(free, Block::Free(free_size));
                } else if self.0[free].size() == self.0[file].size() {
                    // [66666][---][77777][888] -> [66666][888][77777][---]
                    self.0.swap(file, free);
                    if file < self.0.len() - 1 {
                        let mut grow_amount = 0;
                        if let (Block::Free(_), Block::Free(b)) = (&self.0[file], &self.0[file + 1]) {
                            grow_amount = *b;
                        }
                        if grow_amount > 0 {
                            self.0.remove(file + 1);
                            self.0[file].grow(grow_amount);
                        }
                    }
                    break; // whole file moved so we're done
                } else {
                    // [66666][----][77777][888] -> [66666][888][-][77777][---]
                    let file_size = self.0[file].size();
                    let file_block = self.0.remove(file);
                    self.0[free].shrink(file_size);
                    self.0.insert(free, file_block);
                    self.0.insert(file, Block::Free(file_size));
                    break; // whole file moved so we're done
                }
            } else { // no free space to move file chunks to
                break;
            }
        }
    }

    fn find_free(&self) -> Option<usize> {
        self.0.iter().enumerate().skip_while(|x| match x {(_, Block::Free{..}) => false, _ => true}).map(|x| x.0).take(1).last()
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0
            .iter()
            .map(|x| x.to_string())
            .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!("1928".to_string(), process("2333133121414131402"));
    }

    #[test]
    fn blocks_move_correctly() {
        let disk = Disk(vec![Block::File{id: 0, size: 4},Block::Free(2),Block::File{id: 1, size: 2},Block::Free(1),Block::File{id: 2, size: 3},Block::Free(4),Block::File{id: 3, size: 5}]);
        assert_eq!(disk.to_string(), String::from("[0000][--][11][-][222][----][33333]"));

        let mut test_disk = disk.clone();
        test_disk.move_file(2);
        assert_eq!(test_disk.to_string(), String::from("[0000][11][---][222][----][33333]"));

        let mut test_disk = disk.clone();
        test_disk.move_file(4);
        assert_eq!(test_disk.to_string(), String::from("[0000][22][11][2][-------][33333]"));

        let mut test_disk = disk.clone();
        test_disk.move_file(6);
        assert_eq!(test_disk.to_string(), String::from("[0000][33][11][3][222][33][-------]"));
    }
}
