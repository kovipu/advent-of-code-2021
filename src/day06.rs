use std::collections::HashMap;

pub fn part_1(input: &str) -> i64 {
    let mut state: Vec<i32> = input.split(',').map(|fish| fish.parse().unwrap()).collect();

    let ticks = 80;
    for _ in 0..ticks {
        simulate_tick(&mut state);
    }

    state.len().try_into().unwrap()
}

fn simulate_tick(state: &mut Vec<i32>) {
    for (i, fish) in state.clone().iter().enumerate() {
        if *fish == 0 {
            state[i] = 6;
            state.push(8);
        } else {
            state[i] = fish - 1;
        }
    }
}

pub fn part_2(input: &str) -> i64 {
    let mut state: HashMap<u8, i64> = input.split(',').map(|fish| fish.parse().unwrap()).fold(
        HashMap::new(),
        |mut state, fish| {
            *state.entry(fish).or_insert(0) += 1;
            state
        },
    );

    let ticks = 256;
    for _ in 0..ticks {
        simulate_tick_efficient(&mut state);
    }

    state.values().sum()
}

fn simulate_tick_efficient(state: &mut HashMap<u8, i64>) {
    let zeros = *state.entry(0).or_insert(0);

    let new_zeros = *state.entry(1).or_insert(0);
    state.insert(0, new_zeros);

    let new_ones = *state.entry(2).or_insert(0);
    state.insert(1, new_ones);

    let new_twos = *state.entry(3).or_insert(0);
    state.insert(2, new_twos);

    let new_threes = *state.entry(4).or_insert(0);
    state.insert(3, new_threes);

    let new_fours = *state.entry(5).or_insert(0);
    state.insert(4, new_fours);

    let new_fives = *state.entry(6).or_insert(0);
    state.insert(5, new_fives);

    let new_sixes = *state.entry(7).or_insert(0) + zeros;
    state.insert(6, new_sixes);

    let new_sevens = *state.entry(8).or_insert(0);
    state.insert(7, new_sevens);

    state.insert(8, zeros);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("3,4,3,1,2"), 5934);
    }

    #[test]
    fn test_part_2() {
        let expected: i64 = 26984457539;
        assert_eq!(part_2("3,4,3,1,2"), expected);
    }

    #[test]
    fn test_simulate_tick() {
        let mut input = vec![2, 3, 2, 0, 1];
        let expected = vec![1, 2, 1, 6, 0, 8];
        simulate_tick(&mut input);
        assert_eq!(input, expected);
    }
}
