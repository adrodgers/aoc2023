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
    // Part two requires a dynamic programming approach.
    // e.g. https://gist.github.com/icub3d/7aa45ca96ccb88ebf95b91d6a28eba74
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
    "".to_string()
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
        .filter(|(i, c)| *c == '?')
        .map(|c| c.0)
        .collect();
    // dbg!(&unknown_indices);

    // Inspired by https://www.youtube.com/watch?v=FeHR-_mKd88
    let options: Vec<String> = itertools::repeat_n(possible.into_iter(), unknown_indices.len())
        .multi_cartesian_product()
        .map(|v| v.join(""))
        .collect();
    // dbg!(&options);
    let mut valid_options = 0;
    for option in options {
        let mut option_iter = option.chars();
        let filled_option: String = line
            .chars()
            .map(|c| match c {
                // Part two requires a dynamic programming approach.
                // e.g. https://gist.github.com/icub3d/7aa45ca96ccb88ebf95b91d6a28eba74
                '?' => option_iter.next().unwrap(),
                char => char,
            })
            .collect();
        //     dbg!(&filled_option);
        let is_valid = valid_option(&filled_option, batches.clone());
        if is_valid {
            valid_options += 1;
        }
    }
    valid_options
    // Part two requires a dynamic programming approach.
    // e.g. https://gist.github.com/icub3d/7aa45ca96ccb88ebf95b91d6a28eba74
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
}
