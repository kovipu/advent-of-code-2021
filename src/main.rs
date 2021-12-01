use std::fs;

fn main() {
    println!("Running solution for day 1...");

    let input = fs::read_to_string("inputs/01.txt")
            .expect("Something went wrong reading the input file");
    let output = solve(&input);

    println!("Solution: {}", output);
}

fn solve(input: &str) -> u32 {
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
    fn test_solve() {
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

        assert_eq!(solve(input), 7);
    }
}
