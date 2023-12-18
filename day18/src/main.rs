use core::panic;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input1 = include_str!("./input1.txt");
    let output1 = part_1(input1);
    println!("{output1}");
    // let input2 = include_str!("./input2.txt");
    // let output2 = part_2(input2);
    // println!("{output2}");
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
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: usize,
    colour: String,
}

impl Instruction {
    fn parse_all(input: &str) -> Vec<Instruction> {
        input
            .lines()
            .inspect(|x| println!("{x}"))
            .map(|l| {
                let mut split = l.split_whitespace();
                let direction = Direction::from_char(split.next().unwrap().chars().next().unwrap());
                let length = split.next().unwrap().parse::<usize>().unwrap();
                let colour = split
                    .next()
                    .unwrap()
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .to_string();
                Instruction {
                    direction,
                    length,
                    colour,
                }
            })
            .collect()
    }

    fn process(&self) -> (isize, isize) {
        match self.direction {
            Direction::Up => (0, -1 * self.length as isize),
            Direction::Down => (0, 1 * self.length as isize),
            Direction::Left => (-1 * self.length as isize, 0),
            Direction::Right => (1 * self.length as isize, 0),
        }
    }
}

// fn translate_perimeter(
//     perimeter: HashMap<(isize, isize), String>,
// ) -> HashMap<(isize, isize), String> {
//     let mut perimeter_translated: HashMap<(isize, isize), String> = HashMap::new();
//     let x_min = perimeter.keys().map(|p| p.0).min().unwrap();
//     let y_min = perimeter.keys().map(|p| p.1).min().unwrap();
//     perimeter.iter().for_each(|p| {
//         let mut point = *p.0;
//         point.0 -= x_min;
//         point.1 -= y_min;
//         perimeter_translated.insert(point, p.1.clone());
//     });
//     perimeter_translated
// }

// fn perimeter_to_vec(perimeter: HashMap<(isize, isize), String>) -> Vec<Vec<char>> {
//     let x_max = perimeter.keys().map(|p| p.0).max().unwrap();
//     let y_max = perimeter.keys().map(|p| p.1).max().unwrap();
//     let mut map: Vec<Vec<char>> = vec![vec!['.'; (x_max + 2) as usize]; (y_max + 1) as usize];
//     (0..=y_max).for_each(|y| {
//         (0..=x_max).for_each(|x| {
//             if perimeter.contains_key(&(x, y)) {
//                 map[y as usize][x as usize] = '#'
//             }
//         })
//     });
//     map
// }

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
    dbg!(&vertices);

    let perimeter_length = vertices
        .iter()
        .tuple_windows()
        .map(|(one, two)| {
            let distance = ((two.0 - one.0).abs(), (two.1 - one.1).abs());
            (distance.0 + distance.1) as i64
        })
        .sum::<i64>();
    // + {
    //     let one = vertices.iter().last().unwrap();
    //     let two = vertices.iter().next().unwrap();
    //     let distance = ((two.0 - one.0).abs(), (two.1 - one.1).abs());
    //     (distance.0 + distance.1) as i64
    // };
    dbg!(&perimeter_length);
    let area = ((vertices
        .iter()
        .tuple_windows()
        .map(|(one, two)| (one.0 * two.1 - one.1 * two.0) as i64)
        .sum::<i64>()
        + perimeter_length)
        / 2)
    .abs()
        + 1;
    dbg!(&area);
    (area).to_string()
}

fn process_2(input: &str) -> String {
    "".to_string()
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
    // #[test]
    // fn test_2() {
    //     let output = part_2(EXAMPLE_TEXT);
    //     assert_eq!(output, "".to_string())
    // }
}
