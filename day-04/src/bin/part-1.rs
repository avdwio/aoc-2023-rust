use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input-1.txt");
    println!("Result: {:?}", process(input));
}

fn process(input: &str) -> u32 {
    let mut copies = HashMap::new();

    input
        .lines()
        .enumerate()
        .map(|(i, x)| {
            let score = process_line(x);
            let special_i = u32::try_from(i).unwrap();
            let card_count = *copies.get(&special_i).unwrap_or(&0) + 1;

            for j in special_i..(special_i + score) {
                copies.insert(j + 1, copies.get(&(j + 1)).unwrap_or(&0) + card_count);
            }

            card_count
        })
        .sum()
}

fn process_line(line: &str) -> u32 {
    let mut iter = line.split(": ");

    iter.next();

    let mut numbers = iter.next().unwrap().split(" | ");

    let winning_numbers: HashSet<_> = numbers
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|x| {
            let result = x.parse::<u32>();
            match result {
                Ok(x) => Some(x),
                Err(_) => None,
            }
        })
        .collect();
    numbers
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|x| {
            let result = x.parse::<u32>();
            match result {
                Ok(x) => Some(x),
                Err(_) => None,
            }
        })
        .filter(|x| winning_numbers.contains(x))
        .fold(0 as u32, |acc, _| acc + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn it_works() {
        let result = process(include_str!("./input-1-test.txt"));
        assert_eq!(result, 30);
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 4)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn line_test(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(input))
    }
}
