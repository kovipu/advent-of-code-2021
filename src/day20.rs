pub fn part_1(input: &str) -> i64 {
    let parsed = input.split_once("\n\n").unwrap();
    let enhancement_algorithm = parsed.0;
    let mut image = Image::new(parsed.1, 7);

    image.enhance(enhancement_algorithm, false);
    image.enhance(enhancement_algorithm, true);

    image.count_pixels()
}

pub fn part_2(input: &str) -> i64 {
    let parsed = input.split_once("\n\n").unwrap();
    let enhancement_algorithm = parsed.0;

    // pad the input 55 pixels in each direction.
    let mut image = Image::new(parsed.1, 55);

    for i in 1..=50 {
        image.enhance(enhancement_algorithm, i % 2 == 0);
    }

    image.count_pixels()
}

struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Vec<char>>,
}

impl Image {
    fn new(input: &str, padding: usize) -> Image {
        let mut output = String::new();

        let original_width = input.lines().next().unwrap().len();

        for _ in 0..padding {
            output.push_str(&".".repeat(padding * 2 + original_width));
            output.push('\n');
        }

        for line in input.lines() {
            output.push_str(&".".repeat(padding as usize));
            output.push_str(line);
            output.push_str(&".".repeat(padding as usize));
            output.push('\n');
        }

        for _ in 0..padding {
            output.push_str(&".".repeat(padding * 2 + original_width));
            output.push('\n');
        }

        let width = output.lines().next().unwrap().len();
        let height = output.lines().count();

        let pixels = output.lines().map(|line| line.chars().collect()).collect();

        Image {
            width,
            height,
            pixels,
        }
    }

    fn enhance(&mut self, enhancement_algorithm: &str, flip: bool) {
        let mut output = String::new();

        // if the first char is lit, the pixels outside the original image just flip.
        let first_char = enhancement_algorithm.chars().next().unwrap();
        let padding: String = if first_char == '.' {
            if flip {
                "#"
            } else {
                "."
            }
        } else if flip {
            "."
        } else {
            "#"
        }
        .to_string();

        for _ in 0..2 {
            for _ in 0..(self.width) {
                output.push_str(&padding);
            }
            output.push('\n');
        }

        for y in 2..(self.height - 2) {
            output.push_str(&padding.repeat(2));

            for x in 2..(self.width - 2) {
                // collect the 9 bit binary number
                let neighbors = [
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y),
                    (x, y),
                    (x + 1, y),
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                ];

                let binary: String = neighbors
                    .map(|(x, y)| if self.pixels[y][x] == '#' { '1' } else { '0' })
                    .into_iter()
                    .collect();

                // look for that index in enchancement_algorithm
                let usize = usize::from_str_radix(&binary, 2).unwrap();
                output.push_str(
                    &enhancement_algorithm
                        .chars()
                        .nth(usize)
                        .unwrap()
                        .to_string(),
                );
            }

            output.push_str(&(padding.repeat(2) + "\n"));
        }

        for _ in 0..2 {
            for _ in 0..(self.width) {
                output.push_str(&padding);
            }
            output.push('\n');
        }

        self.pixels = output.lines().map(|line| line.chars().collect()).collect();
    }

    fn count_pixels(&self) -> i64 {
        self.pixels
            .iter()
            .map(|line| line.iter().filter(|&&c| c == '#').count() as i64)
            .sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 35);
    }
}

#[cfg(test)]
const INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
