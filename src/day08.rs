use std::collections::HashMap;

extern crate array_tool;
use array_tool::vec::Intersect;

pub fn part_1(input: &str) -> i64 {
    let input: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .collect();

    // Calculate occurences of 1, 4, 7 and 8
    // All of these numbers use an unique number of segments
    // 1 -> 2 segments
    // 4 -> 4 segments
    // 7 -> 3 segments
    // 8 -> 7 segments
    let common_signal_lengths = [2, 3, 4, 7];

    input
        .iter()
        .map(|(_patterns, signals)| {
            signals
                .split_whitespace()
                .filter(|signal| common_signal_lengths.contains(&signal.len()))
                .count() as i64
        })
        .sum()
}

pub fn part_2(input: &str) -> i64 {
    let input: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .collect();

    input
        .iter()
        .map(|(signals, pattern)| {
            let signals: HashMap<String, u8> = find_digit_patterns(signals);
            parse_pattern(signals, pattern)
        })
        .sum::<i32>() as i64
}

// Find which group of signals is which number.
fn find_digit_patterns(signals: &str) -> HashMap<String, u8> {
    let mut signals: Vec<&str> = signals.split_whitespace().collect();

    signals.sort_by_key(|a| a.len());

    let one = signals[0].to_string();
    let seven = signals[1].to_string();
    let four = signals[2].to_string();
    let eight = signals[9].to_string();

    let (two, three, five) = sort_5_segments(&one, &four, &signals[3..=5]);
    let (zero, six, nine) = sort_6_segments(&one, &four, &signals[6..=8]);

    let signals_sorted = [zero, one, two, three, four, five, six, seven, eight, nine];

    signals_sorted
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut map, (i, signal)| {
            map.insert(sort_chars_alphabetically(signal), i as u8);
            map
        })
}

// sort 5 segments long signals to a tuple of (two, three, five)
fn sort_5_segments(one: &str, four: &str, input: &[&str]) -> (String, String, String) {
    // common_segments(1, num) + common_segments(4, num) -> number
    //   3 -> 2
    //   5 -> 3
    //   4 -> 5

    let signal_a = input[0];
    let signal_b = input[1];
    let signal_c = input[2];

    let a_in_common = common_segments(one, signal_a) + common_segments(four, signal_a);
    let b_in_common = common_segments(one, signal_b) + common_segments(four, signal_b);

    let two = if a_in_common == 3 {
        signal_a.to_string()
    } else if b_in_common == 3 {
        signal_b.to_string()
    } else {
        signal_c.to_string()
    };

    let three = if a_in_common == 5 {
        signal_a.to_string()
    } else if b_in_common == 5 {
        signal_b.to_string()
    } else {
        signal_c.to_string()
    };

    let five = if a_in_common == 4 {
        signal_a.to_string()
    } else if b_in_common == 4 {
        signal_b.to_string()
    } else {
        signal_c.to_string()
    };

    (two, three, five)
}

// sort 6 segments into a tuple of (zero, six, nine)
fn sort_6_segments(one: &str, four: &str, input: &[&str]) -> (String, String, String) {
    // if 6 segments, on
    // how many segments in common with 1 + 4
    //   5 -> 0
    //   4 -> 6
    //   6 -> 9

    let signal_a = input[0];
    let signal_b = input[1];
    let signal_c = input[2];

    let a_in_common = common_segments(one, signal_a) + common_segments(four, signal_a);
    let b_in_common = common_segments(one, signal_b) + common_segments(four, signal_b);

    let zero = if a_in_common == 5 {
        signal_a.to_string()
    } else if b_in_common == 5 {
        signal_b.to_string()
    } else {
        signal_c.to_string()
    };

    let six = if a_in_common == 4 {
        signal_a.to_string()
    } else if b_in_common == 4 {
        signal_b.to_string()
    } else {
        signal_c.to_string()
    };

    let nine = if a_in_common == 6 {
        signal_a.to_string()
    } else if b_in_common == 6 {
        signal_b.to_string()
    } else {
        signal_c.to_string()
    };

    (zero, six, nine)
}

fn common_segments(signal_a: &str, signal_b: &str) -> u8 {
    let signal_a: Vec<char> = signal_a.chars().collect();
    let signal_b: Vec<char> = signal_b.chars().collect();
    signal_a.intersect(signal_b).len() as u8
}

fn parse_pattern(signals: HashMap<String, u8>, pattern: &str) -> i32 {
    pattern
        .split_whitespace()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, signal)| {
            let signal = sort_chars_alphabetically(signal);
            let num = *signals.get(&signal).unwrap() as i32;
            acc + num * (10_i32.pow(i as u32))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 61229);
    }

    #[test]
    fn test_find_digit_patterns() {
        let signals: HashMap<String, u8> = HashMap::from([
            ("abd".to_string(), 7),
            ("bcdefg".to_string(), 6),
            ("bcdef".to_string(), 5),
            ("abcdef".to_string(), 9),
            ("abcdeg".to_string(), 0),
            ("abef".to_string(), 4),
            ("abcdf".to_string(), 3),
            ("ab".to_string(), 1),
            ("abcdefg".to_string(), 8),
            ("acdfg".to_string(), 2),
        ]);
        let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";
        assert_eq!(find_digit_patterns(input), signals);
    }

    #[test]
    fn test_parse_pattern() {
        let pattern = "cdfeb fcadb cdfeb cdbaf";
        let expected = 5353;
        let signals: HashMap<String, u8> = HashMap::from([
            ("abd".to_string(), 7),
            ("bcdefg".to_string(), 6),
            ("bcdef".to_string(), 5),
            ("abcdef".to_string(), 9),
            ("abcdeg".to_string(), 0),
            ("abef".to_string(), 4),
            ("abcdf".to_string(), 3),
            ("ab".to_string(), 1),
            ("abcdefg".to_string(), 8),
            ("acdfg".to_string(), 2),
        ]);
        assert_eq!(parse_pattern(signals, pattern), expected);
    }
}

fn sort_chars_alphabetically(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

#[cfg(test)]
const INPUT: &str =
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
