fn main() {
    let input1 = include_str!("./input1.txt");
    let output1 = part_1(input1);
    println!("{output1}");
    let input2 = include_str!("./input1.txt");
    let output2 = part_2(input2);
    println!("{output2}");
}

fn part_1(input: &str) -> String {
    process_1(input)
}
fn part_2(input: &str) -> String {
    process_2(input)
}

fn process_1(input: &str) -> String {
    let puzzles = Puzzle::parse_all(input);
    let mut sum = 0;
    for puzzle in puzzles {
        sum += puzzle.horizontal_reflection();
        sum += puzzle.vertical_reflection();
    }

    sum.to_string()
}

fn process_2(input: &str) -> String {
    let puzzles = Puzzle::parse_all(input);
    let mut sum = 0;
    for puzzle in puzzles {
        let h = puzzle.horizontal_reflection_p2();
        sum += h;
        if h == 0 {
            sum += puzzle.vertical_reflection_p2();
        }
    }
    sum.to_string()
}

#[derive(Debug)]
struct Puzzle {
    data: Vec<Vec<char>>,
    nrows: usize,
    ncols: usize,
}

impl Puzzle {
    fn parse_all(input: &str) -> Vec<Puzzle> {
        let puzzles_str = input.split("\n\n");
        let mut puzzles: Vec<Puzzle> = Vec::new();
        for puzzle in puzzles_str {
            let mut data: Vec<Vec<char>> = Vec::new();
            for row in puzzle.lines().clone() {
                data.push(row.chars().collect());
            }
            let nrows = data.len();
            let ncols = data.first().unwrap().len();
            puzzles.push(Puzzle { data, nrows, ncols })
        }
        puzzles
    }

    fn horizontal_reflection(&self) -> usize {
        let mut reflection_index = 0;
        for idx in 1..self.nrows {
            let left_iter = (0..idx).rev();
            let right_iter = idx..self.nrows;
            if left_iter
                .zip(right_iter)
                .all(|row| self.data[row.0] == self.data[row.1])
            {
                reflection_index = idx * 100;
                break;
            }
        }
        reflection_index
    }

    fn vertical_reflection(&self) -> usize {
        let mut reflection_index = 0;
        let data_transform: Vec<Vec<char>> = (0..self.ncols)
            .map(|col| self.data.iter().map(|row| row[col]).collect())
            .collect();
        for idx in 1..self.ncols {
            let left_iter = (0..idx).rev();
            let right_iter = idx..self.ncols;
            if left_iter
                .zip(right_iter)
                .all(|col| data_transform[col.0] == data_transform[col.1])
            {
                reflection_index = idx;
                break;
            }
        }
        reflection_index
    }

    fn horizontal_reflection_p2(&self) -> usize {
        let original_reflection_horizontal = self.horizontal_reflection();
        let mut reflection_index = 0;
        'outer: for y in 0..self.nrows {
            for x in 0..self.ncols {
                let mut new_data = self.data.clone();
                let c = new_data[y][x];
                match c {
                    '#' => new_data[y][x] = '.',
                    '.' => new_data[y][x] = '#',
                    _ => panic!("invalid char"),
                };
                for idx in 1..self.nrows {
                    let left_iter = (0..idx).rev();
                    let right_iter = idx..self.nrows;

                    if left_iter
                        .zip(right_iter)
                        .all(|row| new_data[row.0] == new_data[row.1])
                        && (idx * 100) != original_reflection_horizontal
                    {
                        reflection_index = idx * 100;
                        break 'outer;
                    }
                }
            }
        }
        reflection_index
    }

    fn vertical_reflection_p2(&self) -> usize {
        let original_reflection_vertical = self.vertical_reflection();
        let mut reflection_index = 0;
        'outer: for y in 0..self.nrows {
            for x in 0..self.ncols {
                let mut new_data = self.data.clone();
                let c = new_data[y][x];
                match c {
                    '#' => new_data[y][x] = '.',
                    '.' => new_data[y][x] = '#',
                    _ => panic!("invalid char"),
                };
                let data_transform: Vec<Vec<char>> = (0..self.ncols)
                    .map(|col| new_data.iter().map(|row| row[col]).collect())
                    .collect();
                for idx in 1..self.ncols {
                    let left_iter = (0..idx).rev();
                    let right_iter = idx..self.ncols;

                    if left_iter
                        .zip(right_iter)
                        .all(|col| data_transform[col.0] == data_transform[col.1])
                        && idx != original_reflection_vertical
                    {
                        reflection_index = idx;
                        break 'outer;
                    }
                }
            }
        }
        reflection_index
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    #[test]
    fn test_1() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "405".to_string())
    }
    #[test]
    fn test_2() {
        let output = part_2(EXAMPLE_TEXT);
        assert_eq!(output, "400".to_string())
    }
}
