use glam::u32::UVec2;

fn main() {
    let input = include_str!("./input-1.txt");
    println!("Result: {:?}", part1(input));
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
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
        println!("Getting window for pos: {}", pos);
        println!(
            "Curr pipe: {:?}, East: ",
            &self.grid[pos.y as usize][pos.x as usize]
        );

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
                    _ => panic!("Unknown character"),
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

    loop {
        println!("Move number: {}, loc: {}", move_number, curr_pos);
        use Direction::*;

        // get window
        let window = sketch.get_window(curr_pos);

        dbg!(&window);
        // see if there are valid moves (assume there is exactly one)
        // can move up?

        match last_dir {
            None => {
                if window.north.movable_by_dir(North) {
                    println!("Moving up");
                    curr_pos.y -= 1;
                    last_dir = Some(North);
                } else if window.east.movable_by_dir(East) {
                    println!("Moving right");
                    curr_pos.x += 1;
                    last_dir = Some(East);
                } else if window.south.movable_by_dir(South) {
                    println!("Moving down");

                    curr_pos.y += 1;
                    last_dir = Some(South);
                } else if window.west.movable_by_dir(West) {
                    println!("Moving left");
                    curr_pos.x -= 1;
                    last_dir = Some(West);
                } else {
                    panic!("No valid moves");
                }
            }
            Some(known_last_dir) => {
                let curr_pipe = sketch.get_curr_pipe(curr_pos);
                match curr_pipe.next_move(known_last_dir) {
                    North => {
                        println!("Moving up");
                        curr_pos.y -= 1;
                        last_dir = Some(North);
                    }
                    East => {
                        println!("Moving right");
                        curr_pos.x += 1;
                        last_dir = Some(East);
                    }
                    South => {
                        println!("Moving down");
                        curr_pos.y += 1;
                        last_dir = Some(South);
                    }
                    West => {
                        println!("Moving left");
                        curr_pos.x -= 1;
                        last_dir = Some(West);
                    }
                }
            }
        }

        move_number += 1;

        if sketch.start == curr_pos {
            break;
        }
    }
    move_number / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works_2() {
        let result = part1(
            "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        );
        assert_eq!(result, 8);
    }
}
