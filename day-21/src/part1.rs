use std::collections::HashSet;

pub fn process(input: &str) -> u64 {
    let (start, rock_map, size) = parse_input(input);

    print_garden(&size, &rock_map, &HashSet::new(), &start);

    let mut visited = HashSet::new();
    visited.insert(start);

    for i in 0..64 {
        visited = visited
            .iter()
            .flat_map(|pos| get_neighbors(pos))
            .filter(|pos| !rock_map.contains(pos))
            .filter(|pos| pos.0 < size.0 && pos.1 < size.1)
            .collect::<HashSet<Position>>();
    }
    println!();
    print_garden(&size, &rock_map, &visited, &start);

    visited.len() as u64
}

fn get_neighbors(pos: &Position) -> Vec<Position> {
    let mut neighbors = Vec::new();
    pos.0
        .checked_sub(1)
        .and_then(|x| Some(neighbors.push((x, pos.1))));
    neighbors.push((pos.0 + 1, pos.1));
    pos.1
        .checked_sub(1)
        .and_then(|y| Some(neighbors.push((pos.0, y))));
    neighbors.push((pos.0, pos.1 + 1));
    neighbors
}

type Position = (usize, usize);
type Positions = HashSet<Position>;

fn parse_input(input: &str) -> (Position, HashSet<Position>, Position) {
    let (start_vec, map_vec): (Vec<Option<Position>>, Vec<Option<Position>>) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((None, Some((x, y)))),
                'S' => Some((Some((x, y)), None)),
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
        (input.lines().next().unwrap().len(), input.lines().count()),
    )
}

fn print_garden(size: &Position, rock_map: &Positions, visited_map: &Positions, start: &Position) {
    for y in 0..size.1 {
        for x in 0..size.0 {
            if rock_map.contains(&(x, y)) {
                print!("#");
            } else if visited_map.contains(&(x, y)) {
                print!("O");
            } else if start == &(x, y) {
                print!("S");
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
