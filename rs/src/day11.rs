use std::collections::HashMap;

use aoc24::utils;

const BLINKS: usize = 25;

fn main() {
    let input = include_str!("../../puzzle_input/day11.txt");
    let result = process(&input);
    println!("{}", result);
}

fn process(input: &str) -> String {
    let mut cache = HashMap::new();
    utils::parse_space_list::<String>(input)
        .unwrap()
        .iter()
        // .map(|stone| blink(&stone, 25))
        .map(|stone| blink(&stone, 0, &mut cache, BLINKS))
        .sum::<u64>()
        .to_string()
}

fn blink<'a>(
    stone: &'a str,
    blinks_done: usize,
    mut cache: &mut HashMap<String, u64>,
    max_blinks: usize,
) -> u64 {
    println!("{}{stone}", " ".repeat(blinks_done));
    // if let Some(res) = cache.get(stone) {
    //     return *res;
    // }
    if blinks_done == max_blinks {
        return 1;
    }
    if stone == "0" {
        println!("0 => 1");
        let res = blink("1", blinks_done + 1, &mut cache, max_blinks);
        cache.insert(stone.into(), res);
        res
    } else if stone.len() % 2 == 0 {
        println!(
            "{}{stone} => {} and {}",
            " ".repeat(blinks_done),
            &stone[0..(stone.len() / 2)],
            &stone[(stone.len() / 2)..stone.len()]
        );
        let left = &stone[0..(stone.len() / 2)];
        let mut right = &stone[(stone.len() / 2)..stone.len()];
        if right.starts_with('0') {
            right = "0";
        }
        let res = blink(left, blinks_done + 1, &mut cache, max_blinks)
            + blink(right, blinks_done + 1, &mut cache, max_blinks);
        cache.insert(stone.into(), res);
        res
    } else {
        let n = stone.parse::<u64>().unwrap() * 2024;
        println!("{}{stone} => {}", " ".repeat(blinks_done), n.to_string());
        let res = blink(
            n.to_string().as_str(),
            blinks_done + 1,
            &mut cache,
            max_blinks,
        );
        cache.insert(stone.into(), res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "125 17";

    #[ignore]
    #[test]
    fn test_process() {
        assert_eq!("55312".to_string(), process(EXAMPLE));
    }

    fn do_blinks(num: usize) -> String {
        let mut cache = HashMap::new();
        utils::parse_space_list::<String>(EXAMPLE)
            .unwrap()
            .into_iter()
            .map(|stone| blink(&stone, 0, &mut cache, num))
            .sum::<u64>()
            .to_string()
    }

    #[test]
    fn test_blink() {
        // assert_eq!("3", do_blinks(1));
        // assert_eq!("4", do_blinks(2));
        // assert_eq!("5", do_blinks(3));
        // assert_eq!("9", do_blinks(4));
        assert_eq!("13", do_blinks(5));
        // assert_eq!("22", do_blinks(6));
    }
}
