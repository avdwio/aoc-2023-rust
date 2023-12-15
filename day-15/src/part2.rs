use std::collections::{HashMap, HashSet};

pub fn process(input: &'static str) -> u64 {
    box_sort(input)
}

fn get_hash_sum(input: &str) -> u64 {
    input.split(",").map(|x| get_hash(x)).sum::<_>()
}

type LensBox = HashMap<u64, HashMap<&'static str, (usize, u64)>>;

fn box_sort(input: &'static str) -> u64 {
    let mut lens_box: LensBox = HashMap::new();

    input.split(",").enumerate().for_each(|(i, x)| {
        let step = process_step(x);
        match step {
            Operation::Plus(left, right) => {
                let hash = get_hash(left);
                lens_box
                    .entry(hash)
                    .or_default()
                    .entry(left)
                    .and_modify(|x| x.1 = right)
                    .or_insert((i, right));
            }
            Operation::Minus(left) => {
                let hash = get_hash(left);
                lens_box.entry(hash).or_default().remove(left);
            }
        };
    });

    println!("{:?}", lens_box);

    lens_box
        .iter()
        .map(|(box_n, lenses)| {
            let mut a = lenses.iter().map(|(_, y)| y).collect::<Vec<_>>();
            a.sort_by(|a, b| a.0.cmp(&b.0));
            let sum = a
                .iter()
                .enumerate()
                .inspect(|(i, (_, y))| println!("{} * {} * {}", box_n + 1, i + 1, y))
                .map(|(i, (_, y))| (i + 1) as u64 * *y)
                .sum::<u64>();
            dbg!((box_n + 1) * sum)
        })
        .sum::<u64>()
}

#[derive(Debug, PartialEq)]
enum Operation {
    Plus(&'static str, u64),
    Minus(&'static str),
}

fn process_step(input: &'static str) -> Operation {
    if input.contains('=') {
        let (left, right_s) = input.split_once('=').unwrap();
        let right = right_s.parse::<u64>().unwrap();
        Operation::Plus(left, right)
    } else if input.contains('-') {
        let (left, _) = input.split_once('-').unwrap();
        Operation::Minus(left)
    } else {
        panic!("Invalid input: {}", input);
    }
}

fn get_hash(input: &str) -> u64 {
    input.chars().fold(0, |acc, c| {
        let mut new = acc + c as u64;
        new *= 17;
        new = new % 256;
        new
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn it_works() {
        let result = get_hash_sum("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, 1320);
    }

    #[test]
    fn it_works_2() {
        let ans = box_sort("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(ans, 145);
    }

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_hash(#[case] input: &str, #[case] expected: u64) {
        let result = get_hash(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("rn=1", Operation::Plus("rn", 1))]
    #[case("qp=3", Operation::Plus("qp", 3))]
    #[case("cm=2", Operation::Plus("cm", 2))]
    #[case("pc=4", Operation::Plus("pc", 4))]
    #[case("ot=9", Operation::Plus("ot", 9))]
    #[case("ab=5", Operation::Plus("ab", 5))]
    #[case("ot=7", Operation::Plus("ot", 7))]
    #[case("pc=6", Operation::Plus("pc", 6))]
    #[case("cm-", Operation::Minus("cm"))]
    #[case("pc-", Operation::Minus("pc"))]
    #[case("qp-", Operation::Minus("qp"))]
    fn test_hash_2(#[case] input: &'static str, #[case] expected: Operation) {
        let result = process_step(input);
        assert_eq!(result, expected);
    }
}
