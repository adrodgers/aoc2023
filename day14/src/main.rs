use std::collections::{BTreeMap, HashMap};

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

#[derive(Debug)]
struct Tile {
    char: char,
    is_stationary: bool,
}

fn process_1(input: &str) -> String {
    let mut mirrors: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let x_max = mirrors.first().unwrap().len();
    let y_max = mirrors.len();
    dbg!(x_max, y_max);
    let mut mirror_set: BTreeMap<(usize, usize), Tile> = BTreeMap::new();
    for x in 0..x_max {
        for y in 0..y_max {
            let c = mirrors[y][x];
            let mut is_stationary = false;
            if c == '#' || (y == 0 && c == 'O') {
                is_stationary = true;
            }
            if y > 0 && mirror_set.get(&(x, y - 1)).unwrap().is_stationary && c == 'O' {
                is_stationary = true;
            }
            mirror_set.insert(
                (x, y),
                Tile {
                    char: c,
                    is_stationary,
                },
            );
        }
    }
    for y in 0..y_max {
        println!();
        for x in 0..x_max {
            print!("{}", mirror_set.get(&(x, y)).unwrap().char);
        }
    }
    // north
    for x in 0..x_max {
        for y in 0..y_max {
            let tile = mirror_set.get(&(x, y)).unwrap();
            if tile.is_stationary || tile.char == '.' || tile.char == '#' {
                continue;
            } else {
                let mut new_y = 0;
                for dy in (0..y).rev() {
                    let c_above = mirror_set.get(&(x, dy)).unwrap();
                    if c_above.is_stationary {
                        new_y = dy + 1;
                        break;
                    }
                    if dy == 0 {
                        new_y = 0;
                    }
                }
                let new_tile = mirror_set.get_mut(&(x, new_y)).unwrap();
                new_tile.char = 'O';
                new_tile.is_stationary = true;
                let tile = mirror_set.get_mut(&(x, y)).unwrap();
                tile.char = '.';
                tile.is_stationary = false;
            }
        }
    }
    for y in 0..y_max {
        println!();
        for x in 0..x_max {
            print!("{}", mirror_set.get(&(x, y)).unwrap().char);
        }
    }
    println!();

    let mut total_zeros = 0;
    for y in 0..y_max {
        let mut zeros_count = 0;
        for x in 0..x_max {
            let c = mirror_set.get(&(x, y)).unwrap();
            if c.char == 'O' {
                zeros_count += 1;
            }
        }
        total_zeros += zeros_count * (y_max - y);
    }
    total_zeros.to_string()
}

fn process_2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_1() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "136".to_string())
    }
    // #[test]
    // fn test_2() {
    //     let output = part_2(EXAMPLE_TEXT);
    //     assert_eq!(output, "".to_string())
    // }
}
