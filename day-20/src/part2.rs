use core::panic;
use std::{
    collections::{BTreeMap, VecDeque},
    fmt::{Display, Formatter},
};

type RelaySet = BTreeMap<&'static str, Relay>;

pub fn process(input: &'static str) -> u64 {
    let mut map = input.lines().map(parse_relay).collect::<BTreeMap<_, _>>();

    prime_relay_conjunctions(&mut map);
    let mut pulse_count = None;

    let mut btn_count = 1;

    let mut last_iter = 0;

    while btn_count < 5000 {
        button_click(&mut map, &mut pulse_count, btn_count);

        let res = vec![
            "jr", "xs", "mg", "lt", "cp", "ln", "rr", "rl", "vh", "vb", "dp", "hh",
        ]
        .iter()
        .all(|x| match map.get(x).unwrap().module {
            Module::FlipFlop(x) => x,
            _ => panic!(),
        });
        // if res {
        //     println!(
        //         "all flipflops are active :: iter: {} :: periodicity: {}",
        //         btn_count,
        //         btn_count - last_iter
        //     );
        //     last_iter = btn_count;
        // }

        // println!("{}", print_active_flipflops(&map));

        btn_count += 1;
    }
    pulse_count.as_ref().map(|p| {
        println!("high: {} low {}", p.high, p.low);
        Some(1)
    });
    pulse_count.map(|p| p.product()).unwrap_or(0)
}
fn conj_is_active(ref map: &BTreeMap<&str, bool>) -> bool {
    map.iter().all(|(_, v)| *v)
}

fn print_active_flipflops(map: &BTreeMap<&str, Relay>) -> String {
    map.iter()
        .filter_map(|(k, v)| match v.module {
            Module::FlipFlop(x) => Some((k, x)),
            _ => None,
        })
        .map(|(k, v)| format!("{:>3} {}", k, if v { '↗' } else { '↘' }))
        .collect::<String>()
}

fn all_flipflops_active(map: &BTreeMap<&str, Relay>) -> bool {
    vec!["rr", "dp", "hh", "cp", "jr", "vb", "vh", "lt", "rl"]
        .iter()
        .all(|x| match map.get(x).unwrap().module {
            Module::FlipFlop(x) => x,
            _ => panic!(),
        })
    // map.iter()
    //     .filter_map(|(_, x)| match x.module {
    //         Module::FlipFlop(x) => Some(x),
    //         _ => None,
    //     })
    //     .all(|x| x)
}

fn pretty_print_conj_map(map: &BTreeMap<&'static str, bool>) -> String {
    map.iter()
        .map(|(k, v)| format!("{:>3}: {:>5}", k, v))
        .collect::<Vec<_>>()
        .join(",")
}

fn button_click(map: &mut RelaySet, pulse_count: &mut Option<PulseCounter>, global_iter: u64) {
    let mut pulses = VecDeque::<Pulse>::new();

    pulses.push_back(Pulse {
        from: "button",
        to: "broadcaster",
        r#type: PulseType::Low,
    });

    let mut i = 1;

    while let Some(pulse) = pulses.pop_front() {
        for mod_name in vec!["ql", "hl", "hq", "bc"] {
            let Module::Conjunction(ref map2) = map.get(mod_name).unwrap().module else {
                panic!();
            };

            if map.iter().all(|(_, v)| *v) {
                println!("mod: {} :: iter: {:>10} :: {}", mod_name, global_iter, i);
            }
        }

        // if all_flipflops_active(&map) {
        //     println!("active after {} iterations :: {}", i, global_iter);
        // }
        i += 1;

        // println!("{} -{}-> {}", pulse.from, pulse.r#type, pulse.to);
        pulse_count.as_mut().map(|p| p.increment(&pulse.r#type));

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
    Conjunction(BTreeMap<&'static str, bool>),
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
            ("&", m_n) => (m_n, Module::Conjunction(BTreeMap::new())),
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
        module: Module::Conjunction(BTreeMap::new()),
    }))]
    fn test_parse_module_config(
        #[case] data: &'static str,
        #[case] expected: (&'static str, Relay),
    ) {
        assert_eq!(expected, parse_relay(data));
    }
}
