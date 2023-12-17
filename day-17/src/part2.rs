use std::{
    collections::{BTreeMap, HashMap},
    iter,
};

use colored::Colorize;

pub fn process(input: &'static str) -> u32 {
    let grid = into_grid(input);
    // println!("here {:?}", grid.get(1, 1));

    find_shortest_walk(&grid)
}

type Position = (usize, usize);
// key = posiition, heading, fwd_count
// value = lowest cost to get to that position
type DjikstraCache = HashMap<(Position, CardinalDir), BTreeMap<u8, u32>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CardinalDir {
    North,
    East,
    South,
    West,
}

impl CardinalDir {
    fn turn(&self, dir: Dir) -> CardinalDir {
        match dir {
            Dir::Forward => self.clone(),
            Dir::Right => match self {
                CardinalDir::North => CardinalDir::East,
                CardinalDir::East => CardinalDir::South,
                CardinalDir::South => CardinalDir::West,
                CardinalDir::West => CardinalDir::North,
            },
            Dir::Left => match self {
                CardinalDir::North => CardinalDir::West,
                CardinalDir::East => CardinalDir::North,
                CardinalDir::South => CardinalDir::East,
                CardinalDir::West => CardinalDir::South,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Forward,
    Left,
    Right,
}

fn try_get_next_coords(pos: &Position, dir: &CardinalDir) -> Option<Position> {
    match dir {
        CardinalDir::North => pos.1.checked_sub(1).map(|y| (pos.0, y)),
        CardinalDir::East => pos.0.checked_add(1).map(|x| (x, pos.1)),
        CardinalDir::South => pos.1.checked_add(1).map(|y| (pos.0, y)),
        CardinalDir::West => pos.0.checked_sub(1).map(|x| (x, pos.1)),
    }
}

#[derive(Debug, Clone)]
struct Crucible<'a> {
    cost: u32,
    heading: CardinalDir,
    heading_history: Vec<CardinalDir>,
    pos_history: Vec<(usize, usize)>,
    fwd_count: u8,
    grid: &'a Grid,
}

impl Crucible<'_> {
    fn get_current_pos(&self) -> &Position {
        self.pos_history.first().expect("should have a position")
    }

    fn has_been_at_current_pos(&self) -> bool {
        self.pos_history
            .iter()
            .skip(1)
            .any(|p| *p == *self.get_current_pos())
    }

    fn try_move(&mut self, dir: Dir) -> Result<(), &'static str> {
        match dir {
            Dir::Forward => {
                self.fwd_count += 1;
                if self.fwd_count >= 10 {
                    return Err("too many forward moves");
                }
            }
            _ => {
                if self.fwd_count < 3 {
                    return Err("must move forward at least 4 times");
                }
                self.fwd_count = 0
            }
        }
        let next_step = self.heading.turn(dir);

        let Some((next_pos, next_cost)) = try_get_next_coords(self.get_current_pos(), &next_step)
            .and_then(|p| self.grid.get(p.0, p.1).map(|c| (p, c)))
        else {
            return Err("position out of bounds");
        };

        self.pos_history.insert(0, next_pos);
        self.heading_history.insert(0, next_step);
        self.cost += next_cost;
        self.heading = next_step;

        if self.fwd_count < 3 {
            self.try_move(Dir::Forward)
        } else {
            Ok(())
        }
    }
}

fn find_next_walk(
    mut crucible: Crucible,
    next_move: Dir,
    current_min: u32,
    cache: &mut DjikstraCache,
) -> Option<u32> {
    // if invalid move, return None
    match crucible.try_move(next_move) {
        Err(_) => {
            // println!("invalid move");
            // println!(
            //     "curr pos: {:?}; dir: {:?}",
            //     crucible.get_current_pos(),
            //     crucible.heading.turn(next_move)
            // );
            return None;
        }
        _ => (),
    };
    if crucible.fwd_count >= 3 {
        let inner_cache = cache
            .entry((*crucible.get_current_pos(), crucible.heading))
            .or_default();

        let is_all = inner_cache
            .range(3..=crucible.fwd_count)
            .all(|e| *e.1 > crucible.cost);
        if is_all {
            inner_cache.insert(crucible.fwd_count, crucible.cost);
        } else {
            // been here before
            // println!("cache hit; heading: {:?}, cu", crucible.heading);
            // println!("his: {:?}", crucible.pos_history);
            // println!("his: {:?}", crucible.pos_history);
            return None;
        }
    }

    // if we've been here before, return None
    if crucible.has_been_at_current_pos() {
        // println!("been here before");
        // println!("curr pos: {:?}", crucible.get_current_pos());
        return None;
    }

    // if new cost is greater than current min, return None
    if crucible.cost >= current_min {
        // println!(
        //     "cost higher than current min: {} > {}",
        //     crucible.cost, current_min
        // );
        // println!("curr pos: {:?}", crucible.get_current_pos());
        return None;
    }

    // are we done? return the curent crucible cost
    if *crucible.get_current_pos() == (crucible.grid.dims.0 - 1, crucible.grid.dims.1 - 1) {
        // check that min run has been reached
        if crucible.fwd_count < 3 {
            // println!("min run not reached");
            // println!("curr pos: {:?}", crucible.get_current_pos());
            return None;
        }

        println!("found a solution: {:?}", crucible.cost);
        // println!("curr pos: {:?}", crucible.get_current_pos());
        // println!("curr pos: {:?}", crucible.pos_history);
        // crucible.grid.print(&crucible);
        return Some(crucible.cost);
    }

    // are we not done?
    // recurse over left, fwd, right
    let least = iter::once(Dir::Left)
        .chain(iter::once(Dir::Forward))
        .chain(iter::once(Dir::Right))
        .fold(current_min, |acc, d| {
            find_next_walk(crucible.clone(), d, acc, cache).unwrap_or(acc)
        });
    // return the crucible with the lowest cost
    Some(least)
}

