fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
}

fn part_1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        println!("{line}");
        let mut first_digit: Option<char> = None;
        let mut current_digit: Option<char> = None;
        for c in line.trim_start().chars() {
            if c.is_numeric() {
                if first_digit.is_none() {
                    first_digit = Some(c);
                }
                current_digit = Some(c);
            }
        }
        let mut num = String::new();
        num.push(first_digit.unwrap()); 
        num.push(current_digit.unwrap()); 
        sum += num.parse::<i32>().unwrap();
    }
    println!("{}",&sum);
    sum.to_string()
}

fn part_2(input: &str) -> String {
    "output".to_string()
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
}
