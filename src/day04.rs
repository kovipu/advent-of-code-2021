pub fn part_1(input: &str) -> i64 {
    let drawn_numbers: Vec<i32> = get_drawn_numbers(input);

    let boards: Vec<Bingo> = input.split("\n\n").skip(1).map(Bingo::new).collect();

    find_winning_score(&boards, &drawn_numbers) as i64
}

pub fn part_2(input: &str) -> i64 {
    let drawn_numbers: Vec<i32> = get_drawn_numbers(input);

    let boards: Vec<Bingo> = input.split("\n\n").skip(1).map(Bingo::new).collect();

    find_losing_score(&boards, &drawn_numbers) as i64
}

fn get_drawn_numbers(input: &str) -> Vec<i32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn find_winning_score(boards: &[Bingo], drawn_numbers: &[i32]) -> i32 {
    for idx in 5..drawn_numbers.len() {
        let numbers = drawn_numbers[0..idx].to_vec();
        for board in boards {
            if board.is_bingo(&numbers) {
                return board.sum_unmarked(&numbers) * numbers[idx - 1];
            }
        }
    }
    0
}

fn find_losing_score(boards: &[Bingo], drawn_numbers: &[i32]) -> i32 {
    let mut boards_still_playing: Vec<Bingo> = boards.to_vec();

    for idx in 5..drawn_numbers.len() {
        let numbers = drawn_numbers[0..idx].to_vec();

        if boards_still_playing.len() == 1 && boards_still_playing[0].is_bingo(&numbers) {
            return boards_still_playing[0].sum_unmarked(&numbers) * numbers[idx - 1];
        }

        boards_still_playing.retain(|board| !board.is_bingo(&numbers));
    }
    0
}

#[derive(Debug, Clone)]
struct Bingo {
    board: Vec<Vec<i32>>,
}

impl Bingo {
    fn new(board: &str) -> Bingo {
        let lines: Vec<&str> = board.lines().collect::<Vec<_>>();
        let board = lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();
        Bingo { board }
    }

    fn is_bingo(&self, numbers: &[i32]) -> bool {
        let line_bingo = self
            .board
            .iter()
            .any(|row| row.iter().all(|&x| numbers.contains(&x)));

        let column_bingo = (0..5).any(|i| {
            self.board
                .iter()
                .map(|row| row[i])
                .all(|x| numbers.contains(&x))
        });

        line_bingo || column_bingo
    }

    fn sum_unmarked(&self, numbers: &[i32]) -> i32 {
        let mut sum = 0;
        for row in &self.board {
            for &num in row {
                if !numbers.contains(&num) {
                    sum += num;
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 4512);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 1924);
    }
}

#[cfg(test)]
const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
