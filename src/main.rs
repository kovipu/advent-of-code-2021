use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    for day in 1..=5 {
        let input_filename = format!("inputs/{:02}.txt", day);
        let input = fs::read_to_string(input_filename).unwrap();
        let (part_1, part_2) = get_day(day);

        println!();
        println!("Running solution for day {}...", day);
        println!("Part 1: {}", part_1(&input));
        println!("Part 2: {}", part_2(&input));
    }
    println!();
}

fn get_day(day: u32) -> (DayFn, DayFn) {
    match day {
        1 => (day01::part_1, day01::part_2),
        2 => (day02::part_1, day02::part_2),
        3 => (day03::part_1, day03::part_2),
        4 => (day04::part_1, day04::part_2),
        5 => (day05::part_1, day05::part_2),
        _ => panic!("Day {} not implemented", day),
    }
}

type DayFn = fn(&str) -> i32;
