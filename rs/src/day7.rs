use std::collections::HashSet;

use aoc24::utils;

fn main() {
    let input = include_str!("../../puzzle_input/day7.txt");
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let mut total = 0;
    'line_loop: for line in input.lines() {
        let mut s = line.split(':');
        let test_value: i32 = s.next().unwrap().parse().unwrap();
        let equation: Vec<i32> = utils::parse_space_list(&s.collect::<String>()).unwrap();
        for reducer in create_reducers(equation.len()) {
            if equation.iter().cloned().enumerate().reduce(|(_, acc), (new_i, new)| do_reduce(&reducer, acc, new, new_i)).map(|(_, n)| n).unwrap() == test_value {
                total += test_value;
                continue 'line_loop;
            }
        }
    }
    total.to_string()
}

fn create_reducers(num: usize) -> HashSet<Vec<u8>> {
    let mut reducers = HashSet::from([vec![0; num], vec![1; num]]);
    reducers.insert(vec![[0, 1]; num].into_iter().flatten().take(num).collect());
    reducers.insert(vec![[1, 0]; num].into_iter().flatten().take(num).collect());
    for i in 0..num {
        reducers.insert(vec![1; num].into_iter().enumerate().map(|(j, x)| if i == j {0} else {x}).collect());
        reducers.insert(vec![1; num].into_iter().enumerate().map(|(j, x)| if i == j {x} else {0}).collect());
        for block_start_index in 0..num-i {

        }
    }
    // pairs, triples, ..., num-1s
    reducers
}

fn do_reduce(reducer: &Vec<u8>, acc: i32, new: i32, new_i: usize) -> (usize, i32) {
    match reducer[new_i] {
        0 => (new_i, acc + new),
        1 => (new_i, acc * new),
        _ => panic!("invalid reducer function specified!")
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

    #[test]
    fn test_process() {
        assert_eq!("3749".to_string(), process(EXAMPLE));
    }

    #[test]
    fn create_reducer_len_4() {
        let actual = create_reducers(4);
        let expected = HashSet::from([
            vec![0, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 1],
            vec![1, 1, 1, 0],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0],
            vec![0, 1, 0, 1],
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 0],
            vec![1, 1, 1, 1],
        ]);
        assert_eq!(expected, actual, "missing {:?}", expected.difference(&actual));
    }

    #[test]
    fn create_reducer_len_3() {
        assert_eq!(HashSet::from([
            vec![0, 0, 0],
            vec![1, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![1, 1, 0],
            vec![0, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1],
        ]), create_reducers(3));
    }
    
    #[test]
    fn create_reducer_len_2() {
        assert_eq!(HashSet::from([
            vec![0, 0],
            vec![1, 0],
            vec![0, 1],
            vec![1, 1],
        ]), create_reducers(2));
    }
    
    #[test]
    fn create_reducer_len_1() {
        assert_eq!(HashSet::from([
            vec![0],
            vec![1],
        ]), create_reducers(1));
    }
}
