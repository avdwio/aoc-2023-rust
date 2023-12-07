use std::{cmp::Ordering, collections::HashMap, vec};

fn main() {
    let input = include_str!("./input.txt");
    println!("Result: {:?}", process(input));
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: &'static str,
    score: u32,
}

impl From<&str> for HandType {
    fn from(s: &str) -> Self {
        let mut letter_counts: HashMap<char, u8> = HashMap::new();
        let char_vec: Vec<char> = s.chars().collect();
        for c in char_vec {
            *letter_counts.entry(c).or_insert(0) += 1;
        }

        let mut specialvar = letter_counts.iter().map(|(_, y)| *y).collect::<Vec<_>>();
        specialvar.sort_by(|a, b| b.cmp(a));

        let x = specialvar.iter().fold(
            (
                u8::from(0),
                u8::from(0),
                u8::from(0),
                u8::from(0),
                u8::from(0),
            ),
            |acc, x| match acc {
                (0, 0, 0, 0, 0) => (*x, 0, 0, 0, 0),
                (a, 0, 0, 0, 0) => (a, *x, 0, 0, 0),
                (a, b, 0, 0, 0) => (a, b, *x, 0, 0),
                (a, b, c, 0, 0) => (a, b, c, *x, 0),
                (a, b, c, d, 0) => (a, b, c, d, *x),
                _ => panic!(),
            },
        );

        match x {
            (1, 1, 1, 1, 1) => HandType::HighCard,
            (2, 1, 1, 1, 0) => HandType::OnePair,
            (2, 2, 1, 0, 0) => HandType::TwoPairs,
            (3, 1, 1, 0, 0) => HandType::ThreeOfAKind,
            (3, 2, 0, 0, 0) => HandType::FullHouse,
            (4, 1, 0, 0, 0) => HandType::FourOfAKind,
            (5, 0, 0, 0, 0) => HandType::FiveOfAKind,
            _ => panic!(),
        }
    }
}

trait Rank {
    fn rank(&self) -> u8;
}

impl Rank for char {
    fn rank(&self) -> u8 {
        match self {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => self.to_digit(10).unwrap() as u8,
        }
    }
}

fn raw_compare(left: &str, right: &str) -> Ordering {
    let x = left
        .chars()
        .into_iter()
        .zip(right.chars().into_iter())
        .filter_map(|(left, right)| {
            let x = left.rank().partial_cmp(&right.rank());
            match x {
                Some(Ordering::Equal) => None,
                Some(x) => Some(x),
                _ => panic!("Invalid input"),
            }
        })
        .find(|_| true);

    match x {
        Some(x) => x,
        None => panic!("Invalid input"),
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        // self.iter().zip(other.iter()).find(|(left, right)| {});
        true
    }
}

// impl PartialOrd for Hand {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         let ord = self
//             .ordered_hand
//             .iter()
//             .zip(other.ordered_hand.iter())
//             .filter_map(|(left, right)| {
//                 if left.0 > right.0 {
//                     return Some(Ordering::Greater);
//                 } else if left.0 < right.0 {
//                     return Some(Ordering::Less);
//                 } else {
//                     return None;
//                 }
//             })
//             .find(|_| true);
//         Some(Ordering::Equal)
//     }
// }

fn process(input: &'static str) -> u32 {
    let mut hands = input
        .lines()
        .map(|line| {
            let mut it = line.split(" ");
            let cards = it.next().unwrap();
            let score = it.next().unwrap().parse::<_>().unwrap();
            let hand_type = HandType::from(cards);
            Hand {
                hand_type,
                cards,
                score,
            }
        })
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| {
        let c = a.hand_type.partial_cmp(&b.hand_type);
        match c {
            Some(Ordering::Equal) => raw_compare(a.cards, b.cards),
            Some(x) => x,
            None => panic!(),
        }
    });

    let result = hands
        .iter()
        .enumerate()
        .fold::<u32, _>(0, |acc, (i, hand)| {
            let modified_score = (u32::try_from(i).unwrap() + 1) * hand.score;
            println!("{:?}", modified_score);
            modified_score + acc
        });

    dbg!(&hands);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn it_works() {
        let input = include_str!("./input-test.txt");
        dbg!(input);
        let result = process(input);
        assert_eq!(result, 6440);
    }

    // #[rstest]
    // #[case("32T3K 765", 0)]
    // #[case("T55J5 684", 0)]
    // #[case("KK677 28", 0)]
    // #[case("KTJJT 220", 0)]
    // #[case("QQQJA 483", 0)]
    // fn test_priority(#[case] input: &'static str, #[case] expected: u32) {
    //     assert_eq!(expected, process(input))
    // }

    // #[rstest]
    // #[case("32T3K", HandType::OnePair)]
    // #[case("T55J5", HandType::ThreeOfAKind)]
    // #[case("KK677", HandType::TwoPairs)]
    // #[case("KTJJT", HandType::TwoPairs)]
    // #[case("QQQJA", HandType::ThreeOfAKind)]
    // fn test_hand_match(#[case] input: &str, #[case] expected: HandType) {
    //     assert_eq!(expected, HandType::from(input))
    // }

    // #[rstest]
    // fn is_higher() {
    //     assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
    // }
}
