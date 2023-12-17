use std::collections::HashSet;

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

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_usize(&self) -> usize {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
        }
    }
}

#[derive(Debug)]
struct Laser {
    // start_position: (isize, isize),
    // start_direction: Direction,
    direction: Direction,
    current_position: (isize, isize),
    previous_positions: HashSet<(isize, isize)>,
    is_complete: bool, // left_grid: bool,
                       // is_loop: bool,
}

impl Laser {
    fn next(&mut self, grid: &Vec<Vec<char>>) -> Option<Vec<Laser>> {
        let x_max = grid.first().unwrap().len() as isize;
        let y_max = grid.len() as isize;
        if is_inside(self.current_position, x_max, y_max) {
            self.previous_positions.insert(self.current_position);
        } else {
            self.is_complete = true;
            return None;
        }
        let char = grid[self.current_position.1 as usize][self.current_position.0 as usize];

        let new_laser: Option<Vec<Laser>> = match (&self.direction, char) {
            (Direction::Right, '/') => {
                self.current_position.1 -= 1;
                self.direction = Direction::Up;
                None
            }
            (Direction::Left, '/') => {
                self.current_position.1 += 1;
                self.direction = Direction::Down;
                None
            }
            (Direction::Up, '/') => {
                self.current_position.0 += 1;
                self.direction = Direction::Right;
                None
            }
            (Direction::Down, '/') => {
                self.current_position.0 -= 1;
                self.direction = Direction::Left;
                None
            }
            (Direction::Right, '\\') => {
                self.current_position.1 += 1;
                self.direction = Direction::Down;
                None
            }
            (Direction::Left, '\\') => {
                self.current_position.1 -= 1;
                self.direction = Direction::Up;
                None
            }
            (Direction::Up, '\\') => {
                self.current_position.0 -= 1;
                self.direction = Direction::Left;
                None
            }
            (Direction::Down, '\\') => {
                self.current_position.0 += 1;
                self.direction = Direction::Right;
                None
            }
            (Direction::Right, '.') => {
                self.current_position.0 += 1;
                None
            }
            (Direction::Left, '.') => {
                self.current_position.0 -= 1;
                None
            }
            (Direction::Up, '.') => {
                self.current_position.1 -= 1;
                None
            }
            (Direction::Down, '.') => {
                self.current_position.1 += 1;
                None
            }

            (Direction::Up, '|') => {
                self.current_position.1 -= 1;
                None
            }
            (Direction::Down, '|') => {
                self.current_position.1 += 1;
                None
            }
            (Direction::Right, '|') => {
                self.is_complete = true;
                Some(vec![
                    Laser {
                        direction: Direction::Down,
                        current_position: self.current_position,
                        previous_positions: HashSet::new(),
                        is_complete: false,
                    },
                    Laser {
                        direction: Direction::Up,
                        current_position: self.current_position,
                        previous_positions: HashSet::new(),
                        is_complete: false,
                    },
                ])
            }
            (Direction::Left, '|') => {
                self.is_complete = true;
                Some(vec![
                    Laser {
                        direction: Direction::Down,
                        current_position: self.current_position,
                        previous_positions: HashSet::new(),
                        is_complete: false,
                    },
                    Laser {
                        direction: Direction::Up,
                        current_position: self.current_position,
                        previous_positions: HashSet::new(),
                        is_complete: false,
                    },
                ])
            }
            (Direction::Right, '-') => {
                self.current_position.0 += 1;
                None
            }
            (Direction::Left, '-') => {
                self.current_position.0 -= 1;
                None
            }
            (Direction::Up, '-') => {
                self.is_complete = true;
                Some(vec![
                    Laser {
                        direction: Direction::Left,
                        current_position: self.current_position,
                        previous_positions: HashSet::new(),
                        is_complete: false,
                    },
                    Laser {
                        direction: Direction::Right,
                        current_position: self.current_position,
                        previous_positions: HashSet::new(),
                        is_complete: false,
                    },
                ])
            }
            (Direction::Down, '-') => {
                self.is_complete = true;
                Some(vec![
                    Laser {
                        direction: Direction::Left,
                        current_position: self.current_position,
                        previous_positions: HashSet::new(),
                        is_complete: false,
                    },
                    Laser {
                        direction: Direction::Right,
                        current_position: self.current_position,
                        previous_positions: HashSet::new(),
                        is_complete: false,
                    },
                ])
            }
            (_, _) => None,
        };
        new_laser
    }
}

fn is_inside(coord: (isize, isize), x_max: isize, y_max: isize) -> bool {
    let mut is_inside = true;
    if coord.0 < 0 || coord.0 >= x_max {
        is_inside = false;
    }
    if coord.1 < 0 || coord.1 >= y_max {
        is_inside = false;
    }
    is_inside
}

fn print_grid(input: &Vec<Vec<char>>) {
    let x_max = input.first().unwrap().len();
    let y_max = input.len();
    for y in 0..y_max {
        println!();
        for x in 0..x_max {
            print!("{}", input[y][x]);
        }
    }
}

fn process_1(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    print_grid(&grid);
    let mut lasers: Vec<Laser> = vec![Laser {
        direction: Direction::Right,
        current_position: (0, 0),
        previous_positions: HashSet::new(),
        is_complete: false,
    }];
    let mut energized: HashSet<(isize, isize)> = HashSet::new();

    let mut lasers_started: HashSet<(isize, isize, usize)> = HashSet::new();
    loop {
        if lasers.iter().all(|l| l.is_complete) {
            break;
        }
        let mut new_lasers: Vec<Laser> = Vec::new();
        for laser in lasers.iter_mut() {
            let split_lasers = laser.next(&grid);
            if split_lasers.is_some() {
                for l in split_lasers.unwrap() {
                    new_lasers.push(l);
                }
            }
        }
        for new_laser in new_lasers {
            if lasers_started.contains(&(
                new_laser.current_position.0,
                new_laser.current_position.1,
                new_laser.direction.to_usize(),
            )) {
                continue;
            }

            lasers_started.insert((
                new_laser.current_position.0,
                new_laser.current_position.1,
                new_laser.direction.to_usize(),
            ));
            lasers.push(new_laser);
        }
    }
    for laser in lasers.iter() {
        for p in laser.previous_positions.clone() {
            energized.insert(p);
        }
    }

    let x_max = grid.first().unwrap().len();
    let y_max = grid.len();
    for y in 0..y_max as isize {
        println!();
        for x in 0..y_max as isize {
            if energized.contains(&(x, y)) {
                print!("*");
            } else {
                print!(".");
            }
        }
    }

    energized.len().to_string()
}

fn process_2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_1() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "46".to_string())
    }
    // #[test]
    // fn test_2() {
    //     let output = part_2(EXAMPLE_TEXT);
    //     assert_eq!(output, "".to_string())
    // }
}
