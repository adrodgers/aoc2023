fn main() {
    let input1 = include_str!("./input1.txt");
    let output1 = part_1(input1);
    println!("{output1}");
    let input2 = include_str!("./input1.txt");
    let output2 = part_2(input2);
    println!("{output2}");
}

fn part_1(input: &str) -> String {
    parse_line(input)
}
fn part_2(input: &str) -> String {
    parse_line_2(input)
}

#[derive(Debug)]
struct SymbolLoc {
    symbol: char,
    i: usize,
    j: usize,
}

#[derive(Debug)]
struct NumLoc {
    num: u32,
    len: usize,
    i_start: usize,
    j: usize,
}

fn parse_line(input: &str) -> String {
    let mut symbol_locs: Vec<SymbolLoc> = Vec::new();
    let mut num_locs: Vec<NumLoc> = Vec::new();
    let mut num_str = String::new();
    for (j, line) in input.lines().enumerate() {
        let line_append = format!("{}{}", line, ".");
        for (i, c) in line_append.trim().chars().enumerate() {
            if c.is_ascii_digit() {
                num_str.push(c)
            }
            if (c.is_ascii_punctuation()) && !num_str.is_empty() {
                num_locs.push(NumLoc {
                    num: num_str.parse().unwrap(),
                    len: num_str.len(),
                    i_start: i - num_str.len(),
                    j,
                })
            }
            if !c.is_ascii_digit() {
                num_str.clear();
            }
            if c.is_ascii_punctuation() && c != '.' {
                symbol_locs.push(SymbolLoc { symbol: c, i, j })
            }
        }
    }
    let mut sum = 0;
    // for symbol in symbol_locs.iter().skip(2).take(1) {
    for symbol in symbol_locs.iter() {
        for num in num_locs.iter() {
            // within 1 line
            if num.j.abs_diff(symbol.j) < 2 {
                let mut num_added = false;
                // within the length of the number
                for l in 0..num.len {
                    if (num.i_start + l).abs_diff(symbol.i) < 2 && !num_added {
                        sum += num.num;
                        num_added = true;
                    }
                }
            }
        }
    }
    sum.to_string()
}

fn parse_line_2(input: &str) -> String {
    let mut symbol_locs: Vec<SymbolLoc> = Vec::new();
    let mut num_locs: Vec<NumLoc> = Vec::new();
    let mut num_str = String::new();
    for (j, line) in input.lines().enumerate() {
        let line_append = format!("{}{}", line, ".");
        for (i, c) in line_append.trim().chars().enumerate() {
            if c.is_ascii_digit() {
                num_str.push(c)
            }
            if (c.is_ascii_punctuation()) && !num_str.is_empty() {
                num_locs.push(NumLoc {
                    num: num_str.parse().unwrap(),
                    len: num_str.len(),
                    i_start: i - num_str.len(),
                    j,
                })
            }
            if !c.is_ascii_digit() {
                num_str.clear();
            }
            if c.is_ascii_punctuation() && c != '.' {
                symbol_locs.push(SymbolLoc { symbol: c, i, j })
            }
        }
    }
    let mut sum = 0;
    let mut gear_ratio_sum = 0;
    for symbol in symbol_locs.iter() {
        let mut num_adjacent: Vec<u32> = Vec::new();
        for num in num_locs.iter() {
            // within 1 line
            if num.j.abs_diff(symbol.j) < 2 {
                let mut num_added = false;
                // within the length of the number
                for l in 0..num.len {
                    if (num.i_start + l).abs_diff(symbol.i) < 2 && !num_added {
                        sum += num.num;
                        num_adjacent.push(num.num);
                        num_added = true;
                    }
                }
            }
        }
        if num_adjacent.len() == 2 {
            gear_ratio_sum += num_adjacent.iter().product::<u32>();
        }
    }
    gear_ratio_sum.to_string()
}

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
        assert_eq!(output, "4361".to_string())
    }
    #[test]
    fn test_2() {
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
        let output = part_2(test_input);
        assert_eq!(output, "467835".to_string())
    }
}