fn find_shortest_walk(grid: &Grid) -> u32 {
    let min_so_far = 9999;

    let cache = &mut HashMap::new();

    let res = vec![
        (
            Dir::Left,
            Crucible {
                pos_history: vec![(1, 0)],
                heading: CardinalDir::East,
                heading_history: vec![CardinalDir::East],
                cost: grid.get(1, 0).unwrap(),
                fwd_count: 0,
                grid: &grid,
            },
        ),
        (
            Dir::Forward,
            Crucible {
                pos_history: vec![(1, 0)],
                heading: CardinalDir::East,
                heading_history: vec![CardinalDir::East],
                cost: grid.get(1, 0).unwrap(),
                fwd_count: 0,
                grid: &grid,
            },
        ),
        (
            Dir::Forward,
            Crucible {
                pos_history: vec![(0, 1)],
                heading: CardinalDir::South,
                heading_history: vec![CardinalDir::South],
                cost: grid.get(0, 1).unwrap(),
                fwd_count: 0,
                grid: &grid,
            },
        ),
        (
            Dir::Right,
            Crucible {
                pos_history: vec![(0, 1)],
                heading: CardinalDir::South,
                heading_history: vec![CardinalDir::South],
                cost: grid.get(0, 1).unwrap(),
                fwd_count: 0,
                grid: &grid,
            },
        ),
    ]
    .iter()
    .fold(min_so_far, |acc, (d, cr)| {
        let res = find_next_walk(cr.clone(), *d, acc, cache);
        res.unwrap_or(acc)
    });

    res
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<&'static str>,
    dims: Position,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<u32> {
        self.cells
            .get(y)
            .and_then(|row| row.chars().nth(x).and_then(|f| f.to_digit(10)))
    }

    fn print(&self, cr: &Crucible) {
        let hm = cr
            .pos_history
            .iter()
            .zip(cr.heading_history.iter())
            .collect::<HashMap<&Position, &CardinalDir>>();

        println!();
        self.cells.iter().enumerate().for_each(|(y, row)| {
            row.chars().enumerate().for_each(|(x, c)| {
                // let s = "x".red().to_string;
                if let Some(dir) = hm.get(&(x, y)) {
                    match dir {
                        CardinalDir::North => print!("{}", "^".red()),
                        CardinalDir::East => print!("{}", ">".red()),
                        CardinalDir::South => print!("{}", "v".red()),
                        CardinalDir::West => print!("{}", "<".red()),
                    }
                } else {
                    print!("{}", c)
                }
            });
            println!()
        });
        println!()
    }
}

fn into_grid(input: &'static str) -> Grid {
    let cells = input.lines().collect::<Vec<_>>();

    let dims = (cells[0].len(), cells.len());

    Grid { cells, dims }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_works() {
        let result = process(
            "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        );
        assert_eq!(result, 102);
    }

    #[test]
    fn test_1() {
        let result = process(
            "111
991
991",
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let result = process(
            "199
199
111",
        );
        assert_eq!(result, 4);
    }

    #[test]
    // #[ignore]
    fn test_3() {
        let result = process(
            "1999
1999
1999
1999
1111",
        );
        assert_eq!(result, 15);
    }

    #[test]
    // #[ignore]
    fn test_4() {
        let result = process(
            "1999999999
1999999999
1999999999
1999999999
1999999999
1999999999
1999999999
1999999999
1999999999
1999999999
1111999999",
        );
        assert_eq!(result, 15);
    }
}
