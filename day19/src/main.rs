use core::panic;
use std::collections::{HashMap, VecDeque};

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
    process_2(input)
}

fn process_1(input: &str) -> String {
    let mut split = input.split("\n\n");
    let mut instruction_map_str = split.next().unwrap();
    let mut instructions: HashMap<&str, Instruction> = HashMap::new();
    for instruction_str in instruction_map_str.lines() {
        let mut split = instruction_str.split('{');
        let key = split.next().unwrap();

        instructions.insert(key, Instruction::parse(split.last().unwrap()));
    }
    // dbg!(&instructions);
    let parts_str = split.last().unwrap();
    let mut parts: Vec<Part> = Part::parse_all(parts_str);

    for part in parts.iter_mut() {
        let start = instructions.get("in").unwrap();
        let mut next = start.check_part(part);
        if next == "R" {
            part.rejected = true;
        }
        if next == "A" {
            part.accepted = true;
        }

        while !part.accepted || !part.rejected {
            let instruction = instructions.get(next).unwrap();

            next = instruction.check_part(part);
            dbg!(&next);
            if next == "R" {
                part.rejected = true;
                break;
            }
            if next == "A" {
                part.accepted = true;
                break;
            }
        }
    }
    parts
        .into_iter()
        .filter(|p| p.accepted)
        .map(|p| p.x + p.m + p.a + p.s)
        .sum::<i32>()
        .to_string()
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
    accepted: bool,
    rejected: bool,
}

impl Part {
    fn parse_all(input: &str) -> Vec<Part> {
        let mut parts: Vec<Part> = Vec::new();
        for line in input.lines() {
            let mut part = Part {
                x: 0,
                m: 0,
                a: 0,
                s: 0,
                accepted: false,
                rejected: false,
            };
            let split = line
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',');
            for str in split {
                let mut s = str.split('=');
                let c = s.next().unwrap().chars().next().unwrap();
                let v = s.last().unwrap().parse::<i32>().unwrap();
                match c {
                    'x' => part.x = v,
                    'm' => part.m = v,
                    'a' => part.a = v,
                    's' => part.s = v,
                    _ => panic!("invalid part field"),
                }
            }

            parts.push(part);
        }
        parts
    }
}

#[derive(Debug)]
struct Instruction<'a> {
    conditions: Vec<Condition<'a>>,
    otherwise: &'a str,
}

impl Instruction<'_> {
    fn parse(input: &str) -> Instruction {
        let condition_strs = input.trim_end_matches('}').split(',');
        let conditions_len = condition_strs.clone().count();
        let otherwise = condition_strs.clone().last().unwrap();
        let mut conditions: Vec<Condition> = Vec::new();
        for condition in condition_strs.into_iter().take(conditions_len - 1) {
            conditions.push(Condition::parse(condition));
        }
        Instruction {
            conditions,
            otherwise,
        }
    }

    fn check_part(&self, part: &Part) -> &str {
        for condition in &self.conditions {
            // dbg!(&condition);
            // dbg!(&part);
            let pass = match (condition.c, condition.op) {
                ('x', '<') => part.x < condition.val,
                ('x', '>') => part.x > condition.val,
                ('m', '<') => part.m < condition.val,
                ('m', '>') => part.m > condition.val,
                ('a', '<') => part.a < condition.val,
                ('a', '>') => part.a > condition.val,
                ('s', '<') => part.s < condition.val,
                ('s', '>') => part.s > condition.val,
                (_, _) => panic!("invalid condition"),
            };
            // dbg!(&pass);
            if pass {
                return condition.to;
            }
        }
        self.otherwise
    }
}

#[derive(Debug)]
struct Condition<'a> {
    c: char,
    op: char,
    val: i32,
    to: &'a str,
}

impl Condition<'_> {
    fn parse(input: &str) -> Condition {
        // dbg!(&input);
        let mut chars = input.split(':').next().unwrap().chars();
        let to = input.split(':').last().unwrap();
        let c = chars.next().unwrap();
        let op = chars.next().unwrap();
        // dbg!(&chars);
        let val = chars.collect::<String>().parse::<i32>().unwrap();
        let to = input.split(':').last().unwrap();
        Condition { c, op, val, to }
    }
}

fn process_2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_1() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "19114".to_string())
    }
    // #[test]
    // fn test_2() {
    //     let output = part_2(EXAMPLE_TEXT);
    //     assert_eq!(output, "".to_string())
    // }
}
