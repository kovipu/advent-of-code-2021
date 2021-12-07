pub fn part_1(input: &str) -> i64 {
    let input: Vec<i32> = input.split(',').map(|x| x.parse().unwrap()).collect();

    let min_position = &input.iter().min().unwrap().clone();
    let max_position = &input.iter().max().unwrap().clone();

    let possible_fuel_values = (*min_position..*max_position)
        .map(|pos| input.iter().map(|crab| (pos - crab).abs()).sum::<i32>());

    possible_fuel_values.min().unwrap() as i64
}

pub fn part_2(input: &str) -> i64 {
    let input: Vec<i32> = input.split(',').map(|x| x.parse().unwrap()).collect();

    let min_position = &input.iter().min().unwrap().clone();
    let max_position = &input.iter().max().unwrap().clone();

    let possible_fuel_values = (*min_position..*max_position).map(|pos| {
        input
            .iter()
            .map(|crab| {
                let distance = (pos - crab).abs();
                (0..=distance).sum::<i32>()
            })
            .sum::<i32>()
    });

    possible_fuel_values.min().unwrap() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("16,1,2,0,4,2,7,1,2,14"), 37);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("16,1,2,0,4,2,7,1,2,14"), 168);
    }
}
