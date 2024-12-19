use aoc24::utils;

fn main() {
    let input = include_str!("../../puzzle_input/day7.txt");
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let mut total = 0;
    let mut reducer_length = 0;
    let lines: Vec<(i64, Vec<i64>)> = input.lines().map(|line| {
        let mut s = line.split(':');
        let test_value: i64 = s.next().unwrap().parse().unwrap();
        let equation: Vec<i64> = utils::parse_space_list(&s.collect::<String>()).unwrap();
        reducer_length = std::cmp::max(reducer_length, equation.len());
        (test_value, equation)
    }).collect();
    'line_loop: for i in 0..lines.len() {
        let (test_value, equation) = &lines[i];
        println!("{}", test_value);
        let mut reducer = Reducer::new(reducer_length);
        loop {
            let mut ans = equation[0];
            let mut eq_pointer = 1;
            while eq_pointer < equation.len() {
                match reducer.get(eq_pointer - 1) {
                    Op::Add => ans += equation[eq_pointer],
                    Op::Mul => ans *= equation[eq_pointer],
                    Op::Cat => ans = (ans.to_string() + &equation[eq_pointer].to_string()).parse().unwrap(),
                }
                eq_pointer += 1;
            }
            let mut done = reducer.done();
            if &ans == test_value {
                total += test_value;
                done = true;
            }
            if done {
                continue 'line_loop;
            }
            reducer.update();
        }
    }
    total.to_string()
}

#[derive(Debug, Clone, PartialEq)]
struct Reducer {
    operations: Vec<Op>,
    counter: usize,
}

impl Reducer {
    fn new(length: usize) -> Self {
        Reducer { operations: vec![Op::Add; length], counter: 0 }
    }

    fn get(&self, idx: usize) -> Op {
        self.operations[idx]
    }
    
    fn update(&mut self) {
        self.counter += 1;
        self.operations.iter_mut().enumerate().for_each(|(i, op)| if self.counter % 3_usize.pow(i as u32) == 0 { op.roll() });
    }

    fn done(&self) -> bool {
        self.operations.iter().all(|x| if let Op::Cat = *x {true} else {false})
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
enum Op {
    Add, Mul, Cat
}

impl Op {
    fn roll(&mut self){
        match self {
            Op::Add => *self = Op::Mul,
            Op::Mul => *self = Op::Cat,
            Op::Cat => *self = Op::Add,
        };
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &'static str = "190: 10 19\n\
                                    3267: 81 40 27\n\
                                    83: 17 5\n\
                                    156: 15 6\n\
                                    7290: 6 8 6 15\n\
                                    161011: 16 10 13\n\
                                    192: 17 8 14\n\
                                    21037: 9 7 18 13\n\
                                    292: 11 6 16 20";

    // #[ignore]
    #[test]
    fn test_process() {
        assert_eq!("11387".to_string(), process(EXAMPLE));
    }

    #[test]
    fn reducer_rolls_correctly() {
        let mut reducer = Reducer::new(3);
        assert_eq!(reducer.operations, vec![Op::Add, Op::Add, Op::Add]);
        reducer.update();
        assert_eq!(reducer.operations, vec![Op::Mul, Op::Add, Op::Add]);
        reducer.update();
        assert_eq!(reducer.operations, vec![Op::Cat, Op::Add, Op::Add]);
        reducer.update();
        assert_eq!(reducer.operations, vec![Op::Add, Op::Mul, Op::Add]);


        reducer.operations = vec![Op::Cat; 3];
        reducer.counter = 8;
        reducer.update();
        assert_eq!(reducer.operations, vec![Op::Add; 3]);
    }
    
}
