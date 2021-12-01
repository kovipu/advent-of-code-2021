pub fn part_1(input: &str) -> u32 {
    let input: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut acc = 0;

    input.iter().enumerate().for_each(|(i, x)| {
        if i > 0 && x > &input[i - 1] {
            acc += 1;
        }
    });

    acc
}

pub fn part_2(input: &str) -> u32 {
    let input: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

    let windows = input.windows(3);

    let i1 = &windows;
    let i2 = &windows;

    i1.clone()
        .zip(i2.clone().skip(1))
        .fold(0, |acc, (prev, cur)| {
            if prev.iter().sum::<u32>() < cur.iter().sum::<u32>() {
                acc + 1
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let expected = 7;
        assert_eq!(part_1(INPUT), expected);
    }

    #[test]
    fn test_part_2() {
        let expected = 5;
        assert_eq!(part_2(INPUT), expected);
    }
}

const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";
