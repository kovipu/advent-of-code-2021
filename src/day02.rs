pub fn part_1(input: &str) -> i64 {
    let (position, depth): (i32, i32) = input.lines().fold((0, 0), |(position, depth), line| {
        let words: Vec<&str> = line.split_whitespace().collect();
        let direction = words[0];
        let x: i32 = words[1].parse().unwrap();

        match direction {
            "forward" => (position + x, depth),
            "down" => (position, depth + x),
            "up" => (position, depth - x),
            _ => panic!("Unknown direction: {}", direction),
        }
    });

    (position * depth) as i64
}

pub fn part_2(input: &str) -> i64 {
    let (_aim, position, depth): (i32, i32, i32) =
        input
            .lines()
            .fold((0, 0, 0), |(aim, position, depth), line| {
                let words: Vec<&str> = line.split_whitespace().collect();
                let direction = words[0];
                let x: i32 = words[1].parse().unwrap();

                match direction {
                    "down" => (aim + x, position, depth),
                    "up" => (aim - x, position, depth),
                    "forward" => (aim, position + x, depth + (aim * x)),
                    _ => panic!("Unknown direction: {}", direction),
                }
            });

    (position * depth) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 150);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 900);
    }
}

#[cfg(test)]
const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
