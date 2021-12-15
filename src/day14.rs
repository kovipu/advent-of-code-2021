use std::collections::HashMap;

pub fn part_1(input: &str) -> i64 {
    let (polymer, pair_insertion_rules) = input.split_once("\n\n").unwrap();
    let mut polymer: Vec<char> = polymer.chars().collect();

    let pair_insertion_rules: HashMap<(char, char), char> = build_rules(pair_insertion_rules);

    for _ in 1..=10 {
      step(&mut polymer, &pair_insertion_rules);
    }

    let counts: HashMap<char, i32> = polymer.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(*c).or_insert(0) += 1;
        acc
    });

    let result = counts.values().max().unwrap() - counts.values().min().unwrap();
    result as i64
}

pub fn part_2(input: &str) -> i64 {
    0
}

fn build_rules(pair_insertion_rules: &str) -> HashMap<(char, char), char> {
    pair_insertion_rules
        .lines()
        .map(|line| {
            let (pair, new_element) = line.split_once(" -> ").unwrap();
            let a = pair.chars().next().unwrap();
            let b = pair.chars().nth(1).unwrap();

            ((a, b), new_element.chars().next().unwrap())
        })
        .collect()
}

fn step(polymer: &mut Vec<char>, pair_insertion_rules: &HashMap<(char, char), char>) {
    let mut new_polymer: Vec<char> =
        polymer
            .iter()
            .zip(polymer.clone().iter().skip(1))
            .fold(vec![], |mut acc, (cur, next)| {
                let insertion: char = *pair_insertion_rules.get(&(*cur, *next)).unwrap();

                acc.push(*cur);
                acc.push(insertion);
                acc
            });
    new_polymer.push(*polymer.last().unwrap());
    *polymer = new_polymer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 1588);
    }

    #[test]
    fn test_step() {
      let (polymer, pair_insertion_rules) = INPUT.split_once("\n\n").unwrap();
      let mut polymer: Vec<char> = polymer.chars().collect();
  
      let pair_insertion_rules: HashMap<(char, char), char> = build_rules(pair_insertion_rules);

      step(&mut polymer, &pair_insertion_rules);

      let expected: Vec<char> = "NCNBCHB".chars().collect();
      assert_eq!(polymer, expected);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 0);
    }
}

#[cfg(test)]
const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
