use std::collections::{BTreeMap, BTreeSet};

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
    let x_max = input.lines().next().unwrap().len();
    let y_max = input.lines().count();
    let mut galaxy_id: u64 = 1;
    let mut galaxy_map: BTreeMap<u64, (usize, usize)> = BTreeMap::new();
    let mut unoccupied_row: BTreeSet<usize> = BTreeSet::from_iter(0..x_max);
    let mut unoccupied_col: BTreeSet<usize> = BTreeSet::from_iter(0..y_max);
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.trim().chars().enumerate() {
            if char == '#' {
                galaxy_map.insert(galaxy_id, (x, y));
                galaxy_id += 1;
                unoccupied_row.remove(&y);
                unoccupied_col.remove(&x);
            }
        }
    }
    // dbg!(&unoccupied_row);
    // dbg!(&unoccupied_col);
    // dbg!(&galaxy_map);
    for (e, row) in unoccupied_row.into_iter().enumerate() {
        for (_key, loc) in galaxy_map.iter_mut() {
            if loc.1 > row + e {
                loc.1 += 1;
            }
        }
    }
    for (e, col) in unoccupied_col.into_iter().enumerate() {
        for (_key, loc) in galaxy_map.iter_mut() {
            if loc.0 > col + e {
                loc.0 += 1;
            }
        }
    }
    // dbg!(&galaxy_map);
    let mut pairs: BTreeMap<(u64, u64), u64> = BTreeMap::new();
    for galaxy1 in galaxy_map.clone() {
        //     dbg!(&galaxy1);
        for galaxy2 in galaxy_map.clone() {
            //         dbg!(&galaxy2);
            if galaxy1 != galaxy2
                && (!pairs.contains_key(&(galaxy1.0, galaxy2.0))
                    && !pairs.contains_key(&(galaxy2.0, galaxy1.0)))
            {
                let distance =
                    galaxy1.1 .0.abs_diff(galaxy2.1 .0) + galaxy1.1 .1.abs_diff(galaxy2.1 .1);
                pairs.insert((galaxy1.0, galaxy2.0), distance as u64);
                //             dbg!(&pairs);
            }
        }
    }

    let out: u64 = pairs.into_values().sum();
    out.to_string()
}

fn process_2(input: &str) -> String {
    let x_max = input.lines().next().unwrap().len();
    let y_max = input.lines().count();
    let mut galaxy_id: u64 = 1;
    let mut galaxy_map: BTreeMap<u64, (usize, usize)> = BTreeMap::new();
    let mut unoccupied_row: BTreeSet<usize> = BTreeSet::from_iter(0..x_max);
    let mut unoccupied_col: BTreeSet<usize> = BTreeSet::from_iter(0..y_max);
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.trim().chars().enumerate() {
            if char == '#' {
                galaxy_map.insert(galaxy_id, (x, y));
                galaxy_id += 1;
                unoccupied_row.remove(&y);
                unoccupied_col.remove(&x);
            }
        }
    }
    // dbg!(&unoccupied_row);
    // dbg!(&unoccupied_col);
    // dbg!(&galaxy_map);
    for (e, row) in unoccupied_row.into_iter().enumerate() {
        for (_key, loc) in galaxy_map.iter_mut() {
            if loc.1 > row + e * (1000000 - 1) {
                loc.1 += 1000000 - 1;
            }
        }
    }
    for (e, col) in unoccupied_col.into_iter().enumerate() {
        for (_key, loc) in galaxy_map.iter_mut() {
            if loc.0 > col + e * (1000000 - 1) {
                loc.0 += 1000000 - 1;
            }
        }
    }
    // dbg!(&galaxy_map);
    let mut pairs: BTreeMap<(u64, u64), u64> = BTreeMap::new();
    for galaxy1 in galaxy_map.clone() {
        //     dbg!(&galaxy1);
        for galaxy2 in galaxy_map.clone() {
            //         dbg!(&galaxy2);
            if galaxy1 != galaxy2
                && (!pairs.contains_key(&(galaxy1.0, galaxy2.0))
                    && !pairs.contains_key(&(galaxy2.0, galaxy1.0)))
            {
                let distance =
                    galaxy1.1 .0.abs_diff(galaxy2.1 .0) + galaxy1.1 .1.abs_diff(galaxy2.1 .1);
                pairs.insert((galaxy1.0, galaxy2.0), distance as u64);
                //             dbg!(&pairs);
            }
        }
    }

    let out: u64 = pairs.into_values().sum();
    out.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    //     #[test]
    //     fn test_expand() {
    //         let output = part_1(EXAMPLE_TEXT);
    //         assert_eq!(
    //             output,
    //             "....#........
    // .........#...
    // #............
    // .............
    // .............
    // ........#....
    // .#...........
    // ............#
    // .............
    // .............
    // .........#...
    // #....#......."
    //                 .to_string()
    //         )
    //     }
    #[test]
    fn test_shortest_distance() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "374".to_string())
    }
    // #[test]
    // fn test_shortest_distance_p2() {
    //     let output = part_2(EXAMPLE_TEXT);
    //     assert_eq!(output, "374".to_string())
    // }
}
