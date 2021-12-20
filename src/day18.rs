/*
I didn't manage to get the correct solution :(
TODO: come back and try again.
*/

use SnailfishNumber::*;

pub fn part_1(input: &str) -> i64 {
    // let _sum = input
    //     .lines()
    //     .map(|line| SnailfishNumber::new(line).0)
    //     .reduce(|acc, number| acc.add(&number))
    //     .unwrap();
    0
}

pub fn part_2(_input: &str) -> i64 {
    0
}

#[derive(Debug, PartialEq, Clone)]
enum SnailfishNumber {
    Literal(u8),
    Pair(Box<(SnailfishNumber, SnailfishNumber)>),
}

#[derive(Debug)]
struct SnailfishNumberReduceResult {
    exploded: SnailfishNumber,
    carry_left: Option<u8>,
    carry_right: Option<u8>,
}

impl SnailfishNumber {
    fn new(input: &str) -> (SnailfishNumber, &str) {
        // if [ -> new pair starts
        if input.starts_with("[") {
            let (first, rest) = SnailfishNumber::new(&input[1..]);
            let (second, rest) = SnailfishNumber::new(rest);
            return (SnailfishNumber::Pair(Box::new((first, second))), rest);
        }

        // else -> literal
        let literal: u8 = input.chars().next().unwrap().to_digit(10).unwrap() as u8;

        // return rest after comma
        match input.split_once(",") {
            Some((_, rest)) => (SnailfishNumber::Literal(literal), rest),
            None => (SnailfishNumber::Literal(literal), ""),
        }
    }

    fn add(&self, other: &SnailfishNumber) -> SnailfishNumber {
        // make the numbers a new pair
        let left = (*self).clone();
        let right = (*other).clone();
        let mut output = SnailfishNumber::Pair(Box::new((left, right)));

        // loop until neither explosion or split happens
        loop {
            let mut changed = false;

            // loop until explosion doesn't change result
            loop {
                let added = output.explode(0, false, None, None).exploded;
                if output == added {
                    break;
                }
                changed = true;
                output = added;
            }

            // loop until split doesn't change result
            loop {
                let split = output.split();
                if output == split {
                    break;
                }
                changed = true;
                output = split;
            }

            if !changed {
                break;
            }
        }

        output
    }

    fn explode(
        &self,
        depth: u32,
        exploded: bool,
        carry_left: Option<u8>,
        carry_right: Option<u8>,
    ) -> SnailfishNumberReduceResult {
        /* if any pair is nested inside four pairs, the leftmost such pair explodes
         * detect exactly 4 deep nesting, if we do this at every step, there's only 1 layer of nesting added
         * mutate the parent pair of the 4th level
         */

        match self {
            Literal(literal) => SnailfishNumberReduceResult {
                exploded: Literal(*literal),
                carry_left: None,
                carry_right: None,
            },
            Pair(pair) => {
                if depth == 4 && !exploded {
                    // explode
                    match **pair {
                        (Literal(left), Literal(right)) => {
                            return SnailfishNumberReduceResult {
                                exploded: Literal(0),
                                carry_left: Some(left),
                                carry_right: Some(right),
                            };
                        }
                        _ => panic!("unexpected pair"),
                    }
                }

                let new_left = pair.0.explode(depth + 1, exploded, carry_left, carry_right);

                let new_exploded =
                    exploded || new_left.carry_left.is_some() || new_left.carry_right.is_some();

                let new_right = pair.1.explode(
                    depth + 1,
                    new_exploded,
                    new_left.carry_left,
                    new_left.carry_right,
                );

                // there's a carry to the left we can add to the leaf on the right
                if let Some(carry) = carry_left {
                    if let Literal(literal) = &new_right.exploded {
                        return SnailfishNumberReduceResult {
                            exploded: Pair(Box::new((new_left.exploded, Literal(literal + carry)))),
                            carry_left: None,
                            carry_right: new_right.carry_right,
                        };
                    }
                }

                // there's a carry to the right we can add to a leaf on the left
                if let Literal(left) = &new_left.exploded {
                    if let Some(carry_right) = carry_right {
                        return SnailfishNumberReduceResult {
                            exploded: Pair(Box::new((
                                Literal(carry_right + left),
                                new_right.exploded,
                            ))),
                            carry_left: new_right.carry_left,
                            carry_right: None,
                        };
                    }
                }

                // if new_right is literal && new_left has carry_right, add it to new_right
                if let Literal(right) = &new_right.exploded {
                    if let Some(carry_right) = new_left.carry_right {
                        return SnailfishNumberReduceResult {
                            exploded: Pair(Box::new((
                                new_left.exploded,
                                Literal(right + carry_right),
                            ))),
                            carry_left: new_left.carry_left,
                            carry_right: None,
                        };
                    }
                }

                // if new_left is literal && new_right has carry_left, add it to new_left
                if let Literal(left) = &new_left.exploded {
                    if let Some(carry_left) = new_right.carry_left {
                        return SnailfishNumberReduceResult {
                            exploded: Pair(Box::new((
                                Literal(left + carry_left),
                                new_right.exploded,
                            ))),
                            carry_left: None,
                            carry_right: new_right.carry_right,
                        };
                    }
                }

                SnailfishNumberReduceResult {
                    exploded: Pair(Box::new((new_left.exploded, new_right.exploded))),
                    carry_left: new_left.carry_left,
                    carry_right: new_right.carry_right,
                }
            }
        }
    }

    fn split(&self) -> SnailfishNumber {
        match self {
            Literal(literal) => {
                if *literal < 10 {
                    return Literal(*literal);
                }

                let split: f32 = *literal as f32 / 2.0;
                let left: u8 = split.floor() as u8;
                let right: u8 = split.ceil() as u8;

                Pair(Box::new((Literal(left), Literal(right))))
            }
            Pair(pair) => Pair(Box::new((pair.0.split(), pair.1.split()))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 4140);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(""), 0);
    }

    #[test]
    fn test_parse_snailfish_number() {
        let input: &str = "[[1,2],[[3,4],5]]";
        let expected: SnailfishNumber = Pair(Box::new((
            Pair(Box::new((Literal(1), Literal(2)))),
            Pair(Box::new((
                Pair(Box::new((Literal(3), Literal(4)))),
                Literal(5),
            ))),
        )));

        let (actual, _) = SnailfishNumber::new(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_addition() {
        let (a, _) = SnailfishNumber::new("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let (b, _) = SnailfishNumber::new("[1,1]");
        let expected: SnailfishNumber = Pair(Box::new((
            Pair(Box::new((
                Pair(Box::new((
                    Pair(Box::new((Literal(0), Literal(7)))),
                    Literal(4),
                ))),
                Pair(Box::new((
                    Pair(Box::new((Literal(7), Literal(8)))),
                    Pair(Box::new((Literal(6), Literal(0)))),
                ))),
            ))),
            Pair(Box::new((Literal(8), Literal(1)))),
        )));
        assert_eq!(a.add(&b), expected);
    }

    #[test]
    fn test_addition_2() {
        let (a, _) = SnailfishNumber::new("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
        let (b, _) = SnailfishNumber::new("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
        dbg!(a.add(&b));
        assert_eq!(a.add(&b), b);
    }
}

#[cfg(test)]
const INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
