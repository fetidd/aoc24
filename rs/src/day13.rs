use aoc24::utils;
use itertools::Itertools;

fn main() {
    let input = include_str!("../../puzzle_input/day13.txt");
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let mut machines: Vec<Machine> = vec![];
    let mut lines = input.lines();
    loop {
        let machine: Vec<&str> = (&mut lines).take(3).collect();
        machines.push(Machine::from(&machine));
        if let None = lines.next() {
            break;
        }
    }
    machines
        .into_iter()
        .map(|m| m.solve())
        .sum::<usize>()
        .to_string()
}

#[derive(Debug, Clone, Copy)]
struct Button {
    x: usize,
    y: usize,
}

impl Button {
    fn from(input: &str) -> Self {
        let input: String = input
            .chars()
            .filter(|c| c.is_digit(10) || *c == ',' || *c == '-')
            .collect();
        let mut mods = input.split(',');
        Self {
            x: mods.next().unwrap().parse::<usize>().unwrap(),
            y: mods.next().unwrap().parse::<usize>().unwrap(),
        }
    }

    fn mult(&self, n: usize) -> (usize, usize) {
        (self.x * n, self.y * n)
    }

    fn modulo(&self, n: usize) -> (usize, usize) {
        (self.x % n, self.y % n)
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let value = value
            .chars()
            .filter(|v| v.is_digit(10) || *v == ',' || *v == '-')
            .collect::<String>();
        let mut split = value.split(',');
        Self {
            x: split.next().unwrap().parse::<usize>().unwrap(),
            y: split.next().unwrap().parse::<usize>().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: Button,
    b: Button,
    prize: Point,
}

impl Machine {
    fn from(input: &[&str]) -> Self {
        let a = Button::from(input[0]);
        let b = Button::from(input[1]);
        let prize = Point::from(input[2]);
        Self { a, b, prize }
    }

    fn solve(&self) -> usize {
        let mut a_sub = 0;
        if 
        loop {
            let a_presses = 
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Button A: X+94, Y+34\n\
						   Button B: X+22, Y+67\n\
						   Prize: X=8400, Y=5400\n\
						   \n\
						   Button A: X+26, Y+66\n\
						   Button B: X+67, Y+21\n\
						   Prize: X=12748, Y=12176\n\
						   \n\
						   Button A: X+17, Y+86\n\
						   Button B: X+84, Y+37\n\
						   Prize: X=7870, Y=6450\n\
						   \n\
						   Button A: X+69, Y+23\n\
						   Button B: X+27, Y+71\n\
						   Prize: X=18641, Y=10279";

    #[test]
    fn test_process() {
        assert_eq!("480".to_string(), process(EXAMPLE));
    }
}
