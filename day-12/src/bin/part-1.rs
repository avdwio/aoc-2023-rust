use std::{collections::HashMap, fmt::Debug, iter};

fn main() {
    let input = include_str!("./input-1.txt");
    println!("Result: {:?}", part1(input));
}

#[derive(Debug, Clone)]
enum Field {
    Op,
    Dmg,
    Ukn,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (_, Field::Ukn) => true,
            (Field::Ukn, _) => true,
            (Field::Op, Field::Op) => true,
            (Field::Dmg, Field::Dmg) => true,
            _ => false,
        }
    }
}

struct FieldRow(Vec<Field>);

fn stringify_field(field: &[Field]) -> String {
    let mut s = String::new();

    for f in field {
        s.push(match f {
            Field::Op => '#',
            Field::Dmg => '.',
            Field::Ukn => '?',
        });
    }
    s
}

impl Debug for FieldRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for field in &self.0 {
            s.push(match field {
                Field::Op => '#',
                Field::Dmg => '.',
                Field::Ukn => '?',
            });
        }

        write!(f, "{}", s)
    }
}

trait CanIntoFieldRow {
    type Error;

    fn into_field_row(&self, other: Self) -> Result<bool, Self::Error>;
}

impl CanIntoFieldRow for FieldRow {
    type Error = ();

    fn into_field_row(&self, other: Self) -> Result<bool, Self::Error> {
        Ok(self.0.len() == other.0.len())
    }
}

impl TryFrom<char> for Field {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Field::Op),
            '.' => Ok(Field::Dmg),
            '?' => Ok(Field::Ukn),
            _ => Err(()),
        }
    }
}
// HashMap<u64, BTreeSet<u64>>
// fn get_combinations_for(
//     groups: &[u64],
//     start_from: u64,
//     hashmap: &HashMap<u64, BTreeSet<u64>>,
//     depth: u64,
//     pos_history: &Vec<u64>,
// ) -> Option<Vec<Vec<u64>>> {
//     if groups.len() == 0 {
//         return Some(vec![vec![]]);
//     };
//     let curr_group = groups[0];
//     let curr_combos = hashmap.get(&curr_group).unwrap();
//     if curr_combos.range(start_from..).count() == 0 {
//         return None;
//     };
//     // println!("curr_combos: {:?}, depth: {:?}", curr_combos, depth);
//     let a = curr_combos
//         .range(start_from..)
//         .filter_map(|x| {
//             let next_start = *x + curr_group + 1;
//             let next_combos =
//                 get_combinations_for(&groups[1..], next_start, &hashmap, depth + 1, pos_history);

//             match next_combos {
//                 Some(mut r) => {
//                     r.iter_mut().for_each(|t| t.insert(0, *x));
//                     Some(r)
//                 }
//                 None => None,
//             }

//             // println!("next_combos: {:?}, depth: {:?}", next_combos, depth);
//         })
//         .flat_map(|x| x.into_iter())
//         .collect::<Vec<Vec<u64>>>();
//     Some(a)
// }

// fn recreate_field(groups: &[u64], placements: &[u64], len: u64) -> FieldRow {
//     let mut distance = placements
//         .windows(2)
//         .map(|x| x[1] - x[0])
//         .collect::<Vec<_>>();
//     distance.push(len - placements.last().unwrap());
//     let guess = FieldRow(
//         iter::repeat(Field::Dmg)
//             .take(placements[0] as usize + 1)
//             .chain(groups.iter().zip(distance.iter()).flat_map(|(g, d)| {
//                 iter::repeat(Field::Op)
//                     .take(*g as usize)
//                     .chain(iter::repeat(Field::Dmg).take((*d - *g).try_into().unwrap()))
//             }))
//             .collect::<Vec<_>>(),
//     );
//     guess
// }

/** (wiggle, #group) */
type PermCache = HashMap<(usize, u64, u64), u64>;

fn _field_perms(
    row_sl: &[Field],
    groups: &[u64],
    wiggle: u64,
    iter: u64,
    cache: &mut PermCache,
    th: bool,
) -> u64 {
    use Field::*;
    // println!("row_sl: {}", stringify_field(row_sl));
    // let _ = row_sl.starts_with(&vec![Field::Dmg]);
    // println!("wiggle: {:?}", wiggle);

    let buffer = if groups.len() == 1 { 0 } else { 1 };

    if groups.len() == 0 {
        // println!("no more groups, row_sl: {:?}", stringify_field(row_sl));
        return match row_sl.iter().zip(iter::repeat(Dmg)).all(|(a, b)| a.eq(&b)) {
            true => 1,
            false => 0,
        };
    };

    let curr_group = groups[0];

    let x = (0..=wiggle)
        .map(|n| {
            // check cache
            let cache_key = (groups.len() - 1, wiggle - n, n);
            // println!("checking... {:?}", cache_key);
            if th {
                if let Some(v) = cache.get(&cache_key) {
                    // println!("Found! {:?} :: {}", cache_key, v);
                    return *v;
                }
            }

            let partial = row_sl
                .iter()
                .zip(
                    iter::repeat(&Dmg)
                        .take((n) as usize)
                        .chain(iter::repeat(&Op).take(curr_group as usize))
                        .chain(iter::repeat(&Dmg).take(buffer)),
                )
                .all(|(a, b)| a.eq(&b));

            match partial {
                false => {
                    // println!("caching: {:?} :: 0", cache_key);
                    cache.insert(cache_key, 0);
                    0
                }
                // true => Some(n),
                true => {
                    let sect_len = (n + curr_group) as usize + buffer;
                    let perms = _field_perms(
                        &row_sl[sect_len..],
                        &groups[1..],
                        wiggle - n,
                        iter + 1,
                        cache,
                        th,
                    );
                    // println!("caching: {:?} :: {}", cache_key, perms);
                    cache.insert(cache_key, perms);
                    perms
                }
            }
        })
        .sum::<_>();
    // println!("x: {:?}", x);

    x
}

