use std::fs;

mod day01;
mod day02;

fn main() {
    for day in 1..=2 {
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

fn get_day(day: u32) -> (fn(&str) -> i32, fn(&str) -> i32) {
    match day {
        1 => (day01::part_1, day01::part_2),
        2 => (day02::part_1, day02::part_2),
        _ => panic!("Day {} not implemented", day),
    }
}
