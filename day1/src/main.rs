fn main() {
    let input1= include_str!("./input1.txt");
    let input2 = include_str!("./input2.txt");
    part_1(input1);
    part_2(input2);
}

fn part_1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let num = parse_line(line);
        sum += num;
    }
    println!("{}",&sum);
    sum.to_string()
}

fn part_2(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let num = parse_line_2(line);
        sum += num
    }
    println!("{}",&sum);
    sum.to_string()
}

fn parse_line (line: &str) -> i32 {
    let mut digits = String::new();
    for c in line.chars() {
        if c.is_numeric() {
            digits.push(c)
        }
    }
    let first = digits.chars().next().unwrap();
    let last = digits.chars().next_back().unwrap();
    let mut num = String::new();
    num.push(first);
    num.push(last);
    num.parse().unwrap()
}

fn parse_line_2 (line: &str) -> i32 {
    let digit_array = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut digits = String::new();
    for (i,c) in line.chars().enumerate() {
        if c.is_numeric() {
            digits.push(c)
        }
        let s = &line[i..];
        for (idx, str) in digit_array.iter().enumerate() {
            if s.starts_with(str) {
                digits.push_str(&idx.to_string())
            }
        }
    }
    let first = digits.chars().next().unwrap();
    let last = digits.chars().next_back().unwrap();
    let mut num = String::new();
    num.push(first);
    num.push(last);
    num.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let output = part_1(test_input);
        assert_eq!(output,"142".to_string())
    }
    #[test]
    fn test_2() {
        let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let output = part_2(test_input);
        assert_eq!(output,"281".to_string())
    }
}
