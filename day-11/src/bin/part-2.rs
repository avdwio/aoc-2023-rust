use core::panic;
use std::collections::{BTreeSet, HashSet};

use glam::U64Vec2;

fn main() {
    let input = include_str!("./input-1.txt");
    println!("Result: {:?}", part1(input));
}

enum Space {
    Galaxy,
    Empty,
}

impl ToString for Space {
    fn to_string(&self) -> String {
        match self {
            Space::Galaxy => "#".to_string(),
            Space::Empty => ".".to_string(),
        }
    }
}

trait PrettyPrint {
    fn pretty_print(&self);
}

fn into_starmap(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    '#' => Space::Galaxy,
                    _ => panic!("Unknown space type"),
                })
                .collect::<Vec<Space>>()
        })
        .collect::<Vec<_>>()
}

impl PrettyPrint for Vec<Vec<Space>> {
    fn pretty_print(&self) {
        self.iter().for_each(|line| {
            line.iter().for_each(|space| match space {
                Space::Galaxy => print!("#"),
                Space::Empty => print!("."),
            });
            println!();
        });
    }
}

fn part1(input: &str) -> u64 {
    let starmap = into_starmap(&input);

    starmap.pretty_print();

    let mut filled_cols: BTreeSet<u64> = BTreeSet::new();
    let mut filled_rows: BTreeSet<u64> = BTreeSet::new();

    let mut stars = HashSet::new();

    starmap.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, space)| match space {
            Space::Galaxy => {
                filled_cols.insert(x as u64);
                filled_rows.insert(y as u64);
                stars.insert(U64Vec2::new(x as u64, y as u64));
            }
            _ => {}
        })
    });
    let ans = stars
        .iter()
        .flat_map(|s1| {
            stars.iter().map(move |s2| {
                let min_v = U64Vec2::new(s1.x.min(s2.x), s1.y.min(s2.y));
                let max_v = U64Vec2::new(s1.x.max(s2.x), s1.y.max(s2.y));
                // println!("s1:{:?}, s2:{:?}", &s1, &s2);
                (min_v, max_v)
            })
        })
        .map(|(min_v, max_v)| {
            let v_stars_between = filled_rows.range(min_v.y..max_v.y).count() as u64;
            let v_dist = max_v.y - min_v.y;
            let v_spaces = v_dist - v_stars_between;

            let h_stars_between = filled_cols.range(min_v.x..max_v.x).count() as u64;
            let h_dist = max_v.x - min_v.x;
            let h_spaces = h_dist - h_stars_between;

            // println!(
            //     "mn:{:?}, mx:{:?}, V btwn: {:?}, vdist:{:?}, vspaces:{:?}",
            //     min_v, max_v, v_stars_between, v_dist, v_spaces
            // );

            let n = 1000000;
            let dist = h_dist + (n - 1) * h_spaces + v_dist + (n - 1) * v_spaces;
            dist
        })
        .sum::<u64>();
    // .max()
    // .unwrap();

    ans / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, 374);
    }
}