fn parse_line(line: &str) -> (Vec<Field>, Vec<u64>) {
    let mult = 5;
    let (field_str, config_str) = line.split_once(" ").unwrap();
    let field_iter = field_str
        .chars()
        .map(|c| TryInto::<Field>::try_into(c).unwrap());

    let field = iter::repeat(field_iter.clone().chain(iter::once(Field::Ukn)))
        .take(mult - 1)
        .flat_map(|x| x)
        .chain(field_iter)
        .collect::<Vec<_>>();

    let group_iter = config_str.split(",").map(|s| s.parse::<u64>().unwrap());

    let groups = iter::repeat(group_iter)
        .take(mult)
        .flat_map(|x| x)
        .collect::<Vec<_>>();

    (field, groups)
}

fn calculate_wiggle(field: &[Field], groups: &[u64]) -> u64 {
    let field_len = field.len() as u64;

    let min_len = groups.iter().map(|x| x + 1).sum::<u64>() - 1;
    field_len - min_len
}

fn part1(input: &str) -> u64 {
    let parsed = input
        .lines()
        .map(parse_line)
        .enumerate()
        // .skip(40)
        .map(|(i, (field, groups))| {
            // println!("i: {}, field: {}", i, stringify_field(&field));
            // println!("i: {}, groups: {:?}", i, groups);
            let cache = &mut PermCache::new();
            // let cache2 = &mut PermCache::new();
            let wiggle = calculate_wiggle(&field, &groups);
            let perms = _field_perms(&field, &groups, wiggle, 0, cache, true);
            // let perms_f = _field_perms(&field, &groups, wiggle, 0, cache2, true);
            // println!(
            //     "i: {}, perms: {:?}, perms_f: {:?} ; 2 is bigger? {:?}",
            //     i,
            //     perms,
            //     perms_f,
            //     perms_f > perms
            // );
            perms as u64
        })
        // .next()
        // .unwrap();
        // .take(1)
        .max()
        .unwrap();
    // .sum::<u64>();
    // .collect::<Vec<_>>();

    dbg!(&parsed);

    parsed
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[test]
    fn it_works() {
        let result = part1(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        assert_eq!(result, 525152);
    }
    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    fn resting(#[case] input: &str, #[case] expected: u64) {
        let cache = &mut PermCache::new();
        let (field, groups) = parse_line(input);
        let wiggle = calculate_wiggle(&field, &groups);
        let perms = _field_perms(&field, &groups, wiggle, 0, cache, true);
        assert_eq!(perms, expected);
    }
    // #[rstest]
    // #[case("???.### 1,1,3", 0)] // #.#.###
    // #[case(".??..??...?##. 1,1,3", 7)] // #.#.###
    // #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)] // #.###.#.######
    // #[case("????.#...#... 4,1,1", 5)] // ####.#.#
    // #[case("????.######..#####. 1,6,5", 5)] // #.######.#####
    // #[case("?###???????? 3,2,1", 4)] // ###.##.#
    // fn test_wiggle_calc(#[case] input: &str, #[case] expected: u64) {
    //     let (field, groups) = parse_line(input);
    //     let wiggle = calculate_wiggle(&field, &groups);
    //     assert_eq!(wiggle, expected);
    // }

    // #[test]
    // fn it_works_2() {
    //     let set = [1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3];
    //     let hash: HashMap<u64, BTreeSet<u64>> =
    //         HashMap::from([(1, BTreeSet::from([0, 1, 2])), (3, BTreeSet::from([0, 4]))]);

    //     let result = get_combinations_for(&set, 0, &hash, 0, &vec![]);
    //     todo!();
    //     // assert_eq!(result, 1);
    //     panic!()
    // }

    #[rstest]
    fn it_works_2() {
        use Field::*;
        assert_eq!(Ukn.eq(&Ukn), true);
        assert_eq!(Ukn.eq(&Op), true);
        assert_eq!(Ukn.eq(&Dmg), true);

        assert_eq!(Op.eq(&Ukn), true);
        assert_eq!(Op.eq(&Op), true);
        assert_eq!(Op.eq(&Dmg), false);

        assert_eq!(Op.eq(&Ukn), true);
        assert_eq!(Op.eq(&Op), true);
        assert_eq!(Op.eq(&Dmg), false);
    }
}
