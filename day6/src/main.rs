use indicatif::ProgressBar;
use std::iter::zip;

fn main() {
    let input1 = "Time:        40     82     91     66
Distance:   277   1338   1349   1063";
    let output1 = part_1(input1);
    println!("{output1}");
    let output2 = part_2(input1);
    println!("{output2}");
}

fn part_1(input: &str) -> String {
    process_1(input)
}
fn part_2(input: &str) -> String {
    process_2(input)
}

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn possible_distances_above_record(&self) -> u64 {
        let mut num_above_record = 0;
        let bar = ProgressBar::new(self.time);
        for acceleration in 0..=self.time {
            bar.inc(1);
            if (self.time - acceleration) * acceleration > self.record_distance {
                num_above_record += 1;
            }
        }
        bar.finish();
        num_above_record
    }
}

fn process_1(input: &str) -> String {
    let mut lines = input.lines();
    let time: Vec<u64> = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();
    dbg!(&time);
    let record_distance: Vec<u64> = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|d| d.parse().unwrap())
        .collect();
    dbg!(&record_distance);
    let mut races: Vec<Race> = Vec::new();
    for (t, d) in zip(time, record_distance) {
        races.push(Race {
            time: t,
            record_distance: d,
        })
    }
    dbg!(&races);
    let out: u64 = races
        .iter()
        .map(|r| r.possible_distances_above_record())
        .product();
    out.to_string()
}

fn process_2(input: &str) -> String {
    let mut lines = input.lines();
    let time: Vec<&str> = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect();
    let t = time.join("");
    dbg!(&time);
    let record_distance: Vec<&str> = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect();
    let d = record_distance.join("");
    dbg!(&record_distance);
    let race = Race {
        time: t.parse().unwrap(),
        record_distance: d.parse().unwrap(),
    };
    let out = race.possible_distances_above_record();
    out.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_1() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "288".to_string())
    }
    #[test]
    fn test_2() {
        let output = part_2(EXAMPLE_TEXT);
        assert_eq!(output, "71503".to_string())
    }
}
