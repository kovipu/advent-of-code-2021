use regex::Regex;

const MAP_SIZE: usize = 1000;

pub fn part_1(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| Vent::new(line, true))
        .collect::<Vec<_>>();

    // Create a flat vector that has space for 2d stuff
    let mut map: Vec<i32> = vec![0; MAP_SIZE * MAP_SIZE];

    for line in lines {
        for point in line.points {
            map[point.x + (point.y * MAP_SIZE)] += 1;
        }
    }

    map.iter().filter(|x| *x > &1).count().try_into().unwrap()
}

pub fn part_2(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| Vent::new(line, false))
        .collect::<Vec<_>>();

    // Create a flat vector that has space for 2d stuff
    let mut map: Vec<i32> = vec![0; MAP_SIZE * MAP_SIZE];

    for line in lines {
        for point in line.points {
            map[point.x + (point.y * MAP_SIZE)] += 1;
        }
    }

    map.iter().filter(|x| *x > &1).count().try_into().unwrap()
}

#[derive(Debug)]
struct Vent {
    points: Vec<Point>,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Vent {
    fn new(line: &str, ignore_diagonals: bool) -> Self {
        let re = Regex::new(r"([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)").unwrap();
        let caps = re.captures(line).unwrap();

        let x1: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let y1: usize = caps.get(2).unwrap().as_str().parse().unwrap();

        let x2: usize = caps.get(3).unwrap().as_str().parse().unwrap();
        let y2: usize = caps.get(4).unwrap().as_str().parse().unwrap();

        let xrange: Vec<usize> = if x1 < x2 {
            (x1..=x2).collect()
        } else {
            (x2..=x1).rev().collect()
        };

        // These have no reason to be collected, I'm just bad at Rust.
        let yrange: Vec<usize> = if y1 < y2 {
            (y1..=y2).collect()
        } else {
            (y2..=y1).rev().collect()
        };

        if x1 == x2 {
            // line is horizontal
            let points = yrange.iter().map(|y| Point { x: x1, y: *y }).collect();
            Vent { points }
        } else if y1 == y2 {
            // line is vertical
            let points = xrange.iter().map(|x| Point { x: *x, y: y1 }).collect();
            Vent { points }
        } else {
            // line is diagonal
            if ignore_diagonals {
                return Vent { points: vec![] };
            }

            let range_combined = xrange.iter().zip(yrange);
            let points: Vec<Point> = range_combined.map(|(x, y)| Point { x: *x, y }).collect();

            Vent { points }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 12);
    }
}

#[cfg(test)]
const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
