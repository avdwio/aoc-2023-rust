use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Display,
};

use itertools::Itertools;

use colored::Colorize;
use std::io::{stdin, stdout, Read, Write};

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

enum Spot {
    Empty,         // .
    BackMirror,    // \
    ForwardMirror, // /
    SplitVert,     // |
    SplitHori,     // -
}

impl TryFrom<char> for Spot {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Spot::Empty),
            '\\' => Ok(Spot::BackMirror),
            '/' => Ok(Spot::ForwardMirror),
            '|' => Ok(Spot::SplitVert),
            '-' => Ok(Spot::SplitHori),
            _ => Err("Invalid spot"),
        }
    }
}

struct Contraption {
    value: Vec<Vec<Spot>>,
}

type Position = (i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State(Position, Direction);

impl Contraption {
    fn get(&self, position: Position) -> Option<&Spot> {
        let (x, y) = position;
        self.value
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
    }
}

impl Display for Contraption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for row in &self.value {
            for spot in row {
                result.push(match spot {
                    Spot::Empty => '.',
                    Spot::BackMirror => '\\',
                    Spot::ForwardMirror => '/',
                    Spot::SplitVert => '|',
                    Spot::SplitHori => '-',
                });
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

impl Contraption {
    fn print_colorful(&self, energized: &BTreeSet<(Position, Direction)>, beams: Option<&[Beam]>) {
        use Direction::*;

        let beam_locs =
            beams.and_then(|x| Some(x.iter().map(|b| b.position).collect::<HashSet<_>>()));

        let mut result = String::new();
        for (y, row) in self.value.iter().enumerate() {
            for (x, spot) in row.iter().enumerate() {
                // if energized.contains(&(x as i64, y as i64)) {
                //     result.push_str("\x1b[0;31m");
                // }

                let beam = beam_locs
                    .as_ref()
                    .and_then(|n| Some(n.contains(&(x as i64, y as i64))))
                    .unwrap_or(false);

                let c = match spot {
                    Spot::Empty => ".",
                    Spot::BackMirror => "\\",
                    Spot::ForwardMirror => "/",
                    Spot::SplitVert => "|",
                    Spot::SplitHori => "-",
                };

                let contains = energized
                    .range(((x as i64, y as i64), Up)..=((x as i64, y as i64), Right))
                    .count();

                let s = match (beam, contains) {
                    (true, _) => c.red().on_green().to_string(),
                    (_, 4) => c.red().on_yellow().to_string(),
                    (_, 3) => c.red().on_blue().to_string(),
                    (_, 2) => c.red().on_bright_blue().to_string(),
                    (_, 1) => c.red().on_white().to_string(),
                    _ => String::from(c),
                };

                result.push_str(&s);
                // if energized.contains(&(x as i64, y as i64)) {
                //     result.push_str("\x1b[0m");
                // }
            }
            result.push('\n');
        }
        println!("{}", result);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Beam {
    position: Position,
    direction: Direction,
}

impl Beam {
    fn step(&mut self) {
        match self.direction {
            Direction::Up => self.position.1 -= 1,
            Direction::Down => self.position.1 += 1,
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
        }
    }

    fn evaluate(&mut self, spot: &Spot) -> Option<Beam> {
        match spot {
            Spot::Empty => {}
            Spot::BackMirror => match self.direction {
                Direction::Up => self.direction = Direction::Left,
                Direction::Left => self.direction = Direction::Up,
                Direction::Down => self.direction = Direction::Right,
                Direction::Right => self.direction = Direction::Down,
            },
            Spot::ForwardMirror => match self.direction {
                Direction::Up => self.direction = Direction::Right,
                Direction::Right => self.direction = Direction::Up,
                Direction::Down => self.direction = Direction::Left,
                Direction::Left => self.direction = Direction::Down,
            },
            Spot::SplitVert => match self.direction {
                Direction::Left | Direction::Right => {
                    let mut new_beam = self.clone();
                    self.direction = Direction::Down; // << ---
                    new_beam.direction = Direction::Up;
                    return Some(new_beam);
                }
                _ => {}
            },
            Spot::SplitHori => match self.direction {
                Direction::Up | Direction::Down => {
                    let mut new_beam = self.clone();
                    self.direction = Direction::Left; // << ---
                    new_beam.direction = Direction::Right;
                    return Some(new_beam);
                }
                _ => {}
            },
        };
        None
    }
    fn energize(&self) -> ! {
        todo!("Implement energize")
    }
}

fn parse_into_contraption(input: &str) -> Contraption {
    Contraption {
        value: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Spot::try_from(c).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    }
}

pub fn process(input: &str) -> u64 {
    let contraption = parse_into_contraption(input);
    println!("{}", contraption);

    let start_beam = Beam {
        position: (0, 0),
        direction: Direction::Right,
    };

    let mut beams = vec![start_beam];

    let mut visited = BTreeSet::<(Position, Direction)>::new();

    let mut i = 0;
    loop {
        // extract the next beam
        // if there is no next beam, we are done
        let mut b = match beams.get(i) {
            Some(b_) => *b_,
            None => break,
        };
        loop {
            // if we have been here before, done with this beam
            match visited.insert(((b.position.0, b.position.1), b.direction)) {
                false => {
                    // println!(" >>> been here before");
                    break;
                }
                true => {}
            }

            // increment the beam
            b.step();
            // find next spot. if no next spot exists, done with this beam
            let spot = match contraption.get(b.position) {
                Some(spot) => spot,
                None => {
                    // println!(" >>> beam out of bounds");
                    break;
                }
            };
            // decide how the beam rotates with given spot
            // if the spot returns a new beam, add it to the list
            match b.evaluate(spot) {
                Some(new_beam) => {
                    // println!(" >>> beam added");
                    beams.push(new_beam);
                }
                None => {}
            };
            // println!("{:?}", &b);
            // contraption.print_colorful(&energized);
        }
        // println!(" >>> beam done: {:?}", i);
        // pause();
        let x = &beams[(i)..];
        contraption.print_colorful(&visited, Some(&beams[(i + 1)..]));

        i += 1;
    }

    // println!("{:?}", energized);
    println!("{i}, #beams: {}", beams.len());
    println!(
        "{} x {} = {}",
        contraption.value[0].len(),
        contraption.value.len(),
        contraption.value[0].len() * contraption.value.len()
    );

    contraption.print_colorful(&visited, None);

    visited.iter().unique_by(|(pos, _)| *pos).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        assert_eq!(result, 46);
    }

    #[test]
    fn test_loop() {
        let result = process(
            r".-..\.
......
.\../.",
        );
        assert_eq!(result, 11);
    }

    #[test]
    fn test_loop_2() {
        let result = process(
            r".\/.-.\
.\../..
..\.../",
        );
        assert_eq!(result, 17);
    }

    #[test]
    fn test_loop_3() {
        let result = process(
            r".\/.-..
.......
.\../..
.......
..\....",
        );
        assert_eq!(result, 20);
    }

    #[test]
    fn test_loop_4() {
        let result = process(
            r".\...
.....
.\..|
.....
..\./",
        );
        assert_eq!(result, 16);
    }

    #[test]
    fn test_loop_5() {
        let result = process(
            r"..-\.
..\/.",
        );
        assert_eq!(result, 6);
    }

    #[test]
    fn test_loop_6() {
        let result = process(
            r".\/.\..
.......
.\|....
..../..
..\./..",
        );
        assert_eq!(result, 21);
    }

    #[test]
    fn test_loop_7() {
        let result = process(
            r"\
.",
        );
        assert_eq!(result, 2);
    }
}
