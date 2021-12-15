use std::collections::HashMap;
use std::iter;

pub fn part_1(input: &str) -> i64 {
    let (polymer, pair_insertion_rules) = input.split_once("\n\n").unwrap();
    let mut polymer: Polymer = build_polymer(polymer);
    let pair_insertion_rules: HashMap<Pair, (Pair, Pair)> = build_rules(pair_insertion_rules);

    for _ in 1..=10 {
        step(&mut polymer, &pair_insertion_rules);
    }

    let counts: HashMap<char, i64> = polymer.iter().fold(HashMap::new(), |mut acc, (pair, val)| {
        *acc.entry(pair.left).or_insert(0) += val;
        acc
    });

    (counts.values().max().unwrap() - counts.values().min().unwrap()) as i64
}

pub fn part_2(input: &str) -> i64 {
    let (polymer, pair_insertion_rules) = input.split_once("\n\n").unwrap();
    let mut polymer: Polymer = build_polymer(polymer);
    let pair_insertion_rules: HashMap<Pair, (Pair, Pair)> = build_rules(pair_insertion_rules);

    for _ in 1..=40 {
        step(&mut polymer, &pair_insertion_rules);
    }

    let counts: HashMap<char, i64> = polymer.iter().fold(HashMap::new(), |mut acc, (pair, val)| {
        *acc.entry(pair.left).or_insert(0) += val;
        acc
    });

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn build_polymer(polymer: &str) -> Polymer {
    polymer
        .chars()
        .zip(polymer.chars().skip(1).chain(iter::repeat(' ')))
        .fold(HashMap::new(), |mut acc, (left, right)| {
            acc.entry(Pair { left, right })
                .and_modify(|x| *x += 1)
                .or_insert(1);
            acc
        })
}

fn build_rules(pair_insertion_rules: &str) -> PairInsertionRules {
    pair_insertion_rules
        .lines()
        .map(|line| {
            let (pair, new_element) = line.split_once(" -> ").unwrap();
            let a = pair.chars().next().unwrap();
            let b = pair.chars().nth(1).unwrap();

            let c = new_element.chars().next().unwrap();

            (
                Pair { left: a, right: b },
                (Pair { left: a, right: c }, Pair { left: c, right: b }),
            )
        })
        .collect()
}

fn step(polymer: &mut Polymer, pair_insertion_rules: &PairInsertionRules) {
    let mut new_polymer: Polymer = HashMap::new();

    let mut update_polymer = |pair: &Pair, val: i64| {
        *new_polymer
            .entry(Pair {
                left: pair.left,
                right: pair.right,
            })
            .or_insert(0) += val;
    };

    for (key, val) in polymer.iter() {
        match pair_insertion_rules.get(key) {
            Some((first, second)) => {
                update_polymer(first, *val);
                update_polymer(second, *val);
            }
            None => {
                // end molecule
                update_polymer(key, *val);
            }
        }
    }

    *polymer = new_polymer;
}

type Polymer = HashMap<Pair, i64>;
type PairInsertionRules = HashMap<Pair, (Pair, Pair)>;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pair {
    left: char,
    right: char,
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
        let mut polymer: Polymer = build_polymer(polymer);

        let pair_insertion_rules: HashMap<Pair, (Pair, Pair)> = build_rules(pair_insertion_rules);

        step(&mut polymer, &pair_insertion_rules);

        let expected: Polymer = HashMap::from([
            (
                Pair {
                    left: 'H',
                    right: 'B',
                },
                1,
            ),
            (
                Pair {
                    left: 'C',
                    right: 'N',
                },
                1,
            ),
            (
                Pair {
                    left: 'C',
                    right: 'H',
                },
                1,
            ),
            (
                Pair {
                    left: 'N',
                    right: 'B',
                },
                1,
            ),
            (
                Pair {
                    left: 'B',
                    right: 'C',
                },
                1,
            ),
            (
                Pair {
                    left: 'B',
                    right: ' ',
                },
                1,
            ),
            (
                Pair {
                    left: 'N',
                    right: 'C',
                },
                1,
            ),
        ]);
        assert_eq!(polymer, expected);
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
