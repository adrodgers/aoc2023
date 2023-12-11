use colored::Colorize;
use std::collections::BTreeMap;

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
    // let y_max = input.lines().count();
    // let x_max = input.lines().next().unwrap().len();
    // dbg!(y_max);
    // dbg!(x_max);
    let mut tiles: BTreeMap<(isize, isize), char> = BTreeMap::new();
    let mut start_pos: (isize, isize) = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                // J for real input
                // tiles.insert((x, y), 'J');
                // F for test input
                // tiles.insert((x, y), 'F');
                start_pos = (x as isize, y as isize);
                tiles.insert((x as isize, y as isize), char);
            //             // println!("S: {x},{y}");
            } else {
                tiles.insert((x as isize, y as isize), char);
            }
        }
    }
    // dbg!(&start_pos);
    // dbg!(&tiles.get(&start_pos).unwrap());

    // replace start char
    let neighbors = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut valid_neighbors: Vec<(isize, isize)> = Vec::new();
    for neighbor in neighbors {
        if (start_pos.0 + neighbor.0) > 0 && (start_pos.1 + neighbor.1) > 0 {
            let char = tiles.get(&(
                start_pos.0.saturating_add(neighbor.0),
                start_pos.1.saturating_add(neighbor.1),
            ));
            //         dbg!(&char);
            //         dbg!(&neighbor);
            let pos = map_exit_position(
                *char.unwrap(),
                start_pos,
                (start_pos.0 + neighbor.0, start_pos.1 + neighbor.1),
            );
            //         dbg!(&pos);
            if pos.is_some() {
                valid_neighbors.push((neighbor.0, neighbor.1));
            }
        }
    }
    // dbg!(&tiles);
    // dbg!(&tiles.get(&start_pos).unwrap());
    // dbg!(&valid_neighbors);
    let mut prev_pos = (0, 0);
    if (valid_neighbors[0] == (-1, 0) || valid_neighbors[1] == (-1, 0))
        && (valid_neighbors[0] == (1, 0) || valid_neighbors[1] == (1, 0))
    {
        tiles.insert(start_pos, '-');
        prev_pos = (
            valid_neighbors[0].0 + start_pos.0,
            valid_neighbors[0].1 + start_pos.1,
        );
    }
    if (valid_neighbors[0] == (0, 1) || valid_neighbors[1] == (0, 1))
        && (valid_neighbors[0] == (0, -1) || valid_neighbors[1] == (0, -1))
    {
        tiles.insert(start_pos, '|');
        prev_pos = (
            valid_neighbors[0].0 + start_pos.0,
            valid_neighbors[0].1 + start_pos.1,
        );
    }
    if (valid_neighbors[0] == (-1, 0) || valid_neighbors[1] == (-1, 0))
        && (valid_neighbors[0] == (0, -1) || valid_neighbors[1] == (0, -1))
    {
        tiles.insert(start_pos, 'J');
        prev_pos = (
            valid_neighbors[0].0 + start_pos.0,
            valid_neighbors[0].1 + start_pos.1,
        );
    }
    if (valid_neighbors[0] == (1, 0) || valid_neighbors[1] == (1, 0))
        && (valid_neighbors[0] == (0, -1) || valid_neighbors[1] == (0, -1))
    {
        tiles.insert(start_pos, 'L');
        prev_pos = (
            valid_neighbors[0].0 + start_pos.0,
            valid_neighbors[0].1 + start_pos.1,
        );
    }
    if (valid_neighbors[0] == (1, 0) || valid_neighbors[1] == (1, 0))
        && (valid_neighbors[0] == (0, 1) || valid_neighbors[1] == (0, 1))
    {
        tiles.insert(start_pos, 'F');
        prev_pos = (
            valid_neighbors[0].0 + start_pos.0,
            valid_neighbors[0].1 + start_pos.1,
        );
    }
    if (valid_neighbors[0] == (-1, 0) || valid_neighbors[1] == (-1, 0))
        && (valid_neighbors[0] == (0, 1) || valid_neighbors[1] == (0, 1))
    {
        tiles.insert(start_pos, '7');
        prev_pos = (
            valid_neighbors[0].0 + start_pos.0,
            valid_neighbors[0].1 + start_pos.1,
        );
    }
    // dbg!(&tiles.get(&start_pos).unwrap());

    let mut pos = start_pos;
    let mut steps = 0;
    loop {
        steps += 1;
        //     dbg!(&pos);
        //     dbg!(&prev_pos);
        let new_pos = map_exit_position(*tiles.get(&pos).unwrap(), prev_pos, pos);
        if new_pos.unwrap() == start_pos {
            break;
        }
        prev_pos = pos;
        pos = new_pos.unwrap();
    }

    (steps / 2).to_string()
}

