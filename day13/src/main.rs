use itertools::Itertools;

fn main() {
    let input1 = include_str!("./input1.txt");
    let output1 = part_1(input1);
    println!("{output1}");
    // let input2 = include_str!("./input2.txt");
    // let output2 = part_2(input2);
    // println!("{output2}");
}

fn part_1(input: &str) -> String {
    process_1(input)
}
fn part_2(input: &str) -> String {
    process_2(input)
}

fn process_1(input: &str) -> String {
    let puzzles = Puzzle::parse_all(input);
    dbg!(&puzzles);
    let mut sum = 0;
    for puzzle in puzzles {
        sum += puzzle.horizontal_reflection();
        sum += puzzle.vertical_reflection();
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
        for idx in (1..self.nrows) {
            dbg!(&idx);
            let left_iter = (0..idx).rev();
            let right_iter = (idx..self.nrows);
            dbg!(&left_iter);
            dbg!(&right_iter);

            if left_iter
                .zip(right_iter)
                .inspect(|x| println!("{:?}", x))
                .all(|row| self.data[row.0] == self.data[row.1])
            {
                reflection_index = idx * 100;
                break;
            }
        }
        dbg!(&reflection_index);

        reflection_index
    }

    fn vertical_reflection(&self) -> usize {
        let mut reflection_index = 0;
        let data_transform: Vec<Vec<char>> = (0..self.ncols)
            .map(|col| self.data.iter().map(|row| row[col]).collect())
            .collect();
        for idx in (1..self.ncols) {
            dbg!(&idx);
            let left_iter = (0..idx).rev();
            let right_iter = (idx..self.ncols);
            dbg!(&left_iter);
            dbg!(&right_iter);

            if left_iter
                .zip(right_iter)
                .inspect(|x| println!("{:?}", x))
                .all(|col| data_transform[col.0] == data_transform[col.1])
            {
                reflection_index = idx;
                break;
            }
        }
        dbg!(&reflection_index);

        reflection_index
    }
}

fn process_2(input: &str) -> String {
    "".to_string()
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
        assert_eq!(output, "".to_string())
    }
}
