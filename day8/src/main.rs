use core::panic;
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
    let mut lines = input.lines();
    let directions: &str = lines.next().unwrap();
    dbg!(&directions);

    let mut map: BTreeMap<String, (String, String)> = BTreeMap::new();
    let mut start = "".to_string();
    for line in lines.skip(1) {
        let mut split = line.split(" = ");
        let key = split.next().unwrap();
        if start.is_empty() {
            start.push_str(key);
        }
        let mut tuple = split
            .next()
            .unwrap()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(", ");
        map.insert(
            key.to_string(),
            (
                tuple.next().unwrap().to_string(),
                tuple.next().unwrap().to_string(),
            ),
        );
    }
    dbg!(&map.len());
    let mut finished = false;
    let mut directions_iter = directions.chars().cycle();
    let mut key = "AAA".to_string();

    let mut steps = 0;
    while !finished {
        steps += 1;
        let dir = directions_iter.next().unwrap();
        let new_key = match dir {
            'L' => map.get(&key).unwrap().0.clone(),
            'R' => map.get(&key).unwrap().1.clone(),
            _ => panic!("Direction invalid"),
        };
        if new_key == *"ZZZ" {
            finished = true;
        }
        key = new_key;
    }
    steps.to_string()
}

fn process_2(input: &str) -> String {
    let mut lines = input.lines();
    let directions: &str = lines.next().unwrap();
    dbg!(&directions);

    let mut map: BTreeMap<String, (String, String)> = BTreeMap::new();
    let mut start = "".to_string();
    for line in lines.skip(1) {
        let mut split = line.split(" = ");
        let key = split.next().unwrap();
        if start.is_empty() {
            start.push_str(key);
        }
        let mut tuple = split
            .next()
            .unwrap()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(", ");
        map.insert(
            key.to_string(),
            (
                tuple.next().unwrap().to_string(),
                tuple.next().unwrap().to_string(),
            ),
        );
    }
    let directions_iter = directions.chars().cycle();
    let mut keys: Vec<String> = Vec::new();
    for key in map.clone().into_keys() {
        if key.ends_with('A') {
            keys.push(key);
        }
    }

    // Approach taken from https://www.youtube.com/watch?v=embH_LiFh5k
    let result = keys
        .iter()
        .map(|key| {
            let mut current_node = key.clone();
            directions_iter
                .clone()
                .enumerate()
                .find_map(|(index, instruction)| {
                    let options = map
                        .get(&current_node)
                        .expect("always exist at a valid node");
                    let next_node = match instruction {
                        'L' => options.0.clone(),
                        'R' => options.1.clone(),
                        _ => panic!("Direction invalid"),
                    };
                    if next_node.ends_with('Z') {
                        Some(index + 1)
                    } else {
                        current_node = next_node;
                        None
                    }
                })
                .expect("should find a cycle")
        })
        .collect::<Vec<usize>>();

    // while !finished {
    //     steps += 1;
    //     idx += 1;
    //     if idx > max_idx {
    //         idx = 0;
    //     }
    //     dbg!(&keys);
    //     let dir = directions_iter.next().unwrap();
    //     for key in keys.iter_mut() {
    //         let new_key = match dir {
    // 'L' => map.get(key).unwrap().0.clone(),
    // 'R' => map.get(key).unwrap().1.clone(),
    // _ => panic!("Direction invalid"),
    //         };
    //         *key = new_key;
    //     }
    //     dbg!(&keys);
    //     keys.iter().for_each(|k| {
    //         if k.ends_with('Z') && cycles[idx] == 0 {
    //             cycles[idx] = steps;
    //             dbg!(&cycles[idx]);
    //         }
    //     });
    //     dbg!(&cycles);
    //     if cycles.iter().all(|k| *k != 0) {
    //         finished = true;
    //     }
    // }
    // dbg!(&cycles);
    let lcm = lcm(&result);
    lcm.to_string()
}

//
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_TEXT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_TEXT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_1() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "2".to_string())
    }
    #[test]
    fn test_1_a() {
        let output = part_1(EXAMPLE_TEXT_2);
        assert_eq!(output, "6".to_string())
    }
    #[test]
    fn test_2() {
        let output = part_2(EXAMPLE_TEXT_3);
        assert_eq!(output, "6".to_string())
    }
}
