use std::{
    collections::{BTreeMap, HashMap},
    iter,
    rc::Rc,
};

pub fn process(input: &'static str) -> u32 {
    let grid = into_grid(input);
    // println!("here {:?}", grid.get(1, 1));

    find_shortest_walk(&grid)
}

type Position = (usize, usize);
// key = posiition, heading, fwd_count
// value = lowest cost to get to that position
type DjikstraCache = HashMap<(Position, CardinalDir), BTreeMap<u8, u32>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new(t: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                elem: t,
                next: None,
            })),
        }
    }
    fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }
    fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CardinalDir {
    North,
    East,
    South,
    West,
}

impl CardinalDir {
    fn turn(&self, dir: Dir) -> CardinalDir {
        match (self, dir) {
            (_, Dir::Forward) => self.clone(),

            (CardinalDir::North, Dir::Left) => CardinalDir::West,
            (CardinalDir::East, Dir::Left) => CardinalDir::North,
            (CardinalDir::South, Dir::Left) => CardinalDir::East,
            (CardinalDir::West, Dir::Left) => CardinalDir::South,

            (CardinalDir::North, Dir::Right) => CardinalDir::East,
            (CardinalDir::East, Dir::Right) => CardinalDir::South,
            (CardinalDir::South, Dir::Right) => CardinalDir::West,
            (CardinalDir::West, Dir::Right) => CardinalDir::North,
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
    // pos_history: Vec<(usize, usize)>,
    pos_history: List<Position>,
    // curr_pos: Position,
    fwd_count: u8,
    grid: &'a Grid,
}

impl Crucible<'_> {
    fn get_current_pos(&self) -> &Position {
        self.pos_history.head().unwrap()
        // &self.curr_pos
    }

    fn has_been_at_current_pos(&self) -> bool {
        self.pos_history
            .tail()
            .iter()
            .any(|p| *p == *self.get_current_pos())
        // false
    }

    fn try_move(&mut self, dir: Dir) -> Result<(), &'static str> {
        match dir {
            Dir::Forward => {
                self.fwd_count += 1;
                if self.fwd_count >= 3 {
                    return Err("too many forward moves");
                }
            }
            _ => self.fwd_count = 0,
        }
        let next_step = self.heading.turn(dir);

        let Some((next_pos, next_cost)) = try_get_next_coords(self.get_current_pos(), &next_step)
            .and_then(|p| self.grid.get(p.0, p.1).map(|c| (p, c)))
        else {
            return Err("position out of bounds");
        };

        // self.pos_history.insert(0, next_pos);
        // self.curr_pos = next_pos;
        let prepended = self.pos_history.prepend(next_pos);
        self.pos_history = prepended;
        self.cost += next_cost;
        self.heading = next_step;

        Ok(())
    }
}

fn find_next_walk(
    mut crucible: Crucible,
    next_move: Dir,
    current_min: u32,
    cache: &mut DjikstraCache,
) -> Option<u32> {
    // println!("here");
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

    let inner_cache = cache
        .entry((*crucible.get_current_pos(), crucible.heading))
        .or_default();

    let is_all = inner_cache
        .range(0..=crucible.fwd_count)
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
        println!("found a solution: {:?}", crucible.cost);
        // println!("curr pos: {:?}", crucible.get_current_pos());
        // println!("curr pos: {:?}", crucible.pos_history);
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

// fn _find_baseline_walk(grid: &Grid) -> u32 {
//     let mut crucible = Crucible {
//         pos_history: vec![(0, 0)],
//         heading: CardinalDir::East,
//         cost: 0,
//         fwd_count: 0,
//         grid: &grid,
//     };

//     let height = grid.cells.len();
//     let width = grid.cells[0].len();

//     iter::repeat(Dir::Forward)
//         .take(width - 1)
//         .chain(iter::once(Dir::Right))
//         .chain(iter::repeat(Dir::Forward).take(1))
//         .for_each(|d| {
//             // dbg!(&crucible);
//             match crucible.try_move(d) {
//                 Err(e) => panic!("{}", e),
//                 _ => (),
//             };
//         });

//     crucible.cost
// }

fn find_shortest_walk(grid: &Grid) -> u32 {
    // let min_so_far = find_baseline_walk(grid);
    let min_so_far = 9999;

    let cache = &mut HashMap::new();

    let res = vec![
        (
            Dir::Forward,
            Crucible {
                // pos_history: vec![(0, 1)],
                pos_history: List::new((0, 1)),
                // curr_pos: (0, 1),
                heading: CardinalDir::South,
                cost: grid.get(0, 1).unwrap(),
                fwd_count: 0,
                grid: &grid,
            },
        ),
        (
            Dir::Right,
            Crucible {
                // pos_history: vec![(0, 1)],
                pos_history: List::new((0, 1)),
                // curr_pos: (0, 1),
                heading: CardinalDir::South,
                cost: grid.get(0, 1).unwrap(),
                fwd_count: 0,
                grid: &grid,
            },
        ),
        (
            Dir::Left,
            Crucible {
                // pos_history: vec![(1, 0)],
                pos_history: List::new((1, 0)),
                // curr_pos: (1, 0),
                heading: CardinalDir::East,
                cost: grid.get(1, 0).unwrap(),
                fwd_count: 0,
                grid: &grid,
            },
        ),
        (
            Dir::Forward,
            Crucible {
                // pos_history: vec![(1, 0)],
                pos_history: List::new((1, 0)),
                // curr_pos: (1, 0),
                heading: CardinalDir::East,
                cost: grid.get(1, 0).unwrap(),
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
    // let min = find_next_walk(
    //     Crucible {
    //         pos_history: vec![(0, 0)],
    //         heading: CardinalDir::East,
    //         cost: 0,
    //         fwd_count: 0,
    //         grid: &grid,
    //     },
    //     Dir::Forward,
    //     min_so_far,
    //     cache,
    // );

    // let next_min = find_next_walk(
    //     Crucible {
    //         pos_history: vec![(0, 0)],
    //         heading: CardinalDir::South,
    //         cost: 0,
    //         fwd_count: 0,
    //         grid: &grid,
    //     },
    //     Dir::Forward,
    //     min.unwrap(),
    //     cache,
    // );

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

    fn print(&self) {
        self.cells.iter().for_each(|row| {
            row.chars().for_each(|c| print!("{}", c));
            println!()
        });
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
