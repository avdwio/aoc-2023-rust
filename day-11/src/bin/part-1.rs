use std::{
    collections::{BTreeSet, HashSet},
    ptr::hash,
};

use glam::UVec2;

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

struct StarMap {
    space: Vec<Vec<Space>>,
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

fn part1(input: &str) -> u32 {
    println!("Input: {:?}", &input);

    let starmap = into_starmap(&input);

    starmap.pretty_print();

    let mut filled_cols: BTreeSet<u32> = BTreeSet::new();
    let mut filled_rows: BTreeSet<u32> = BTreeSet::new();

    starmap.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, space)| match space {
            Space::Galaxy => {
                filled_cols.insert(x as u32);
                filled_rows.insert(y as u32);
            }
            _ => {}
        })
    });
    let starmap_width = starmap[0].len() as u32;
    let starmap_height = starmap.len() as u32;

    let expanded_starmap_width = starmap_width * 2 - filled_rows.len() as u32;

    let empty_row = (0..=expanded_starmap_width)
        .map(|_| ".")
        .collect::<String>();

    let expanded_map_str = starmap
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let mut n = line
                .iter()
                .enumerate()
                .map(|(x, space)| match filled_cols.contains(&(x as u32)) {
                    true => space.to_string(),
                    false => "..".to_string(),
                })
                .collect::<String>();
            n.push_str("\n");
            if !filled_rows.contains(&(y as u32)) {
                n.push_str(empty_row.as_str());
                n.push_str("\n");
                println!("Empty row: {:?}", &y)
            }
            n
        })
        .collect::<String>();

    let expanded_map = into_starmap(&expanded_map_str);

    println!("Filled cols: {:?}", &filled_cols);
    println!("Filled rows: {:?}", &filled_rows);
    expanded_map.pretty_print();

    let mut stars: HashSet<UVec2> = HashSet::new();

    expanded_map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, space)| match space {
            Space::Galaxy => {
                stars.insert(UVec2 {
                    x: x as u32,
                    y: y as u32,
                });
            }
            _ => {}
        })
    });

    println!("Stars: {:?}", &stars);

    let ans = stars
        .iter()
        .map(|s1| {
            stars
                .iter()
                .map(move |(s2)| {
                    let d = s1.clone().as_ivec2() - s2.as_ivec2();

                    let abs = d.abs();
                    abs.x + abs.y
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    u32::try_from(ans).unwrap() / 2
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
