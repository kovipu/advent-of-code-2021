use std::collections::HashSet;

pub fn part_1(input: &str) -> i64 {
    let (points, folds) = input.split_once("\n\n").unwrap();

    let mut points: Vec<(i32, i32)> = points
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect();

    fold_paper(&mut points, folds.lines().next().unwrap());

    let unique: HashSet<(i32, i32)> = points.iter().cloned().collect();
    unique.len() as i64
}

pub fn part_2(input: &str) -> i64 {
    let (points, folds) = input.split_once("\n\n").unwrap();

    let mut points: Vec<(i32, i32)> = points
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect();

    for fold in folds.lines() {
        fold_paper(&mut points, fold);
    }

    let unique: HashSet<(i32, i32)> = points.iter().cloned().collect();

    let width: i32 = *unique.iter().map(|(x, _)| x).max().unwrap();
    let height: i32 = *unique.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..height {
        for x in 0..width {
            if !unique.contains(&(x, y)) {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!();
    }

    0
}

fn fold_paper(points: &mut Vec<(i32, i32)>, fold: &str) {
    let (axel, coord) = fold
        .strip_prefix("fold along ")
        .unwrap()
        .split_once("=")
        .unwrap();

    let coord = coord.parse::<i32>().unwrap();

    match axel {
        "x" => {
            points.iter_mut().for_each(|(x, _)| {
                if *x > coord {
                    let distance = *x - coord;
                    *x -= distance * 2;
                }
            });
        }
        "y" => {
            points.iter_mut().for_each(|(_, y)| {
                if *y > coord {
                    let distance = *y - coord;
                    *y -= distance * 2;
                }
            });
        }
        _ => panic!("Unknown axel: {}", axel),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 17);
    }
}

#[cfg(test)]
const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";
