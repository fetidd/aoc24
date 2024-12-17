use aoc24::utils;

fn main() {
    let input = include_str!("../../puzzle_input/day7.txt");
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
        let mut s = line.split(':');
        let test_value: i32 = s.next().unwrap().parse().unwrap();
        let equation: Vec<i32> = utils::parse_space_list(&s.collect::<String>()).unwrap();
        // just adds
        if equation.iter().cloned().reduce(|a, b| a + b).unwrap() == test_value {
            total += test_value;
            continue;
        }
        // just multiplies
        if equation.iter().cloned().reduce(|a, b| a * b).unwrap() == test_value {
            total += test_value;
            continue;
        }
    }
    total.to_string()
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
}
