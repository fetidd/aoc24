use std::str::FromStr;

pub fn parse_space_list<T: FromStr>(list: &str) -> Result<Vec<T>, String> {
    list
        .split(' ')
        .map(|x| x.trim_start().trim_end())
        .filter(|x| *x != "")
        .map(|x| x.parse::<T>())
        .collect::<Result<Vec<T>, <T as FromStr>::Err>>()
        .map_err(|_| format!("failed to parse '{list}'"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_space_list() {
        let tests = vec![
            ("", Ok(vec![])),
            ("1 2 3 4 5", Ok(vec![1, 2, 3, 4, 5])),
            (" 1 2   3 4  5 ", Ok(vec![1, 2, 3, 4, 5])),
            ("1 2 3 b 5", Err("failed to parse '1 2 3 b 5'".to_string())),
        ];
        for (input, expected) in tests {
            assert_eq!(expected, parse_space_list::<i32>(input));
        }
        
        let tests = vec![
            ("", Ok(vec![])),
            ("a b c d e", Ok(vec!['a', 'b', 'c', 'd', 'e'])),
            (" a   b c  d e ", Ok(vec!['a', 'b', 'c', 'd', 'e'])),
        ];
        for (input, expected) in tests {
            assert_eq!(expected, parse_space_list::<char>(input));
        }
    }

}
