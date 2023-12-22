use crate::puzzle::Puzzle;
use std::collections::{HashMap, VecDeque};

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        let mut modules = self.parse_modules();
        let mut low_pulses = 0;
        let mut high_pulses = 0;
        let mut queue = VecDeque::new();
        for _ in 0..1000 {
            queue.push_back(Message {
                src: "button".to_string(),
                dst: "broadcaster".to_string(),
                pulse: Pulse::Low,
            });
            while let Some(message) = queue.pop_front() {
                match message.pulse {
                    Pulse::Low => low_pulses += 1,
                    Pulse::High => high_pulses += 1,
                }
                if let Some(module) = modules.get_mut(&message.dst) {
                    queue.extend(module.receive_message(message));
                }
            }
        }
        (low_pulses * high_pulses).to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut modules = self.parse_modules();

        let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
        for (src, module) in &modules {
            let dsts = match module {
                Module::FlipFlop { outputs, .. } => outputs,
                Module::Conjunction { outputs, .. } => outputs,
                Module::Broadcast { outputs, .. } => outputs,
            };
            for dst in dsts {
                inputs.entry(dst.clone()).or_default().push(src.clone());
            }
        }

        let rx_input = inputs.get("rx").unwrap().first().unwrap(); // This is the conjunction that feeds rx.
        let num_cycles = inputs.get(rx_input).unwrap().len(); // This is the number of cycles that feed the conjunction that feeds rx.
        let mut cycles = HashMap::new();
        let mut total_presses = 0u64;
        let mut queue = VecDeque::new();
        loop {
            queue.push_back(Message {
                src: "button".to_string(),
                dst: "broadcaster".to_string(),
                pulse: Pulse::Low,
            });
            total_presses += 1;
            while let Some(message) = queue.pop_front() {
                if message.dst == *rx_input && message.pulse == Pulse::High {
                    cycles.insert(message.src.clone(), total_presses);
                    if cycles.len() == num_cycles {
                        return cycles
                            .into_values()
                            .reduce(num::integer::lcm)
                            .unwrap()
                            .to_string();
                    }
                }

                if let Some(module) = modules.get_mut(&message.dst) {
                    queue.extend(module.receive_message(message));
                }
            }
        }
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }

    fn parse_modules(&self) -> HashMap<String, Module> {
        let mut input_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut modules: HashMap<String, Module> = HashMap::new();
        for line in self.input.lines() {
            let mut parts = line.split(" -> ");
            let full_name = parts.next().unwrap();
            let name = if full_name.starts_with('%') || full_name.starts_with('&') {
                full_name[1..].to_string()
            } else {
                full_name.to_string()
            };
            let outputs = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            for output in &outputs {
                input_map
                    .entry(output.clone())
                    .or_default()
                    .push(name.clone());
            }
            if full_name.starts_with('%') {
                modules.insert(
                    name.clone(),
                    Module::FlipFlop {
                        name,
                        enabled: false,
                        outputs,
                    },
                );
            } else if full_name.starts_with('&') {
                modules.insert(
                    name.clone(),
                    Module::Conjunction {
                        name,
                        memory: HashMap::new(),
                        outputs,
                    },
                );
            } else {
                modules.insert(name.clone(), Module::Broadcast { name, outputs });
            };
        }
        for (name, module) in modules.iter_mut() {
            if let Module::Conjunction { memory, .. } = module {
                for input in input_map.get(name).unwrap() {
                    memory.insert(input.clone(), Pulse::Low);
                }
            }
        }
        modules
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

struct Message {
    src: String,
    dst: String,
    pulse: Pulse,
}

enum Module {
    FlipFlop {
        name: String,
        enabled: bool,
        outputs: Vec<String>,
    },
    Conjunction {
        name: String,
        memory: HashMap<String, Pulse>,
        outputs: Vec<String>,
    },
    Broadcast {
        name: String,
        outputs: Vec<String>,
    },
}

impl Module {
    fn receive_message(&mut self, message: Message) -> Vec<Message> {
        match self {
            Module::FlipFlop {
                name,
                ref mut enabled,
                outputs,
            } => {
                if message.pulse == Pulse::High {
                    return Vec::new();
                }
                *enabled = !*enabled;
                let sent_pulse = if *enabled { Pulse::High } else { Pulse::Low };
                outputs
                    .iter()
                    .map(|output| Message {
                        src: name.clone(),
                        dst: output.clone(),
                        pulse: sent_pulse,
                    })
                    .collect()
            }
            Module::Conjunction {
                name,
                ref mut memory,
                outputs,
            } => {
                memory.insert(message.src, message.pulse);
                let sent_pulse = if memory.values().all(|pulse| *pulse == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                outputs
                    .iter()
                    .map(|output| Message {
                        src: name.clone(),
                        dst: output.clone(),
                        pulse: sent_pulse,
                    })
                    .collect()
            }
            Module::Broadcast { name, outputs } => {
                let sent_pulse = message.pulse;
                outputs
                    .iter()
                    .map(|output| Message {
                        src: name.clone(),
                        dst: output.clone(),
                        pulse: sent_pulse,
                    })
                    .collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "broadcaster -> a, b, c\n\
            %a -> b\n\
            %b -> c\n\
            %c -> inv\n\
            &inv -> a";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "32000000");
    }

    #[test]
    fn test_part_1_example_2() {
        let input = "broadcaster -> a\n\
            %a -> inv, con\n\
            &inv -> b\n\
            %b -> con\n\
            &con -> output";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "11687500");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/20")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "825167435");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/20")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "225514321828633");
    }
}
