use std::collections::HashMap;

fn main() {
    let input1 = include_str!("./input1.txt");
    part_1(input1);
}

fn part_1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let val = parse_line(line);
        sum += val;
    }
    println!("{sum}");
    sum.to_string()
}

fn part_2(input: &str) -> String {
    todo!()
}

fn parse_line(line: &str) -> u32 {
    let game_map: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let split: Vec<&str> = line.split(':').collect();
    let mut id = split[0]
        .trim()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let games: Vec<&str> = split[1].trim().split(';').map(|s| s.trim()).collect();
    for game in games {
        let balls: Vec<&str> = game.split(',').map(|s| s.trim()).collect();
        for ball in balls {
            let val: Vec<&str> = ball.split_whitespace().collect();
            let num = val[0].parse::<u32>().unwrap();
            let colour = val[1];
            let allowed = game_map.get(colour).unwrap();
            if allowed < &num {
                id = 0;
            }
        }
    }
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let output = part_1(test_input);
        assert_eq!(output, "8".to_string())
    }
}
