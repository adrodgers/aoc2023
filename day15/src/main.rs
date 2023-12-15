use std::collections::BTreeMap;

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
    let mut running_sum = 0;
    let str_vec: Vec<&str> = input.split(',').map(|x| x.trim()).collect();
    for str in str_vec.into_iter() {
        let mut current_value = 0;
        for c in str.chars() {
            current_value += c as u8 as u64;
            current_value *= 17;
            current_value %= 256;
        }
        running_sum += current_value;
    }
    running_sum.to_string()
}

fn process_2(input: &str) -> String {
    let lenses = Lens::parse_all(input);
    let mut boxes: BTreeMap<u8, BTreeMap<String, (usize, u8)>> = BTreeMap::new();
    for (idx, lens) in lenses.into_iter().enumerate() {
        // dbg!(&lens);
        let box_num = lens.to_box();
        // dbg!(&box_num);
        let b = boxes.entry(box_num).or_default();
        if lens.operation == '-' {
            b.remove(&lens.label);
        }
        if lens.operation == '=' {
            if b.contains_key(&lens.label) {
                b.get_mut(&lens.label).unwrap().1 = lens.focal_length.unwrap();
            } else {
                b.insert(lens.label, (idx, lens.focal_length.unwrap()));
            }
        }
    }
    let mut total = 0;
    for b in boxes {
        let b_num = b.0 as u64 + 1;
        let mut lenses: Vec<(usize, u8)> = b.1.into_values().collect();
        lenses.sort();
        for (i, lens) in lenses.into_iter().enumerate() {
            total += b_num * ((i + 1) as u64) * lens.1 as u64;
        }
    }
    total.to_string()
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: Option<u8>,
    operation: char,
}

impl Lens {
    fn parse_all(input: &str) -> Vec<Lens> {
        let lens_str_vec: Vec<&str> = input.trim_matches('\n').split(',').collect();
        let mut lenses: Vec<Lens> = Vec::new();
        for lens_str in lens_str_vec {
            for c in ['=', '-'] {
                let split = lens_str.split_once(c);
                if split.is_some() {
                    let mut focal_length = 0;
                    if !split.unwrap().1.is_empty() {
                        focal_length = split.unwrap().1.parse().unwrap();
                    }
                    lenses.push(Lens {
                        label: split.unwrap().0.to_string(),
                        focal_length: Some(focal_length),
                        operation: c,
                    })
                }
            }
        }
        lenses
    }

    fn to_box(&self) -> u8 {
        let mut current_value = 0;
        for c in self.label.chars() {
            current_value += c as u8 as u64;
            current_value *= 17;
            current_value %= 256;
        }
        current_value as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_1() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "1320".to_string())
    }
    #[test]
    fn test_2() {
        let output = part_2(EXAMPLE_TEXT);
        assert_eq!(output, "145".to_string())
    }
}
