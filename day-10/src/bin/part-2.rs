use core::panic;
use std::collections::HashMap;

use glam::u32::UVec2;

fn main() {
    let input = include_str!("./input-1.txt");
    println!("Result: {:?}", part1(input));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Start,      // S
    NorthEast,  // L
    NorthSouth, // |
    NorthWest,  // J
    SouthEast,  // F
    SouthWest,  // 7
    EastWest,   // -
    None,       // .
}

impl Pipe {
    fn movable_by_dir(&self, dir: Direction) -> bool {
        if self == &Pipe::Start {
            return true;
        }
        match dir {
            Direction::North => match self {
                Pipe::SouthEast => true,
                Pipe::SouthWest => true,
                Pipe::NorthSouth => true,
                _ => false,
            },
            Direction::East => match self {
                Pipe::NorthWest => true,
                Pipe::SouthWest => true,
                Pipe::EastWest => true,
                _ => false,
            },
            Direction::South => match self {
                Pipe::NorthEast => true,
                Pipe::NorthSouth => true,
                Pipe::NorthWest => true,
                _ => false,
            },
            Direction::West => match self {
                Pipe::NorthEast => true,
                Pipe::SouthEast => true,
                Pipe::EastWest => true,
                _ => false,
            },
        }
    }

    fn next_move(&self, last_move: Direction) -> &Direction {
        match last_move {
            Direction::North => match self {
                Pipe::SouthEast => &Direction::East,
                Pipe::SouthWest => &Direction::West,
                Pipe::NorthSouth => &Direction::North,
                _ => panic!("Invalid move"),
            },
            Direction::East => match self {
                Pipe::NorthWest => &Direction::North,
                Pipe::SouthWest => &Direction::South,
                Pipe::EastWest => &Direction::East,
                _ => panic!("Invalid move"),
            },
            Direction::South => match self {
                Pipe::NorthEast => &Direction::East,
                Pipe::NorthSouth => &Direction::South,
                Pipe::NorthWest => &Direction::West,
                _ => panic!("Invalid move"),
            },
            Direction::West => match self {
                Pipe::NorthEast => &Direction::North,
                Pipe::SouthEast => &Direction::South,
                Pipe::EastWest => &Direction::West,
                _ => panic!("Invalid move"),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Sketch {
    grid: Vec<Vec<Pipe>>,
    start: UVec2,
}

#[derive(Debug)]
struct Window<'a> {
    north: &'a Pipe,
    south: &'a Pipe,
    east: &'a Pipe,
    west: &'a Pipe,
}

impl Sketch {
    fn get_curr_pipe(&self, pos: UVec2) -> &Pipe {
        &self.grid[pos.y as usize][pos.x as usize]
    }

    fn get_window(&self, pos: UVec2) -> Window {
        Window {
            north: match pos.y.checked_sub(1) {
                Some(new_y) => &self.grid[new_y as usize][pos.x as usize],
                None => &Pipe::None,
            },
            east: &self.safe_get_pos(pos.y as usize, pos.x as usize + 1),
            south: &self.safe_get_pos(pos.y as usize + 1, pos.x as usize),
            west: match pos.x.checked_sub(1) {
                Some(new_x) => &self.grid[pos.y as usize][new_x as usize],
                None => &Pipe::None,
            },
        }
    }

    fn safe_get_pos(&self, y: usize, x: usize) -> &Pipe {
        match self.grid.get(y).and_then(|row| row.get(x)) {
            Some(pipe) => pipe,
            None => &Pipe::None,
        }
    }

    fn print(&self) {
        let _ = &self.grid.iter().for_each(|row| {
            row.iter().for_each(|pipe| match pipe {
                Pipe::Start => print!("S "),
                Pipe::NorthEast => print!("└─"),
                Pipe::NorthSouth => print!("| "),
                Pipe::NorthWest => print!("┘ "),
                Pipe::SouthEast => print!("┌─"),
                Pipe::SouthWest => print!("┐ "),
                Pipe::EastWest => print!("──"),
                Pipe::None => print!(". "),
            });
            print!("\n");
        });
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Rotation {
    CW,
    CCW,
}

impl Sketch {
    fn is_start(&self) -> bool {
        self.grid[self.start.x as usize][self.start.y as usize] == Pipe::Start
    }
}

fn part1(input: &str) -> u32 {
    let mut curr_pos: UVec2 = UVec2 { x: 0, y: 0 };
    let grid = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    'L' => Pipe::NorthEast,
                    '|' => Pipe::NorthSouth,
                    'J' => Pipe::NorthWest,
                    'F' => Pipe::SouthEast,
                    '7' => Pipe::SouthWest,
                    '-' => Pipe::EastWest,
                    '.' => Pipe::None,
                    'S' => {
                        curr_pos = UVec2 {
                            x: j as u32,
                            y: i as u32,
                        };
                        Pipe::Start
                    }
                    x => panic!("Unknown character: {}", x),
                })
                .collect::<Vec<Pipe>>()
        })
        .collect::<Vec<Vec<Pipe>>>();

    let sketch = Sketch {
        grid,
        start: curr_pos,
    };

    let mut last_dir: Option<Direction> = None;

    let mut move_number: u32 = 0;

    let mut pos_history: HashMap<UVec2, (Direction, Pipe)> = HashMap::new();

    loop {
        use Direction::*;

        // get window
        let window = sketch.get_window(curr_pos);

        // see if there are valid moves (assume there is exactly one)
        // can move up?

        let curr_pipe = *sketch.get_curr_pipe(curr_pos);

        match last_dir {
            None => {
                if window.north.movable_by_dir(North) {
                    last_dir = Some(North);
                    pos_history.insert(curr_pos, (North, curr_pipe));
                    curr_pos.y -= 1;
                } else if window.east.movable_by_dir(East) {
                    last_dir = Some(East);
                    pos_history.insert(curr_pos, (East, curr_pipe));
                    curr_pos.x += 1;
                } else if window.south.movable_by_dir(South) {
                    last_dir = Some(South);
                    pos_history.insert(curr_pos, (South, curr_pipe));
                    curr_pos.y += 1;
                } else if window.west.movable_by_dir(West) {
                    last_dir = Some(West);
                    pos_history.insert(curr_pos, (West, curr_pipe));
                    curr_pos.x -= 1;
                } else {
                    panic!("No valid moves");
                }
            }
            Some(known_last_dir) => {
                let curr_pipe = sketch.get_curr_pipe(curr_pos);
                match curr_pipe.next_move(known_last_dir) {
                    North => {
                        last_dir = Some(North);
                        pos_history.insert(curr_pos, (North, *curr_pipe));
                        curr_pos.y -= 1;
                    }
                    East => {
                        last_dir = Some(East);
                        pos_history.insert(curr_pos, (East, *curr_pipe));
                        curr_pos.x += 1;
                    }
                    South => {
                        last_dir = Some(South);
                        pos_history.insert(curr_pos, (South, *curr_pipe));
                        curr_pos.y += 1;
                    }
                    West => {
                        last_dir = Some(West);
                        pos_history.insert(curr_pos, (West, *curr_pipe));
                        curr_pos.x -= 1;
                    }
                }
            }
        }

        move_number += 1;

        if sketch.start == curr_pos {
            break;
        }
    }

    let new_pos_history = pos_history;

    let res = (0..sketch.grid.len())
        .flat_map(|i| (0..sketch.grid[0].len()).map(move |j| (j, i)))
        .map(|(j, i)| {
            new_pos_history.get(&UVec2 {
                x: j as u32,
                y: i.clone() as u32,
            })
        })
        .collect::<Vec<_>>();
    let handedness = match res.iter().find_map(|x| match x {
        Some((Direction::South, _)) => Some(Direction::South),
        Some((x, _)) if (*x == Direction::East || *x == Direction::West) => Some(Direction::North),
        _ => None,
    }) {
        Some(Direction::South) => Rotation::CCW,
        Some(Direction::North) => Rotation::CW,
        _ => panic!("unknown"),
    };

    let len = sketch.grid.len();

    let ans = res
        .iter()
        .enumerate()
        .fold((false, 0), |acc: (bool, u32), (i, x)| {
            use Direction::*;
            use Pipe::*;
            use Rotation::*;
            // assume clockwise
            let ret = match (handedness, x) {
                (CW, Some((West, SouthWest))) => (true, acc.1),
                (CW, Some((North, NorthWest))) => (true, acc.1),
                (CW, Some((East, NorthEast))) => (false, acc.1),
                (CW, Some((South, SouthEast))) => (false, acc.1),
                (CW, Some((North, NorthSouth))) => (true, acc.1),
                (CW, Some((South, NorthSouth))) => (false, acc.1),

                (CCW, Some((South, SouthWest))) => (true, acc.1),
                (CCW, Some((West, NorthWest))) => (true, acc.1),
                (CCW, Some((North, NorthEast))) => (false, acc.1),
                (CCW, Some((East, SouthEast))) => (false, acc.1),
                (CCW, Some((South, NorthSouth))) => (true, acc.1),
                (CCW, Some((North, NorthSouth))) => (false, acc.1),

                _ => (acc.0, if acc.0 { acc.1 + 1 } else { acc.1 }),
            };
            println!("({:?}, {:?}), {:?}, {:?}", i % len, i / len, ret, x);
            ret
        });

    sketch.print();

    println!("ans: {:?}, handedness: {:?}", ans, handedness);
    ans.1
}

#[cfg(test)]
mod tests {
    use super::*;
    //     #[test]
    //     fn it_works() {
    //         let result = part1(
    //             ".....
    // .S-7.
    // .|.|.
    // .L-J.
    // .....",
    //         );
    //         assert_eq!(result, 4);
    //     }
    //     #[test]
    //     fn it_works_2() {
    //         let result = part1(
    //             "7-F7-
    // .FJ|7
    // SJLL7
    // |F--J
    // LJ.LJ",
    //         );
    //         assert_eq!(result, 8);
    //     }

    // pt 2 test cases
    #[test]
    fn it_works_3() {
        let result = part1(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        );
        assert_eq!(result, 10);
    }
    #[test]
    fn it_works_4() {
        let result = part1(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        );
        assert_eq!(result, 8);
    }
    #[test]
    fn it_works_5() {
        let result = part1(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works_mod_1() {
        let result = part1(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, 1);
    }
    #[test]
    fn it_works_mod_2() {
        let result = part1(
            "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        );
        assert_eq!(result, 1);
    }
}
