use std::collections::BTreeMap;

fn main() {
    let input1 = include_str!("./input1.txt");
    part_1(input1);
    // let input2 = include_str!("./input2.txt");
    // part_2(input2);
}

fn part_1(input: &str) -> String {
    let output = process_1(input);
    println!("{output}");
    output
}
fn part_2(input: &str) -> String {
    let output = process_2(input);
    println!("{output}");
    output
}

fn process_1(input: &str) -> String {
    let mut lines = input.lines().into_iter();
    let mut seeds: Vec<u64> = lines
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    dbg!(&seeds);
    let mut maps: Vec<Vec<Vec<u64>>> = Vec::new();
    let mut map: Vec<Vec<u64>> = Vec::new();
    let num_lines = lines.clone().count();
    for (i, line) in lines.enumerate() {
        // dbg!(&line);
        if !line.contains(':') && !line.is_empty() {
            let line_data: Vec<u64> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            // dbg!(&line_data);
            map.push(line_data);
        }
        if (line.is_empty() || i == num_lines) && !map.is_empty() {
            maps.push(map.clone());
            dbg!(&map);
            map.clear();
        }
    }
    dbg!(&maps.len());
    for seed in seeds.iter_mut() {
        println!("seed: ");
        dbg!(&seed);
        for map in maps.iter() {
            dbg!(&map.len());
            let mut in_map = false;
            for range in map.iter() {
                dbg!(&range);
                if (*seed < (range[1] + range[2])) && (*seed >= (range[1])) && !in_map {
                    *seed = range[0] + *seed - range[1];
                    dbg!(&seed);
                    in_map = true;
                }
            }
        }
        dbg!(seed);
    }
    let out = seeds.iter().min().unwrap();
    out.to_string()
}

fn process_2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

";

    #[test]
    fn test_1() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "35".to_string())
    }
    #[test]
    fn test_2() {
        let output = part_2(EXAMPLE_TEXT);
        assert_eq!(output, "".to_string())
    }
}
