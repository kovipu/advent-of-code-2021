use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // day given as first argument
    if args.len() >= 2 {
        let day = args[1].parse().expect("Argument must be a number");
        run_day(day);
        println!();
        return;
    }

    // no argument, run all days
    for day in 1..=20 {
        run_day(day);
    }
    println!();
}

fn run_day(day: u8) {
    let input_filename = format!("inputs/{:02}.txt", day);
    let input = fs::read_to_string(input_filename).unwrap();
    let (part_1, part_2) = get_day(day);

    println!();
    println!("Running solution for day {}...", day);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn get_day(day: u8) -> (DayFn, DayFn) {
    match day {
        1 => (day01::part_1, day01::part_2),
        2 => (day02::part_1, day02::part_2),
        3 => (day03::part_1, day03::part_2),
        4 => (day04::part_1, day04::part_2),
        5 => (day05::part_1, day05::part_2),
        6 => (day06::part_1, day06::part_2),
        7 => (day07::part_1, day07::part_2),
        8 => (day08::part_1, day08::part_2),
        9 => (day09::part_1, day09::part_2),
        10 => (day10::part_1, day10::part_2),
        11 => (day11::part_1, day11::part_2),
        12 => (day12::part_1, day12::part_2),
        13 => (day13::part_1, day13::part_2),
        14 => (day14::part_1, day14::part_2),
        15 => (day15::part_1, day15::part_2),
        16 => (day16::part_1, day16::part_2),
        17 => (day17::part_1, day17::part_2),
        18 => (day18::part_1, day18::part_2),
        19 => (day19::part_1, day19::part_2),
        20 => (day20::part_1, day20::part_2),
        _ => panic!("Day {} not implemented", day),
    }
}

type DayFn = fn(&str) -> i64;
