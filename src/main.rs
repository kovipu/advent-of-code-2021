use std::fs;

mod day01;

fn main() {
    println!("Running solution for day 1...");

    let input =
        fs::read_to_string("inputs/01.txt").expect("Something went wrong reading the input file");

    let output_1 = day01::part_1(&input);
    println!("Part 1 output: {}", output_1);

    let output_2 = day01::part_2(&input);
    println!("Part 2 output: {}", output_2);
}
