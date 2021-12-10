pub fn part_1(input: &str) -> i64 {
    let lava_tube = LavaTube::new(input);

    lava_tube
        .points
        .iter()
        .enumerate()
        .fold(0, |acc, (i, point)| {
            // find all neighboring numbers to point
            let neighbors = lava_tube.get_neighbors(i);

            // find if point is smaller than all its neighbors
            if neighbors.iter().all(|n| n.value > point.value) {
                acc + (point.value + 1) as i64
            } else {
                acc
            }
        })
}

pub fn part_2(input: &str) -> i64 {
    let lava_tube = LavaTube::new(input);

    let (_found, mut basins): (Vec<usize>, Vec<i32>) =
        lava_tube
            .points
            .iter()
            .fold(([].to_vec(), [].to_vec()), |(found, mut basins), point| {
                let (new_found, new_basin) = find_basin(&lava_tube, &found, point);

                if new_basin > 0 {
                    basins.push(new_basin);
                }

                (new_found, basins)
            });

    basins.sort_unstable();

    basins
        .iter()
        .rev()
        .take(3)
        .fold(1, |acc, x| acc * *x as i64)
}

fn find_basin(lava_tube: &LavaTube, prev_found: &[usize], point: &Point) -> (Vec<usize>, i32) {
    // point is already in a basin or is not a basin
    if prev_found.contains(&point.position) || point.value == 9 {
        return (prev_found.to_vec(), 0);
    }

    let mut found = prev_found.to_vec();
    found.push(point.position);

    // find all neighboring numbers to point
    let neighbors = lava_tube.get_neighbors(point.position);
    let (new_found, basin_size) = neighbors
        .iter()
        .fold((found, 1), |(f, basin_size), neighbor| {
            let (new_found, new_size) = find_basin(lava_tube, &f, neighbor);
            (new_found, basin_size + new_size)
        });

    (new_found, basin_size)
}

#[derive(Debug)]
struct Point {
    position: usize,
    value: u8,
}

// Use a 1 dimensional vector and a width to simulate a 2d array.
struct LavaTube {
    points: Vec<Point>,
    width: usize,
}

impl LavaTube {
    fn new(input: &str) -> LavaTube {
        let width = input.lines().next().unwrap().len();

        let points: Vec<Point> = input
            .replace("\n", "")
            .chars()
            .enumerate()
            .map(|(position, c)| Point {
                value: c.to_digit(10).unwrap() as u8,
                position,
            })
            .collect();

        LavaTube { points, width }
    }

    fn get_neighbors(&self, i: usize) -> Vec<&Point> {
        // use checked functions to avoid overflow
        let left = if i % self.width != 0 {
            i.checked_sub(1)
        } else {
            None
        };

        let right = if i % self.width != self.width - 1 {
            i.checked_add(1)
        } else {
            None
        };

        let up = i.checked_sub(self.width);
        let down = i.checked_add(self.width);

        [left, right, up, down]
            .iter()
            .filter_map(|position| match position {
                Some(position) => self.points.get(*position),
                None => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 1134);
    }
}

#[cfg(test)]
const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
