fn main() {
    let input1 = include_str!("./input1.txt");
    part_1(input1);
    // let input2 = include_str!("./input2.txt");
    // part_2(input2);
}

fn part_1(input: &str) -> String {
    parse_line(input);
    "".to_string()
}
fn part_2(input: &str) -> String {
    "".to_string()
}

#[derive(Debug)]
struct SymbolLoc {
    symbol: char,
    i: usize,
    j: usize,
}

struct NumLoc {
    num: u32,
    i: usize,
    j: usize,
}

fn parse_line(input: &str) -> () {
    let mut symbol_locs: Vec<SymbolLoc> = Vec::new();
    let mut num_locs: Vec<NumLoc> = Vec::new();
    // input
    //     .lines()
    //     .enumerate()
    //     .map(|(j, line)| line.chars().enumerate().filter(|(i, c)| *c != '.'));
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_punctuation() {
                symbol_locs.push(SymbolLoc {
                    symbol: c,
                    i: i,
                    j: j,
                })
            }
            if c.is_numeric() {}
        }
    }
    dbg!(symbol_locs);
}

fn parse_line_2(line: &str) -> () {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let output = part_1(test_input);
        assert_eq!(output, "".to_string())
    }
    #[test]
    fn test_2() {
        let test_input = "
";
        let output = part_2(test_input);
        assert_eq!(output, "".to_string())
    }
}
