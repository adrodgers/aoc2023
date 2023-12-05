use indicatif::ProgressIterator;

fn main() {
    let input1 = include_str!("./input1.txt");
    part_1(input1);
    let input2 = include_str!("./input1.txt");
    part_2(input2);
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
    let mut lines = input.lines();
    let mut seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut maps: Vec<Vec<Vec<u64>>> = Vec::new();
    let mut map: Vec<Vec<u64>> = Vec::new();
    let num_lines = lines.clone().count();
    for (i, line) in lines.enumerate() {
        if !line.contains(':') && !line.is_empty() {
            let line_data: Vec<u64> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            map.push(line_data);
        }
        if (line.is_empty() || i == num_lines) && !map.is_empty() {
            maps.push(map.clone());
            map.clear();
        }
    }
    for seed in seeds.iter_mut().progress() {
        for map in maps.iter() {
            let mut in_map = false;
            for range in map.iter() {
                if (*seed < (range[1] + range[2])) && (*seed >= (range[1])) && !in_map {
                    *seed = range[0] + *seed - range[1];
                    in_map = true;
                }
            }
        }
    }
    let out = seeds.iter().min().unwrap();
    out.to_string()
}

fn process_2(input: &str) -> String {
    let mut lines = input.lines();
    let seed_line: Vec<u64> = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let seed_ranges: Vec<[u64; 2]> = seed_line
        .chunks(2)
        .map(|chunk| -> [u64; 2] {
            let mut array: [u64; 2] = [0; 2];
            for (i, val) in chunk.iter().enumerate() {
                array[i] = *val;
            }
            array
        })
        .collect();
    let mut seeds: Vec<u64> = Vec::new();
    for seed_range in seed_ranges {
        for i in 0..seed_range[1] - 1 {
            seeds.push(seed_range[0] + i);
        }
    }
    let mut maps: Vec<Vec<Vec<u64>>> = Vec::new();
    let mut map: Vec<Vec<u64>> = Vec::new();
    let num_lines = lines.clone().count();
    for (i, line) in lines.enumerate() {
        if !line.contains(':') && !line.is_empty() {
            let line_data: Vec<u64> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            map.push(line_data);
        }
        if (line.is_empty() || i == num_lines) && !map.is_empty() {
            maps.push(map.clone());
            map.clear();
        }
    }

    for seed in seeds.iter_mut().progress() {
        for map in maps.iter() {
            let mut in_map = false;
            for range in map.iter() {
                if (*seed < (range[1] + range[2])) && (*seed >= (range[1])) && !in_map {
                    *seed = range[0] + *seed - range[1];
                    in_map = true;
                }
            }
        }
    }
    let out = seeds.iter().min().unwrap();
    out.to_string()
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
        assert_eq!(output, "46".to_string())
    }
}
