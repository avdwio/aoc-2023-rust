use std::{collections::HashMap, path::Iter};

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
    let node = DirectionMap { left, right };
    (first, node)
}

fn part1(input: &'static str) -> u32 {
    let (_instructions, nodes) = input.split_once("\n\n").unwrap();

    let desert_map = nodes
        .lines()
        .map(|node| node_parser(node))
        .collect::<HashMap<_, _>>();

    // let ans = _instructions.chars().fold(("AAA", 1), |acc, x| {
    //     if acc.0 == "ZZZ" {
    //         return acc;
    //     };
    //     let next_node = match x {
    //         'L' => desert_map.get(acc.0).unwrap().left,
    //         'R' => desert_map.get(acc.0).unwrap().right,
    //         _ => panic!("Unknown direction"),
    //     };
    //     (next_node, acc.1 + 1)
    // });

    let __instructions = _instructions.chars().collect::<Vec<_>>();
    println!("{:?}", __instructions.len());

    let mut curr = "AAA";
    let mut step = 0;
    while curr != "ZZZ" {
        curr = match __instructions[step % __instructions.len()] {
            'L' => desert_map.get(curr).unwrap().left,
            'R' => desert_map.get(curr).unwrap().right,
            _ => panic!("Unknown direction"),
        };
        step = step + 1;
        println!("{} -> {}", step, curr)
    }

    step.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let result = part1(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works_2() {
        let result = part1(
            "LLRLLRLLRLLRLLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 6);
    }
}
