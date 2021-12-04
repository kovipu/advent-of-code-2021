pub fn part_1(input: &str) -> i32 {
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

    position * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 150);
    }
}

#[cfg(test)]
const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
