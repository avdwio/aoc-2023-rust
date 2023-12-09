use core::panic;
use std::collections::HashMap;
use std::fmt::Formatter;

fn main() {
    let input = include_str!("./input-1.txt");
    println!("Result: {:?}", part1(input));
}

#[derive(Debug)]
struct DirectionMap {
    left: &'static str,
    right: &'static str,
}

fn node_parser(input: &'static str) -> (&str, DirectionMap) {
    let (first, other) = input.split_once(" = (").unwrap();
    let (left, rest) = other.split_once(", ").unwrap();
    let (right, _) = rest.split_once(")").unwrap();
    let paths = DirectionMap { left, right };
    (first, paths)
}

fn check_nodes_end_in_z(input: &Vec<&str>) -> bool {
    input.iter().all(|x| x.ends_with("Z"))
}

fn get_nodes_end_in_a<'a>(input: &Vec<&'a str>) -> Vec<&'a str> {
    input
        .iter()
        .filter_map(|x| match x.ends_with("A") {
            true => Some(*x),
            false => None,
        })
        .collect::<Vec<_>>()
}

fn follow_map<'a>(input: &str, instruction: char, map: &HashMap<&'a str, DirectionMap>) -> &'a str {
    match instruction {
        'L' => map.get(input).unwrap().left,
        'R' => map.get(input).unwrap().right,
        _ => panic!("Unknown direction"),
    }
}

struct SuccessLoc {
    loc: &'static str,
    step: usize,
    ptr: usize,
}

impl core::fmt::Debug for SuccessLoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "SuccessLoc {{ loc: {}, step: {}, ptr: {} }}",
            self.loc, self.step, self.ptr
        )
    }
}

fn have_i_been_here_before(s_l: &SuccessLoc, vec: &Vec<SuccessLoc>) -> bool {
    vec.iter().any(|x| x.loc == s_l.loc && x.ptr == s_l.ptr)
}

pub fn lcm(nums: &[usize]) -> usize {
    match nums {
        [a] => *a,
        [a, b @ ..] => {
            let b_star = lcm(&b);
            a * b_star / gcd(*a, b_star)
        }
        _ => panic!("lcm called with empty list"),
    }
}

fn gcd(a: usize, b: usize) -> usize {
    match b == 0 {
        true => a,
        false => gcd(b, a % b),
    }
}

fn part1(input: &'static str) -> u64 {
    let (instr_str, nodes) = input.split_once("\n\n").unwrap();

    let desert_map = nodes
        .lines()
        .map(|node| node_parser(node))
        .collect::<HashMap<_, _>>();

    let all_locs = desert_map.keys().map(|x| *x).collect::<Vec<_>>();

    let start_locs = get_nodes_end_in_a(&all_locs);

    dbg!(&start_locs);

    let instructions = instr_str.chars().collect::<Vec<_>>();

    dbg!("canary");

    let vec_of_interest = start_locs
        .iter()
        .map(|start_loc| {
            let mut success_locs = Vec::<SuccessLoc>::new();

            let mut curr_step = 0;
            let mut curr_ptr = 0;
            let mut curr_loc = *start_loc;
            loop {
                // do step then add to vec
                let instruction = instructions[curr_ptr];
                curr_loc = follow_map(curr_loc, instruction, &desert_map);
                let next_el = SuccessLoc {
                    loc: curr_loc,
                    step: curr_step + 1,
                    ptr: curr_ptr,
                };
                if curr_loc.ends_with("Z") {
                    // have I been here before?
                    if have_i_been_here_before(&next_el, &success_locs) {
                        break;
                    };
                    // push to vec

                    success_locs.push(next_el);
                }

                // ++
                curr_step += 1;
                curr_ptr = curr_step % instructions.len();
            }
            success_locs
        })
        .collect::<Vec<_>>();

    let shortest_walk = vec_of_interest[0]
        .iter()
        .filter_map(|state_of_interest| {
            let steps_group = vec_of_interest[1..]
                .iter()
                .map(|path| {
                    match path
                        .iter()
                        .find(|other_state| other_state.ptr == state_of_interest.ptr)
                    {
                        Some(s) => Some(s.step),
                        None => None,
                    }
                })
                .collect::<Option<Vec<_>>>();

            match steps_group {
                Some(mut steps) => {
                    steps.push(state_of_interest.step);
                    Some(lcm(steps.as_slice()))
                }
                None => None,
            }
        })
        .min()
        .unwrap();

    shortest_walk.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcm() {
        let result = lcm(&[5, 8, 12]);
        assert_eq!(result, 120);
    }

    #[test]
    fn test_lc_2() {
        let result = lcm(&[18023, 21251, 15871, 16409, 14257, 11567]);
        assert_eq!(result, 11678319315857);
    }
}
