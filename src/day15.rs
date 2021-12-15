use std::collections::{HashMap, HashSet};

pub fn part_1(input: &str) -> i64 {
    let cave: Cave = Cave::new(input);
    find_least_dangerous(cave)
}

pub fn part_2(input: &str) -> i64 {
    0
}

fn find_least_dangerous(cave: Cave) -> i64 {
    // find path using Dijkstra's algorithm
    let mut searched: HashSet<usize> = HashSet::new();
    let mut distances: HashMap<usize, (usize, i32)> = HashMap::new();
    let mut current: usize = 0;

    loop {
        let neighbors: Vec<usize> = cave.get_neighbors(current);

        if current == cave.width * cave.height - 1 {
            break;
        }

        for &n in neighbors.iter().filter(|&n| !searched.contains(n)) {
            // check new distance to neighbors
            // if distance is smaller than previously found, update
            let (_, old_distance) = distances.get(&n).unwrap_or(&(0, std::i32::MAX));

            let new_distance =
                distances.get(&current).unwrap_or(&(0, 0)).1 + (cave.points[n] as i32);

            if new_distance < *old_distance {
                distances.insert(n, (current, new_distance));
            }
        }

        // set current to smallest distance
        searched.insert(current);

        // filter points already visited
        let smallest_distance = distances
            .iter()
            .filter(|(key, _)| !searched.contains(key))
            .min_by_key(|(_, value)| value.1)
            .unwrap();

        current = *smallest_distance.0;
    }

    distances
        .get(&(cave.width * cave.height - 1))
        .unwrap()
        .1
        .into()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 40);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 0);
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
