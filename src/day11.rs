pub fn part_1(input: &str) -> i64 {
    let width: usize = input.lines().next().unwrap().len();
    let mut octopi: Vec<u8> = input
        .replace("\n", "")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut flashed_octopi: i64 = 0;

    let ticks = 100;
    for _ in 0..ticks {
        let (flashed, _) = simulate_tick(&mut octopi, width);
        flashed_octopi += flashed as i64;
    }

    flashed_octopi
}

pub fn part_2(input: &str) -> i64 {
    let width: usize = input.lines().next().unwrap().len();
    let mut octopi: Vec<u8> = input
        .replace("\n", "")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let ticks = 1000;
    for i in 1..ticks {
        let (_, all_flashed) = simulate_tick(&mut octopi, width);
        if all_flashed {
            return i as i64;
        }
    }
    panic!("You probably need to increase the number of ticks");
}

fn simulate_tick(octopi: &mut Vec<u8>, width: usize) -> (usize, bool) {
    // make a vector of same length as input to keep state of which octopi have flashed
    let mut flashed_octopi: Vec<bool> = vec![false; octopi.len()];

    // first increment each octopus by 1
    octopi.iter_mut().for_each(|octopus| {
        *octopus += 1;
    });

    loop {
        let mut flashed: bool = false;

        // increment numbers > 9 neighbors' by one
        // -> if no one got incremented, break
        for (i, octopus) in octopi.to_vec().iter().enumerate() {
            if *octopus < 10 || flashed_octopi[i] {
                continue;
            }

            flashed_octopi[i] = true;
            flashed = true;

            // increment neighbors, check for wrapping.
            let top_left = if i % width != 0 {
                i.checked_sub(width + 1)
            } else {
                None
            };
            let top = i.checked_sub(width);
            let top_right = if i % width != width - 1 {
                i.checked_sub(width - 1)
            } else {
                None
            };
            let right = if i % width != width - 1 {
                i.checked_add(1)
            } else {
                None
            };
            let bottom_right = if i % width != width - 1 {
                i.checked_add(width + 1)
            } else {
                None
            };
            let bottom = i.checked_add(width);
            let bottom_left = if i % width != 0 {
                i.checked_add(width - 1)
            } else {
                None
            };
            let left = if i % width != 0 {
                i.checked_sub(1)
            } else {
                None
            };

            let neighbors = [
                top_left,
                top,
                top_right,
                right,
                bottom_right,
                bottom,
                bottom_left,
                left,
            ];

            for neighbor in neighbors.into_iter().flatten() {
                if let Some(octopus) = octopi.get_mut(neighbor) {
                    *octopus += 1;
                }
            }
        }

        if !flashed {
            break;
        }
    }

    octopi.iter_mut().for_each(|octopus| {
        if *octopus >= 10 {
            *octopus = 0;
        }
    });

    let flashed_total: usize = flashed_octopi.iter().filter(|b| **b).count();
    let all_flashed: bool = flashed_total == octopi.len();

    (flashed_total, all_flashed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_simulate_tick() {
        let mut octopi = vec![
            1,1,1,1,1,
            1,9,9,9,1,
            1,9,1,9,1,
            1,9,9,9,1,
            1,1,1,1,1,
        ];
        let expected = vec![
            3,4,5,4,3,
            4,0,0,0,4,
            5,0,0,0,5,
            4,0,0,0,4,
            3,4,5,4,3,
        ];
        let (flashed, _) = simulate_tick(&mut octopi, 5);

        assert_eq!(flashed, 9);
        assert_eq!(octopi, expected);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 1656);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 195);
    }
}

#[cfg(test)]
const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
