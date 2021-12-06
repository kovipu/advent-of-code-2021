use regex::Regex;

const MAP_SIZE: usize = 1000;

pub fn part_1(input: &str) -> i32 {
    let lines = input.lines().map(Vent::new).collect::<Vec<_>>();

    // Create a flat vector that has space for 2d stuff
    let mut map: Vec<i32> = vec![0; MAP_SIZE * MAP_SIZE];

    for line in lines {
        for point in line.points {
            map[point.x + (point.y * MAP_SIZE)] += 1;
        }
    }

    map.iter().filter(|x| *x > &1).count().try_into().unwrap()
}

pub fn part_2(input: &str) -> i32 {
    0
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
    fn new(line: &str) -> Self {
        let re = Regex::new(r"([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)").unwrap();
        let caps = re.captures(line).unwrap();

        let x1: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let y1: usize = caps.get(2).unwrap().as_str().parse().unwrap();

        let x2: usize = caps.get(3).unwrap().as_str().parse().unwrap();
        let y2: usize = caps.get(4).unwrap().as_str().parse().unwrap();

        if x1 == x2 {
            // line is horizontal
            let range = if y1 < y2 { y1..=y2 } else { y2..=y1 };
            let points = range.map(|y| Point { x: x1, y }).collect();
            Vent { points }
        } else if y1 == y2 {
            // line is vertical
            let range = if x1 < x2 { x1..=x2 } else { x2..=x1 };
            let points = range.map(|x| Point { x, y: y1 }).collect();
            Vent { points }
        } else {
            // line is diagonal
            Vent { points: vec![] }
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
