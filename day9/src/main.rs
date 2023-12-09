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
    let mut predictions: Vec<i32> = Vec::new();
    for line in input.lines() {
        let readings: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let mut differences: Vec<Vec<i32>> = vec![];
        let mut diff: Vec<i32> = readings.windows(2).map(|d| d[1] - d[0]).collect();
        differences.push(diff.clone());
        while !diff.clone().into_iter().all(|d| d == 0) {
            diff = differences
                .last()
                .unwrap()
                .windows(2)
                .map(|d| d[1] - d[0])
                .collect();
            differences.push(diff.clone());
        }
        let mut num = 0;
        for difference in differences.into_iter().rev() {
            num += difference.into_iter().last().unwrap();
        }
        predictions.push(readings.into_iter().last().unwrap() + num);
    }
    predictions.into_iter().sum::<i32>().to_string()
}

fn process_2(input: &str) -> String {
    let mut predictions: Vec<i32> = Vec::new();
    for line in input.lines() {
        let readings: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let mut differences: Vec<Vec<i32>> = vec![];
        let mut diff: Vec<i32> = readings.windows(2).map(|d| d[1] - d[0]).collect();
        differences.push(diff.clone());
        while !diff.clone().into_iter().all(|d| d == 0) {
            diff = differences
                .last()
                .unwrap()
                .windows(2)
                .map(|d| d[1] - d[0])
                .collect();
            differences.push(diff.clone());
        }
        let mut num = 0;
        for difference in differences.into_iter().rev() {
            num = difference.into_iter().next().unwrap() - num;
        }
        predictions.push(readings.into_iter().next().unwrap() - num);
    }
    predictions.into_iter().sum::<i32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    // #[test]
    // fn test_1() {
    //     let output = part_1(EXAMPLE_TEXT);
    //     assert_eq!(output, "114".to_string())
    // }
    #[test]
    fn test_2() {
        let output = part_2(EXAMPLE_TEXT);
        assert_eq!(output, "2".to_string())
    }
}
