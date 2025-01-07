use std::collections::HashMap;

use aoc24::utils;

const BLINKS: usize = 75;

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
    mut cache: &mut HashMap<(String, usize), u64>,
    max_blinks: usize,
) -> u64 {
    // println!("{}{stone}", " ".repeat(blinks_done));
    let blinks_remaining = max_blinks - blinks_done;
    if let Some(res) = cache.get(&(stone.into(), blinks_remaining)) {
        return *res;
    }
    if blinks_done == max_blinks {
        return 1;
    }
    if stone == "0" {
        // println!("0 => 1");
        let res = blink("1", blinks_done + 1, &mut cache, max_blinks);
        cache.insert((stone.into(), blinks_remaining), res);
        res
    } else if stone.len() % 2 == 0 {
        let left = &stone[0..(stone.len() / 2)];
        let mut right = &stone[(stone.len() / 2)..stone.len()];
        while right.starts_with('0') && right.len() > 1 {
            right = &right[1..];
        }
        // println!(
        //     "{}{stone} => {} and {}",
        //     " ".repeat(blinks_done),
        //     &left,
        //     &right
        // );
        let res = blink(left, blinks_done + 1, &mut cache, max_blinks)
            + blink(right, blinks_done + 1, &mut cache, max_blinks);
        cache.insert((stone.into(), blinks_remaining), res);
        res
    } else {
        let n = stone.parse::<u64>().unwrap() * 2024;
        // println!("{}{stone} => {}", "  ".repeat(blinks_done), n.to_string());
        let res = blink(
            n.to_string().as_str(),
            blinks_done + 1,
            &mut cache,
            max_blinks,
        );
        cache.insert((stone.into(), blinks_remaining), res);
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

    fn do_blinks(stones: &str, num: usize) -> String {
        let mut cache = HashMap::new();
        utils::parse_space_list::<String>(stones)
            .unwrap()
            .into_iter()
            .map(|stone| blink(&stone, 0, &mut cache, num))
            .sum::<u64>()
            .to_string()
    }

    #[test]
    fn test_blink() {
        assert_eq!("3", do_blinks(EXAMPLE, 1));
        assert_eq!("4", do_blinks(EXAMPLE, 2));
        assert_eq!("5", do_blinks(EXAMPLE, 3));
        assert_eq!("9", do_blinks(EXAMPLE, 4));
        assert_eq!("13", do_blinks(EXAMPLE, 5));
        assert_eq!("22", do_blinks(EXAMPLE, 6));
    }
}
