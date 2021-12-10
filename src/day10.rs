use std::collections::HashMap;

pub fn part_1(input: &str) -> i64 {
    input
        .lines()
        .map(find_illegal_char)
        .fold(0, |acc, x| match x {
            Some(')') => acc + 3,
            Some(']') => acc + 57,
            Some('}') => acc + 1197,
            Some('>') => acc + 25137,
            Some(_) => panic!("Unexpected character"),
            None => acc,
        })
}

pub fn part_2(input: &str) -> i64 {
    let mut scores: Vec<i64> = input
        .lines()
        .filter(|line| find_illegal_char(line).is_none())
        .map(|line| {
            find_missing_chars(line).iter().fold(0, |acc, x| match x {
                ')' => acc * 5 + 1,
                ']' => acc * 5 + 2,
                '}' => acc * 5 + 3,
                '>' => acc * 5 + 4,
                _ => panic!("Unexpected character"),
            })
        })
        .collect();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn find_illegal_char(input: &str) -> Option<char> {
    let mut stack = Vec::new();
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    for c in input.chars() {
        if pairs.contains_key(&c) {
            stack.push(c);
        } else {
            // should be the correct pair. if not -> return the char
            let correct_char = pairs.get(&stack.pop().unwrap()).unwrap();

            if c != *correct_char {
                return Some(c);
            }
        }
    }
    None
}

fn find_missing_chars(input: &str) -> Vec<char> {
    let mut stack = Vec::new();
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    for c in input.chars() {
        if pairs.contains_key(&c) {
            stack.push(c);
        } else {
            stack.pop();
        }
    }
    stack.iter().map(|c| *pairs.get(c).unwrap()).rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 26397);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 288957);
    }
}

#[cfg(test)]
const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