fn map_exit_position(
    char: char,
    entry_pos: (isize, isize),
    pos: (isize, isize),
) -> Option<(isize, isize)> {
    let pos_diff: (i32, i32) = (
        pos.0 as i32 - entry_pos.0 as i32,
        pos.1 as i32 - entry_pos.1 as i32,
    );
    match (pos_diff, char) {
        ((0, 1), '|') => Some((pos.0, pos.1 + 1)),
        ((0, -1), '|') => Some((pos.0, pos.1 - 1)),
        ((1, 0), '-') => Some((pos.0 + 1, pos.1)),
        ((-1, 0), '-') => Some((pos.0 - 1, pos.1)),
        ((-1, 0), 'L') => Some((pos.0, pos.1 - 1)),
        ((0, 1), 'L') => Some((pos.0 + 1, pos.1)),
        ((1, 0), 'J') => Some((pos.0, pos.1 - 1)),
        ((0, 1), 'J') => Some((pos.0 - 1, pos.1)),
        ((1, 0), '7') => Some((pos.0, pos.1 + 1)),
        ((0, -1), '7') => Some((pos.0 - 1, pos.1)),
        ((0, -1), 'F') => Some((pos.0 + 1, pos.1)),
        ((-1, 0), 'F') => Some((pos.0, pos.1 + 1)),
        _ => None,
    }
}

