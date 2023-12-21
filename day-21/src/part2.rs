use colored::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn process(input: &str) -> u64 {
    let (start, rock_map, size) = parse_input(input);

    print_garden(&size, &rock_map, &HashSet::new(), &start);

    let mut visited_one = HashSet::new();
    visited_one.insert(start);

    let mut visited_other = HashSet::new();

    let mut cumulative_visited_one = HashSet::new();
    let mut cumulative_visited_other = HashSet::new();

    for i in 1..=1375 {
        (
            visited_one,
            visited_other,
            cumulative_visited_one,
            cumulative_visited_other,
        ) = {
            let v_o = get_steps(&visited_one, &cumulative_visited_other, &rock_map, &size);

            cumulative_visited_other.extend(v_o.clone());
            (
                v_o,
                visited_one,
                cumulative_visited_other,
                cumulative_visited_one,
            )
        };

        //=====
        // visited_one = get_steps(&visited_other, &cumulative_visited_one, &rock_map, &size);

        // cumulative_visited_one.extend(visited_one.clone());

        if i % 131 == 65 {
            println!("{}: {}", i, cumulative_visited_one.len());
        }
        // pause();
        // print_garden(&size, &rock_map, &cumulative_visited_other, &start);
    }

    // (1..6).cartesian_product(1..8).for_each(|x| {
    //     println!("{:?}", x);
    // });

    // let x = { 1 };

    // print_garden(&size, &rock_map, &visited_other, &start);

    fn get_steps(
        visited: &HashSet<Position>,
        visited_other: &HashSet<Position>,
        rock_map: &HashSet<Position>,
        size: &Position,
    ) -> HashSet<Position> {
        visited
            .iter()
            .flat_map(|pos| get_neighbors(pos))
            .filter(|pos| {
                let rem = (pos.0.rem_euclid(size.0), pos.1.rem_euclid(size.1));
                !rock_map.contains(&rem)
            })
            .filter(|pos| !visited_other.contains(pos))
            .collect::<HashSet<Position>>()
    }

    cumulative_visited_one.len() as u64
}

fn get_neighbors(pos: &Position) -> Vec<Position> {
    let mut neighbors = Vec::new();
    neighbors.push((pos.0 + 1, pos.1));
    neighbors.push((pos.0 - 1, pos.1));
    neighbors.push((pos.0, pos.1 + 1));
    neighbors.push((pos.0, pos.1 - 1));
    neighbors
}

type Position = (i64, i64);
type Positions = HashSet<Position>;

fn parse_input(input: &str) -> (Position, HashSet<Position>, Position) {
    let (start_vec, map_vec): (Vec<Option<Position>>, Vec<Option<Position>>) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((None, Some((x as i64, y as i64)))),
                'S' => Some((Some((x as i64, y as i64)), None)),
                _ => None,
            })
        })
        .unzip();
    let map = map_vec
        .into_iter()
        .filter_map(|x| x)
        .collect::<HashSet<Position>>();
    let start = start_vec.into_iter().find_map(|x| x);

    (
        start.unwrap(),
        map,
        (
            input.lines().next().unwrap().len() as i64,
            input.lines().count() as i64,
        ),
    )
}

fn print_garden(size: &Position, rock_map: &Positions, visited_map: &Positions, start: &Position) {
    for y in 0..size.1 {
        for x in 0..size.0 {
            if rock_map.contains(&(x, y)) {
                print!("{}", "#".blue());
            } else if visited_map.contains(&(x, y)) {
                print!("{}", "O".red());
            } else if start == &(x, y) {
                print!("{}", "S".green());
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = process(
            "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
        );
        assert_eq!(result, 16);
    }
}
