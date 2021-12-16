use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

pub fn part_1(input: &str) -> i64 {
    let cave: Cave = Cave::new(input);
    find_least_dangerous(cave)
}

pub fn part_2(input: &str) -> i64 {
    let cave = Cave::new_five_times_as_large(input);
    find_least_dangerous(cave)
}

fn find_least_dangerous(cave: Cave) -> i64 {
    // efficient Djikstra's using a priority queue
    let mut pq: BinaryHeap<SearchedPoint> = BinaryHeap::new();
    let mut visited: HashSet<usize> = HashSet::new();

    let goal: usize = cave.points.len() - 1;

    // priority queue should have the point with smallest danger at the top
    pq.push(SearchedPoint { idx: 0, danger: 0 });
    visited.insert(0);

    // we should pop the one with the smallest danger. hmm.
    while let Some(SearchedPoint { idx, danger }) = pq.pop() {
        if idx == goal {
            return danger;
        }

        for n_idx in cave.get_neighbors(idx) {
            if visited.contains(&n_idx) {
                continue;
            }

            visited.insert(n_idx);
            pq.push(SearchedPoint {
                idx: n_idx,
                danger: danger + cave.points[n_idx] as i64,
            });
        }
    }

    panic!("No path found!");
}

struct Cave {
    width: usize,
    height: usize,
    points: Vec<u8>,
}

impl Cave {
    fn new(input: &str) -> Cave {
        let width: usize = input.lines().next().unwrap().len();
        let height: usize = input.lines().count();
        let points: Vec<u8> = input
            .replace("\n", "")
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        Cave {
            width,
            height,
            points,
        }
    }

    fn new_five_times_as_large(input: &str) -> Cave {
        let width: usize = input.lines().next().unwrap().len() * 5;
        let height: usize = input.lines().count() * 5;

        let mut points: Vec<u8> = Vec::new();

        for y in 0..5 {
            input.lines().for_each(|line| {
                // copy line 5 times
                for x in 0..5 {
                    line.chars().for_each(|c| {
                        let val = (c.to_digit(10).unwrap() + x + y) as u8;

                        if val > 9 {
                            points.push(val - 9);
                        } else {
                            points.push(val);
                        }
                    });
                }
            });
        }

        Cave {
            width,
            height,
            points,
        }
    }

    fn get_neighbors(&self, idx: usize) -> Vec<usize> {
        let mut neighbors: Vec<usize> = Vec::new();

        let x: usize = idx % self.width;
        let y: usize = idx / self.width;

        if x > 0 {
            neighbors.push(idx - 1);
        }
        if x < self.width - 1 {
            neighbors.push(idx + 1);
        }
        if y > 0 {
            neighbors.push(idx - self.width);
        }
        if y < self.height - 1 {
            neighbors.push(idx + self.width);
        }

        neighbors
    }
}

#[derive(Debug, Eq)]
struct SearchedPoint {
    danger: i64,
    idx: usize,
}

impl Ord for SearchedPoint {
    fn cmp(&self, other: &SearchedPoint) -> Ordering {
        other.danger.cmp(&self.danger)
    }
}

impl PartialOrd for SearchedPoint {
    fn partial_cmp(&self, other: &SearchedPoint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SearchedPoint {
    fn eq(&self, other: &SearchedPoint) -> bool {
        self.idx == other.idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 40);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 315);
    }
}

#[cfg(test)]
const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
