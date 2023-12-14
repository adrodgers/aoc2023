use std::collections::HashMap;

use itertools::Itertools;

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
    let mut total_options = 0;
    for line in input.lines() {
        total_options += possible_options(line);
    }
    total_options.to_string()
}

fn process_2(input: &str) -> String {
    let mut total_options = 0;
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let arr = split.next().unwrap();
        let dots = std::iter::repeat(arr).take(5).join("?");
        let batches: Vec<usize> = split
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let blocks = std::iter::repeat(batches).take(5).flatten().collect_vec();
        let mut set: HashMap<(usize, usize, usize), usize> = HashMap::new();
        total_options += dynamic_programming(&dots, blocks, 0, 0, 0, &mut set);
    }
    total_options.to_string()
}

/// from https://github.com/jonathanpaulson/AdventOfCode/blob/master/2023/12.py
fn dynamic_programming(
    dots: &str,
    blocks: Vec<usize>,
    // index into dots
    i: usize,
    // index into blocks
    bi: usize,
    // current block length
    current: usize,
    //
    set: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    // check if we have already cached the solution
    if set.get(&(i, bi, current)).is_some() {
        return *set.get(&(i, bi, current)).unwrap();
    }
    // if we are at the end of the pattern
    if i == dots.len() {
        if (bi == blocks.len() && current == 0) || bi == (blocks.len() - 1) && blocks[bi] == current
        {
            return 1;
        }
        return 0;
    }
    let mut ans = 0;
    for c in ['#', '.'].into_iter() {
        let dots_vec = dots.chars().collect_vec();
        if dots_vec[i] == c || dots_vec[i] == '?' {
            if c == '.' && current == 0 {
                ans += dynamic_programming(dots, blocks.clone(), i + 1, bi, 0, set);
            } else if c == '.' && current > 0 && bi < blocks.len() && blocks[bi] == current {
                ans += dynamic_programming(dots, blocks.clone(), i + 1, bi + 1, 0, set);
            } else if c == '#' {
                ans += dynamic_programming(dots, blocks.clone(), i + 1, bi, current + 1, set);
            }
        }
    }
    set.insert((i, bi, current), ans);
    ans
}

fn possible_options(line: &str) -> u32 {
    let mut split = line.split_whitespace();
    let arr = split.next().unwrap();
    let batches: Vec<usize> = split
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let possible = ["#", "."];
    let unknown_indices: Vec<usize> = arr
        .char_indices()
        .filter(|(_i, c)| *c == '?')
        .map(|c| c.0)
        .collect();

    // Inspired by https://www.youtube.com/watch?v=FeHR-_mKd88
    let options: Vec<String> = itertools::repeat_n(possible.into_iter(), unknown_indices.len())
        .multi_cartesian_product()
        .map(|v| v.join(""))
        .collect();
    let mut valid_options = 0;
    for option in options {
        let mut option_iter = option.chars();
        let filled_option: String = line
            .chars()
            .map(|c| match c {
                '?' => option_iter.next().unwrap(),
                char => char,
            })
            .collect();
        let is_valid = valid_option(&filled_option, batches.clone());
        if is_valid {
            valid_options += 1;
        }
    }
    valid_options
}

fn valid_option(option: &str, batches: Vec<usize>) -> bool {
    let counts = option
        .chars()
        .group_by(|c| c == &'#')
        .into_iter()
        .filter_map(|(is_hashes, group)| is_hashes.then_some(group.into_iter().count()))
        .collect::<Vec<usize>>();
    counts == batches
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALID_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_1() {
        let output = part_1(VALID_INPUT);
        assert_eq!(output, "21".to_string())
    }
    #[test]
    fn test_2() {
        let output = part_2(VALID_INPUT);
        assert_eq!(output, "525152".to_string())
    }
}
