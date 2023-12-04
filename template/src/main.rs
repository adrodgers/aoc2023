const EXAMPLE_TEXT: &str = "";

fn main() {
    let input1 = include_str!("./input1.txt");
    part_1(input1);
    // let input2 = include_str!("./input2.txt");
    // part_2(input2);
}

fn part_1(input: &str) -> String {
    let output = process_2(input);
    output
}
fn part_2(input: &str) -> String {
    let output = process_1(input);
    output
}

fn process_1(input: &str) -> String {
    "".to_string()
}

fn process_2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "".to_string())
    }
    #[test]
    fn test_2() {
        let output = part_2(EXAMPLE_TEXT);
        assert_eq!(output, "".to_string())
    }
}
