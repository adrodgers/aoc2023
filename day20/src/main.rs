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

#[derive(Debug, Clone)]
struct Signal {
    pulse: Pulse,
    origin: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum ModuleType {
    FlipFlop(bool),
    Broadcaster,
    Conjunction(HashMap<String, Pulse>),
}

impl ModuleType {
    fn from_char(input: char) -> ModuleType {
        // dbg!(&input);
        match input {
            '%' => Self::FlipFlop(false),
            '&' => Self::Conjunction(HashMap::new()),
            'b' => Self::Broadcaster,
            _ => panic!("Invalid module type"),
        }
    }
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    module_type: ModuleType,
    destinations: Vec<String>,
}

impl Module {
    fn process_signal(&mut self, signal: Signal) -> VecDeque<(String, Signal)> {
        let mut signals: VecDeque<(String, Signal)> = VecDeque::new();
        match &mut self.module_type {
            ModuleType::FlipFlop(status) => match (signal.clone().pulse, status) {
                (Pulse::High, _) => {}
                (Pulse::Low, status) => match status {
                    true => {
                        *status = false;
                        self.destinations.iter().for_each(|d| {
                            signals.push_back((
                                d.clone(),
                                Signal {
                                    pulse: Pulse::Low,
                                    origin: self.name.clone(),
                                },
                            ))
                        })
                    }
                    false => {
                        *status = true;
                        self.destinations.iter().for_each(|d| {
                            signals.push_back((
                                d.clone(),
                                Signal {
                                    pulse: Pulse::High,
                                    origin: self.name.clone(),
                                },
                            ))
                        })
                    }
                },
            },
            ModuleType::Broadcaster => self.destinations.iter().for_each(|d| {
                signals.push_back((
                    d.clone(),
                    Signal {
                        pulse: signal.pulse.clone(),
                        origin: "broadcaster".to_string(),
                    },
                ))
            }),
            ModuleType::Conjunction(memory) => {
                memory.insert(signal.origin.clone(), signal.pulse.clone());
                if memory.iter_mut().all(|m| *m.1 == Pulse::High) {
                    self.destinations.iter().for_each(|d| {
                        signals.push_back((
                            d.clone(),
                            Signal {
                                pulse: Pulse::Low,
                                origin: self.name.clone(),
                            },
                        ))
                    })
                } else {
                    self.destinations.iter().for_each(|d| {
                        signals.push_back((
                            d.clone(),
                            Signal {
                                pulse: Pulse::High,
                                origin: self.name.clone(),
                            },
                        ))
                    })
                }
            }
        };
        signals
    }
    fn parse_all(input: &str) -> HashMap<String, Module> {
        input
            .lines()
            .map(|l| {
                let mut split = l.split(" -> ");
                let module = split.next().unwrap();
                let dests = split.next().unwrap();
                let mut name = module.chars().skip(1).collect::<String>();
                if module.contains("broadcaster") {
                    name = "broadcaster".to_string();
                }
                let module_type = ModuleType::from_char(module.chars().next().unwrap());
                let destinations = dests
                    .split(", ")
                    .map(|d| d.to_string())
                    .collect::<Vec<String>>();
                (
                    name.clone(),
                    Module {
                        name,
                        module_type,
                        destinations,
                    },
                )
            })
            .collect()
    }
}

fn process_1(input: &str) -> String {
    let mut modules = Module::parse_all(input);
    // dbg!(&modules);
    // Initialize memory
    let mut conjunction_modules: HashMap<String, Vec<String>> = modules
        .clone()
        .into_iter()
        .filter(|(_, m)| match m.module_type {
            ModuleType::Conjunction(_) => true,
            _ => false,
        })
        .map(|m| (m.1.name, Vec::new()))
        .collect();
    for module in modules.iter() {
        for dest in module.1.destinations.iter() {
            if conjunction_modules.contains_key(dest) {
                let m = conjunction_modules.get_mut(dest).unwrap();
                m.push(module.0.clone());
            }
        }
    }
    // dbg!(&conjunction_modules);
    for conjunction_module in conjunction_modules {
        for s in conjunction_module.1 {
            let m = modules.get_mut(&conjunction_module.0).unwrap();
            match m.module_type {
                ModuleType::Conjunction(ref mut memory) => {
                    memory.insert(s, Pulse::Low);
                }
                _ => panic!("Not conjunction module"),
            }
        }
    }
    // dbg!(&modules);
    let mut num_high_signals = 0;
    let mut num_low_signals = 0;
    for _ in 0..1000 {
        let mut signal_queue: VecDeque<(String, Signal)> = VecDeque::from([(
            String::from("broadcaster"),
            Signal {
                pulse: Pulse::Low,
                origin: "button".to_string(),
            },
        )]);
        while !signal_queue.is_empty() {
            // conjunction check
            // for cmod in conjunction_modules {

            // }
            let next = signal_queue.pop_front().unwrap();
            match next.1.pulse {
                Pulse::Low => num_low_signals += 1,
                Pulse::High => num_high_signals += 1,
            }
            // dbg!(&next);
            let module = modules.get_mut(&next.0);
            if module.is_some() {
                let mut new_signals = module.unwrap().process_signal(next.1);
                // dbg!(&new_signals);
                signal_queue.append(&mut new_signals);
            }
        }
    }
    dbg!(num_low_signals);
    dbg!(num_high_signals);

    // dbg!(&modules);
    (num_low_signals * num_high_signals).to_string()
}

fn process_2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_TEXT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const EXAMPLE_TEXT_2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_1() {
        let output = part_1(EXAMPLE_TEXT);
        assert_eq!(output, "32000000".to_string())
    }
    #[test]
    fn test_2() {
        let output = part_1(EXAMPLE_TEXT_2);
        assert_eq!(output, "11687500".to_string())
    }
}
