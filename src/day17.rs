use regex::Regex;
use std::ops::Range;

pub fn part_1(input: &str) -> i64 {
    let mut probe = Probe::new(input);
    let mut highest_y: i32 = std::i32::MIN;

    // brute force velocity simulations
    // if the probe hits target, and y value reaches it's highes -> save new highest y

    for vel_x in 1..100 {
        for vel_y in 1..1000 {
            probe.reset(vel_x, vel_y);

            let mut probe_highest_y = std::i32::MIN;

            while probe.state == State::Moving {
                probe.step();

                if probe.y > probe_highest_y {
                    probe_highest_y = probe.y;
                }
            }

            if probe.state == State::InTarget && probe_highest_y > highest_y {
                highest_y = probe_highest_y;
            }
        }
    }
    highest_y as i64
}

pub fn part_2(input: &str) -> i64 {
    let mut probe = Probe::new(input);
    let mut possible_velocities: i32 = 0;

    // brute force velocity simulations
    // if the probe hits target -> increment possible velocities

    for vel_x in 1..200 {
        for vel_y in (Range {
            start: -200,
            end: 3000,
        }) {
            probe.reset(vel_x, vel_y);

            while probe.state == State::Moving {
                probe.step();
            }

            if probe.state == State::InTarget {
                possible_velocities += 1;
            }
        }
    }

    possible_velocities as i64
}

#[derive(Debug)]
struct Probe {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
    target: Target,
    state: State,
}

#[derive(Debug)]
struct Target {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
}

#[derive(Debug, PartialEq)]
enum State {
    Moving,
    InTarget,
    Missed,
}

impl Probe {
    fn new(input: &str) -> Probe {
        let re = Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
        let caps = re.captures(input).unwrap();

        let x_start: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let x_end: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let y_start: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let y_end: i32 = caps.get(4).unwrap().as_str().parse().unwrap();

        Probe {
            x: 0,
            y: 0,
            vel_x: 1,
            vel_y: 1,
            target: Target {
                x_start,
                x_end,
                y_start,
                y_end,
            },
            state: State::Moving,
        }
    }

    fn reset(&mut self, vel_x: i32, vel_y: i32) {
        self.x = 0;
        self.y = 0;
        self.vel_x = vel_x;
        self.vel_y = vel_y;
        self.state = State::Moving;
    }

    fn step(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;

        if self.vel_x > 0 {
            self.vel_x -= 1;
        } else if self.vel_x < 0 {
            self.vel_x += 1;
        }

        self.vel_y -= 1;

        if self.y >= self.target.y_start
            && self.y <= self.target.y_end
            && self.x >= self.target.x_start
            && self.x <= self.target.x_end
        {
            self.state = State::InTarget;
        } else if self.y < self.target.y_start || self.x > self.target.x_end {
            self.state = State::Missed;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 45);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 112);
    }
}

#[cfg(test)]
const INPUT: &str = "target area: x=20..30, y=-10..-5";