fn process_2(input: &str) -> String {
    let y_max = input.lines().count();
    let x_max = input.lines().next().unwrap().len();
    // dbg!(y_max);
    // dbg!(x_max);
    let mut tiles: BTreeMap<(isize, isize), char> = BTreeMap::new();
    let mut start_pos: (isize, isize) = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                // J for real input
                // tiles.insert((x, y), 'J');
                // F for test input
                // tiles.insert((x, y), 'F');
                start_pos = (x as isize, y as isize);
                tiles.insert((x as isize, y as isize), char);
            //             // println!("S: {x},{y}");
            } else {
                tiles.insert((x as isize, y as isize), char);
            }
        }
    }
    // dbg!(&start_pos);
    // dbg!(&tiles.get(&start_pos).unwrap());

    // replace start char
    let neighbors = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut valid_neighbors: Vec<(isize, isize)> = Vec::new();
    for neighbor in neighbors {
        if (start_pos.0 + neighbor.0) > 0 && (start_pos.1 + neighbor.1) > 0 {
            let char = tiles.get(&(
                start_pos.0.saturating_add(neighbor.0),
                start_pos.1.saturating_add(neighbor.1),
            ));
            //         dbg!(&char);
            //         dbg!(&neighbor);
            let pos = map_exit_position(
                *char.unwrap(),
                start_pos,
                (start_pos.0 + neighbor.0, start_pos.1 + neighbor.1),
            );
            //         dbg!(&pos);
            if pos.is_some() {
                valid_neighbors.push((neighbor.0, neighbor.1));
            }
        }
    }
    // dbg!(&tiles);
    // dbg!(&tiles.get(&start_pos).unwrap());
    // dbg!(&valid_neighbors);
    let mut prev_pos = (0, 0);
    if (valid_neighbors[0] == (-1, 0) || valid_neighbors[1] == (-1, 0))
        && (valid_neighbors[0] == (1, 0) || valid_neighbors[1] == (1, 0))
    {
        tiles.insert(start_pos, '-');
        prev_pos = (
            valid_neighbors[0].0 + start_pos.0,
            valid_neighbors[0].1 + start_pos.1,
        );
    }
    if (valid_neighbors[0] == (0, 1) || valid_neighbors[1] == (0, 1))
        && (valid_neighbors[0] == (0, -1) || valid_neighbors[1] == (0, -1))
    {
        tiles.insert(start_pos, '|');
        prev_pos = (
            valid_neighbors[0].0 + start_pos.0,
            valid_neighbors[0].1 + start_pos.1,
        );
    }
    if (valid_neighbors[0] == (-1, 0) || valid_neighbors[1] == (-1, 0))
        && (valid_neighbors[0] == (0, -1) || valid_neighbors[1] == (0, -1))
    {
        tiles.insert(start_pos, 'J');
        prev_pos = (
            valid_neighbors[0].0 + start_pos.0,
            valid_neighbors[0].1 + start_pos.1,
        );
    }
    if (valid_neighbors[0] == (1, 0) || valid_neighbors[1] == (1, 0))
        && (valid_neighbors[0] == (0, -1) || valid_neighbors[1] == (0, -1))
    {
        tiles.insert(start_pos, 'L');
        prev_pos = (
            valid_neighbors[0].0 + start_pos.0,
            valid_neighbors[0].1 + start_pos.1,
        );
    }
    if (valid_neighbors[0] == (1, 0) || valid_neighbors[1] == (1, 0))
        && (valid_neighbors[0] == (0, 1) || valid_neighbors[1] == (0, 1))
    {
        tiles.insert(start_pos, 'F');
        prev_pos = (
            valid_neighbors[0].0 + start_pos.0,
            valid_neighbors[0].1 + start_pos.1,
        );
    }
    if (valid_neighbors[0] == (-1, 0) || valid_neighbors[1] == (-1, 0))
        && (valid_neighbors[0] == (0, 1) || valid_neighbors[1] == (0, 1))
    {
        tiles.insert(start_pos, '7');
        prev_pos = (
            valid_neighbors[0].0 + start_pos.0,
            valid_neighbors[0].1 + start_pos.1,
        );
    }
    let mut pos = start_pos;
    let mut steps = 0;
    loop {
        steps += 1;
        let char = *tiles.get(&pos).unwrap();
        let new_pos = map_exit_position(char, prev_pos, pos);

        if char == '|' || char == 'L' || char == 'J' {
            tiles.insert(pos, 'N');
        } else if char == '-' || char == 'F' || char == '7' {
            tiles.insert(pos, 'S');
        } else {
            tiles.insert(pos, 'O');
        }

        if new_pos.unwrap() == start_pos {
            break;
        }
        prev_pos = pos;
        pos = new_pos.unwrap();
    }
    let mut is_inside_sum = 0;

    for y in 0..y_max as isize {
        for x in 0..x_max as isize {
            let mut is_inside = false;
            let char = tiles.get(&(x, y)).unwrap();
            if *char == 'N' || *char == 'O' || *char == 'S' {
                continue;
            }
            // let mut pipe_crossings = 0;
            for i in 0..x {
                let c = tiles.get(&(i, y)).unwrap();
                if c == &'N' {
                    // pipe_crossings += 1;
                    is_inside = !is_inside;
                }
            }
            if is_inside {
                tiles.insert((x, y), '*');
                is_inside_sum += 1;
            }
        }
    }
    dbg!(y_max);
    dbg!(x_max);
    for y in 0..y_max as isize {
        println!();
        for x in 0..x_max as isize {
            let char = *tiles.get(&(x, y)).unwrap();
            if char == 'N' || char == 'O' || char == 'S' {
                print!("{}", char.to_string().red());
            } else if char == '*' {
                print!("{}", char.to_string().yellow());
            } else {
                print!("{}", char.to_string().blue());
            }
        }
    }

    (is_inside_sum).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
    const EXAMPLE_TEXT_2: &str = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";
    const EXAMPLE_TEXT_3: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    #[test]
    fn test_1() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "4".to_string())
    }
    #[test]
    fn test_2_a() {
        let output = part_2(EXAMPLE_TEXT_2);
        assert_eq!(output, "4".to_string())
    }
    #[test]
    fn test_2_b() {
        let output = part_2(EXAMPLE_TEXT_3);
        assert_eq!(output, "8".to_string())
    }
}
