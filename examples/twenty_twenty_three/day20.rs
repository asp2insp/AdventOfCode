#[derive(Clone, Copy)]
enum Signal {
    High,
    Low,
}

impl Signal {
    fn is_high(&self) -> bool {
        match self {
            High => true,
            Low => false,
        }
    }
}

use std::collections::HashMap;

use itertools::Itertools;
use Signal::*;

struct FlipFlop {
    is_on: bool,
}

impl FlipFlop {
    fn new() -> FlipFlop {
        FlipFlop { is_on: false }
    }

    fn recv(&mut self, _sender: String, signal: Signal) -> Option<Signal> {
        match signal {
            High => None,
            Low => {
                if self.is_on {
                    self.is_on = false;
                    Some(Low)
                } else {
                    self.is_on = true;
                    Some(High)
                }
            }
        }
    }
}

struct Conjunction {
    inputs: HashMap<String, Signal>,
}

impl Conjunction {
    fn new() -> Conjunction {
        Conjunction {
            inputs: HashMap::new(),
        }
    }

    fn recv(&mut self, sender: String, signal: Signal) -> Option<Signal> {
        self.inputs.insert(sender, signal);
        if self.inputs.values().all(Signal::is_high) {
            Some(Low)
        } else {
            Some(High)
        }
    }

    fn add_input(&mut self, name: String) {
        self.inputs.insert(name, Low);
    }
}

enum Component {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast,
    Rx,
}

impl Component {
    fn recv(&mut self, sender: String, signal: Signal) -> Option<Signal> {
        match self {
            Component::FlipFlop(ff) => ff.recv(sender, signal),
            Component::Conjunction(c) => c.recv(sender, signal),
            Component::Broadcast => match signal {
                High => Some(High),
                Low => Some(Low),
            },
            Component::Rx => None,
        }
    }

    fn add_input(&mut self, name: String) {
        match self {
            Component::FlipFlop(_) => {}
            Component::Conjunction(c) => c.add_input(name),
            Component::Broadcast => {}
            Component::Rx => {}
        }
    }

    fn inputs(&self) -> Vec<String> {
        match self {
            Component::FlipFlop(_) => vec![],
            Component::Conjunction(c) => c.inputs.keys().cloned().collect(),
            Component::Broadcast => vec![],
            Component::Rx => vec![],
        }
    }
}

fn parse(input: &str) -> HashMap<String, (Component, Vec<String>)> {
    let mut components: HashMap<String, (Component, Vec<String>)> = HashMap::new();
    let mut reverse_map = vec![];
    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let left = parts.next().unwrap().trim();
        let right = parts.next().unwrap().trim();
        let (component, name) = match left.chars().next().unwrap() {
            '%' => {
                let ff = FlipFlop::new();
                (Component::FlipFlop(ff), left[1..].to_string())
            }
            '&' => {
                let c = Conjunction::new();
                (Component::Conjunction(c), left[1..].to_string())
            }
            'b' => (Component::Broadcast, left.to_string()),
            _ => panic!("Unknown component"),
        };
        let outputs: Vec<String> = right.split(',').map(|s| s.trim().to_string()).collect();
        reverse_map.push((name.clone(), outputs.clone()));
        components.insert(name, (component, outputs));
    }
    components.insert("rx".to_string(), (Component::Rx, vec![]));
    for (name, outputs) in reverse_map {
        for output in outputs {
            if let Some(c) = components.get_mut(&output) {
                c.0.add_input(name.clone());
            }
        }
    }
    components
}

pub fn part1(input: String) -> String {
    let mut components = parse(&input);
    let mut counts = (0, 0);
    for _ in 0..1000 {
        let mut s = vec![("button".to_string(), "broadcaster".to_string(), Low)];
        while !s.is_empty() {
            s = s
                .into_iter()
                .flat_map(|(sender, name, signal)| {
                    match signal {
                        High => counts.1 += 1,
                        Low => counts.0 += 1,
                    };
                    let (component, outputs) = components.get_mut(&name).expect(&name);
                    if let Some(out) = component.recv(sender, signal) {
                        outputs
                            .iter()
                            .map(|o| (name.clone(), o.clone(), out))
                            .collect()
                    } else {
                        vec![]
                    }
                })
                .collect();
        }
    }
    (counts.0 * counts.1).to_string()
}

pub fn part2(input: String) -> String {
    let mut components = parse(&input);
    let mut i = 0;
    let mut is_done = false;
    let mut targets = components
        .get("zh")
        .unwrap()
        .0
        .inputs()
        .iter()
        .map(|s| (s.clone(), 0))
        .collect::<HashMap<_, _>>();
    while !is_done {
        i += 1;
        let mut s = vec![("button".to_string(), "broadcaster".to_string(), Low)];
        while !s.is_empty() {
            s = s
                .into_iter()
                .flat_map(|(sender, name, signal)| {
                    let (component, outputs) = components.get_mut(&name).expect(&name);
                    if let Some(out) = component.recv(sender, signal) {
                        outputs
                            .iter()
                            .map(|o| (name.clone(), o.clone(), out))
                            .collect()
                    } else {
                        vec![]
                    }
                })
                .collect();
            for t in s
                .iter()
                .filter_map(|(_, n, s)| {
                    if !s.is_high() && targets.contains_key(n) {
                        Some(n)
                    } else {
                        None
                    }
                })
                .collect_vec()
            {
                if let Some(0) = targets.get(t).cloned() {
                    targets.insert(t.clone(), i);
                }
            }
            is_done = targets.values().all(|v| *v > 0);
        }
    }
    targets.into_values().product::<usize>().to_string()
}
