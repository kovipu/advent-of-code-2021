use std::collections::HashMap;

pub fn part_1(input: &str) -> i32 {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let transposed_array = transpose(input);

    let most_commons: String = transposed_array
        .iter()
        .map(|column| most_common(column))
        .collect();

    // This was more readable than bitwise not and a bitmask.
    let least_commons: String = most_commons
        .chars()
        .map(|char| if char == '0' { '1' } else { '0' })
        .collect();

    let gamma_rate: i32 = i32::from_str_radix(&most_commons, 2).unwrap();
    let epsilon_rate: i32 = i32::from_str_radix(&least_commons, 2).unwrap();

    gamma_rate * epsilon_rate
}

pub fn part_2(input: &str) -> i32 {
    let input: Vec<&str> = input.lines().collect();

    let oxygen_generator_rating = calculate_rating(&input, false, 0);
    let oxygen_generator_rating = i32::from_str_radix(&oxygen_generator_rating, 2).unwrap();

    let co2_scrubber_rating = calculate_rating(&input, true, 0);
    let co2_scrubber_rating = i32::from_str_radix(&co2_scrubber_rating, 2).unwrap();

    oxygen_generator_rating * co2_scrubber_rating
}

fn calculate_rating(input: &[&str], use_least_common: bool, search_index: usize) -> String {
    // &input, search_index = 0, if multiple matches, recurse with matches and search_index +1
    let chars_to_search = input
        .iter()
        .map(|line| line.chars().nth(search_index).unwrap());

    let ones = &chars_to_search.clone().filter(|c| *c == '1').count();
    let zeros = &chars_to_search.filter(|c| *c == '0').count();

    let considered_value = if zeros > ones {
        if use_least_common {
            '1'
        } else {
            '0'
        }
    } else if use_least_common {
        '0'
    } else {
        '1'
    };

    let matches: Vec<&str> = input
        .iter()
        .filter(|line| line.chars().nth(search_index).unwrap() == considered_value)
        .copied()
        .collect();

    match matches.len() {
        1 => matches[0].to_string(),
        _ => calculate_rating(&matches, use_least_common, search_index + 1),
    }
}

fn transpose(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect()
}

// might not default to 1 or 0 as it should
fn most_common(v: &[char]) -> char {
    let mut counts = HashMap::new();
    for i in v {
        *counts.entry(i).or_insert(0) += 1;
    }
    let mut max_count = 0;
    let mut max_key = v[0];
    for (key, count) in counts {
        if count > max_count {
            max_count = count;
            max_key = *key;
        }
    }
    max_key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 198);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 230);
    }
}

#[cfg(test)]
const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
