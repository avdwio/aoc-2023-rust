use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    fmt::{Display, Formatter},
};

type RelaySet = HashMap<&'static str, Relay>;

pub fn process(input: &'static str) -> u64 {
    let mut map = input.lines().map(parse_relay).collect::<HashMap<_, _>>();

    prime_relay_conjunctions(&mut map);
    let mut pulse_count = PulseCounter::new();

    let mut counter = 1;
    for _ in 0..5000 {
        button_click(&mut map, &mut pulse_count, counter);
        // println!();
        counter += 1;
    }

    println!("high: {} low {}", pulse_count.high, pulse_count.low);

    pulse_count.product()
}

fn button_click(map: &mut RelaySet, pulse_count: &mut PulseCounter, global_counter: u64) {
    let mut pulses = VecDeque::<Pulse>::new();

    pulses.push_back(Pulse {
        from: "button",
        to: "broadcaster",
        r#type: PulseType::Low,
    });

    let mut counter = 1;

    while let Some(pulse) = pulses.pop_front() {
        for mod_name in vec!["ql", "hl", "hq", "bc"] {
            let Module::Conjunction(ref map2) = map.get(mod_name).unwrap().module else {
                panic!();
            };

            if map2.iter().all(|(_, v)| *v) {
                println!(
                    "mod: {} :: iter: {:>10} :: {}",
                    mod_name, global_counter, counter
                );
            }
        }
        counter += 1;

        // println!("{} -{}-> {}", pulse.from, pulse.r#type, pulse.to);
        pulse_count.increment(&pulse.r#type);

        let Some(relay) = map.get_mut(pulse.to) else {
            continue;
        };

        // fire the relay module, and if it gets a result, fan out

        if let Some(pulse_type) = relay.module.fire(&pulse) {
            for to in relay.output.iter() {
                pulses.push_back(Pulse {
                    from: pulse.to,
                    to,
                    r#type: pulse_type,
                });
            }
        }
    }
}

struct PulseCounter {
    high: u64,
    low: u64,
}

impl PulseCounter {
    fn new() -> Self {
        Self { high: 0, low: 0 }
    }
    fn increment(&mut self, pulse_type: &PulseType) {
        match pulse_type {
            PulseType::High => self.high += 1,
            PulseType::Low => self.low += 1,
        }
    }
    fn product(&self) -> u64 {
        self.high * self.low
    }
}

fn prime_relay_conjunctions(map: &mut RelaySet) {
    map.iter()
        .flat_map(|(sender, v)| v.output.iter().map(|receiver| (*sender, *receiver)))
        // need this block due to borrow checker
        // TODO: how do I get around this?
        .collect::<Vec<_>>()
        .iter()
        //
        .for_each(|(sender, receiver)| {
            let entry = map.entry(receiver).and_modify(|relay| {
                if let Module::Conjunction(ref mut h) = relay.module {
                    h.insert(sender, false);
                }
            });
            // match entry {
            //     Entry::Occupied(_) => (),
            //     _ => panic!("entry must exist, or Input is broken"),
            // };
        });
}

#[derive(Debug, PartialEq)]
enum Module {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<&'static str, bool>),
}

impl Module {
    fn fire(&mut self, pulse: &Pulse) -> Option<PulseType> {
        match self {
            Self::Broadcaster => Some((*pulse).r#type),
            Self::FlipFlop(state) => match pulse.r#type {
                PulseType::High => None,
                PulseType::Low => {
                    *state = !*state;
                    Some(state.then(|| PulseType::High).unwrap_or(PulseType::Low))
                }
            },
            Self::Conjunction(ref mut state) => {
                state.entry(pulse.from).and_modify(|v| {
                    *v = match pulse.r#type {
                        PulseType::High => true,
                        _ => false,
                    }
                });
                let x = state
                    .iter()
                    .all(|(_, v)| *v)
                    .then(|| PulseType::Low)
                    .unwrap_or(PulseType::High);
                Some(x)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Relay {
    output: Vec<&'static str>,
    module: Module,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum PulseType {
    High,
    Low,
}

impl Display for PulseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PulseType::High => write!(f, "high"),
            PulseType::Low => write!(f, "low"),
        }
    }
}

fn parse_relay(input: &'static str) -> (&'static str, Relay) {
    let (mod_str, out_str) = input.split_once(" -> ").unwrap();

    let (m_n, module_type) = if mod_str == "broadcaster" {
        ("broadcaster", Module::Broadcaster)
    } else {
        match mod_str.split_at(1) {
            ("&", m_n) => (m_n, Module::Conjunction(HashMap::new())),
            ("%", m_n) => (m_n, Module::FlipFlop(false)),
            _ => panic!(),
        }
    };

    let output = out_str.split(", ").collect::<Vec<_>>();

    (
        m_n,
        Relay {
            output,
            module: module_type,
        },
    )
}

struct Pulse {
    from: &'static str,
    to: &'static str,
    r#type: PulseType,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[ignore]
    fn test_test_data_1() {
        let result = process(include_str!("../input-1-test-1.txt"));
        assert_eq!(result, 1);
    }

    #[rstest]
    #[ignore]
    fn test_test_data_2() {
        let result = process(include_str!("../input-1-test-2.txt"));
        assert_eq!(result, 1);
    }

    #[rstest]
    #[case("%fx -> kh, hl", ("fx",Relay {
        output: vec!["kh", "hl"],
        module: Module::FlipFlop(false),
    }))]
    #[case("broadcaster -> kh, hl", ("broadcaster", Relay {
        output: vec!["kh", "hl"],
        module: Module::Broadcaster,
    }))]
    #[case("&hr -> kh, hl", ("hr", Relay {
        output: vec!["kh", "hl"],
        module: Module::Conjunction(HashMap::new()),
    }))]
    fn test_parse_module_config(
        #[case] data: &'static str,
        #[case] expected: (&'static str, Relay),
    ) {
        assert_eq!(expected, parse_relay(data));
    }
}
