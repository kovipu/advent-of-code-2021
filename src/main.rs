use std::fs;

mod day01;
mod day02;

fn main() {
    println!("Running solution for day 1...");

    let day_1_input =
        fs::read_to_string("inputs/01.txt").expect("Something went wrong reading the input file");

    let day_1_output_1 = day01::part_1(&day_1_input);
    println!("Part 1 output: {}", day_1_output_1);

    let day_1_output_2 = day01::part_2(&day_1_input);
    println!("Part 2 output: {}", day_1_output_2);

    println!("");
    println!("Running solution for day 2...");

    let day_2_input =
        fs::read_to_string("inputs/02.txt").expect("Something went wrong reading the input file");

    let day_2_output_1 = day02::part_1(&day_2_input);
    println!("Part 1 output: {}", day_2_output_1);
}
