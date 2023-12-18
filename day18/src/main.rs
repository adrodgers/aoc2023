use core::panic;
use itertools::Itertools;

fn main() {
    let input1 = include_str!("./input1.txt");
    let output1 = part_1(input1);
    println!("{output1}");
    let input2 = include_str!("./input1.txt");
    let output2 = part_2(input2);
    println!("{output2}");
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            '3' => Direction::Up,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '0' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: usize,
}

impl Instruction {
    fn parse_all(input: &str) -> Vec<Instruction> {
        input
            .lines()
            .map(|l| {
                let mut split = l.split_whitespace();
                let direction = Direction::from_char(split.next().unwrap().chars().next().unwrap());
                let length = split.next().unwrap().parse::<usize>().unwrap();
                // let colour = split
                //     .next()
                //     .unwrap()
                //     .trim_start_matches('(')
                //     .trim_end_matches(')')
                //     .to_string();
                Instruction { direction, length }
            })
            .collect()
    }

    fn parse_all_p2(input: &str) -> Vec<Instruction> {
        input
            .lines()
            .map(|l| {
                let split = l.split_whitespace();
                let hex = split
                    .last()
                    .unwrap()
                    .trim_start_matches("(#")
                    .trim_end_matches(')');
                let direction = Direction::from_char(hex.chars().last().unwrap());
                let length = usize::from_str_radix(&hex[0..5], 16).unwrap();
                Instruction { direction, length }
            })
            .collect()
    }

    fn process(&self) -> (isize, isize) {
        match self.direction {
            Direction::Up => (0, -(self.length as isize)),
            Direction::Down => (0, self.length as isize),
            Direction::Left => (-(self.length as isize), 0),
            Direction::Right => (self.length as isize, 0),
        }
    }
}

fn part_1(input: &str) -> String {
    process_1(input)
}
fn part_2(input: &str) -> String {
    process_2(input)
}

fn process_1(input: &str) -> String {
    let instructions = Instruction::parse_all(input);
    let mut vertices: Vec<(isize, isize)> = Vec::new();

    let mut pos = (0, 0);
    vertices.push(pos);
    instructions.iter().for_each(|i| {
        let ins = i.process();
        let new_pos = (pos.0 + ins.0, pos.1 + ins.1);
        vertices.push(new_pos);
        pos = new_pos;
    });

    let perimeter_length = vertices
        .iter()
        .tuple_windows()
        .map(|(one, two)| {
            let distance = ((two.0 - one.0).abs(), (two.1 - one.1).abs());
            (distance.0 + distance.1) as i64
        })
        .sum::<i64>();
    let area = ((vertices
        .iter()
        .tuple_windows()
        .map(|(one, two)| (one.0 * two.1 - one.1 * two.0) as i64)
        .sum::<i64>()
        + perimeter_length)
        / 2)
    .abs()
        + 1;
    (area).to_string()
}

fn process_2(input: &str) -> String {
    let instructions = Instruction::parse_all_p2(input);
    let mut vertices: Vec<(isize, isize)> = Vec::new();

    let mut pos = (0, 0);
    vertices.push(pos);
    instructions.iter().for_each(|i| {
        let ins = i.process();
        let new_pos = (pos.0 + ins.0, pos.1 + ins.1);
        vertices.push(new_pos);
        pos = new_pos;
    });

    let perimeter_length = vertices
        .iter()
        .tuple_windows()
        .map(|(one, two)| {
            let distance = ((two.0 - one.0).abs(), (two.1 - one.1).abs());
            (distance.0 + distance.1) as i64
        })
        .sum::<i64>();
    let area = ((vertices
        .iter()
        .tuple_windows()
        .map(|(one, two)| (one.0 * two.1 - one.1 * two.0) as i64)
        .sum::<i64>()
        + perimeter_length)
        / 2)
    .abs()
        + 1;
    (area).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_1() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "62".to_string())
    }
    #[test]
    fn test_2() {
        let output = part_2(EXAMPLE_TEXT);
        assert_eq!(output, "952408144115".to_string())
    }
}
