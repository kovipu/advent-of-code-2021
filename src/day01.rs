pub fn part_1(input: &str) -> u32 {
    let input: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut acc = 0;

    input.iter().enumerate().for_each(|(i, x)| {
        if i > 0 && x > &input[i - 1] {
            acc += 1;
        }
    });

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "199
200
208
210
200
207
240
269
260
263";

        assert_eq!(part_1(input), 7);
    }
}
