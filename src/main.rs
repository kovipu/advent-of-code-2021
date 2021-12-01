use std::fs;

mod day01;

fn main() {
    println!("Running solution for day 1...");

    let input = fs::read_to_string("inputs/01.txt")
            .expect("Something went wrong reading the input file");
    let output = day01::part_1(&input);

    println!("Part 1 output: {}", output);
}
