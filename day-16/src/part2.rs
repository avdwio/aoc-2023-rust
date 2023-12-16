use std::{collections::BTreeSet, fmt::Display};

use itertools::Itertools;

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
    find_highest_energy_for_any_beam(input)
}

fn find_energy_for_base_beam(input: &str) -> u64 {
    let contraption = parse_into_contraption(input);

    let mut start_beam = Beam {
        position: (0, 0),
        direction: Direction::Right,
    };

    find_energy(&contraption, &mut start_beam)
}

fn find_highest_energy_for_any_beam(input: &str) -> u64 {
    let contraption = parse_into_contraption(input);

    iterate_beams(&contraption)
        .map(|mut beam| find_energy(&contraption, &mut beam))
        .max()
        .unwrap_or(0)
}

fn iterate_beams(contraption: &Contraption) -> impl Iterator<Item = Beam> {
    let width = contraption.value[0].len() as i64;
    let height = contraption.value.len() as i64;

    (0..width)
        .map(|x| Beam {
            position: (x, 0),
            direction: Direction::Down,
        })
        .chain((0..height).map(|y| Beam {
            position: (0, y),
            direction: Direction::Right,
        }))
        .chain((0..height).map(move |y| Beam {
            position: (width - 1, y),
            direction: Direction::Left,
        }))
        .chain((0..width).map(move |x| Beam {
            position: (x, height - 1),
            direction: Direction::Up,
        }))
}

fn find_energy(contraption: &Contraption, start_beam: &mut Beam) -> u64 {
    // initial spot:
    let maybe_next = start_beam.evaluate(contraption.get(start_beam.position).unwrap());

    let mut beams = vec![*start_beam];
    maybe_next.map(|x| beams.push(x));

    let mut visited = BTreeSet::<(Position, Direction)>::new();

    let mut i = 0;
    loop {
        let mut b = match beams.get(i) {
            Some(b_) => *b_,
            None => break,
        };
        loop {
            let true = visited.insert(((b.position.0, b.position.1), b.direction)) else {
                break;
            };
            b.step();
            let Some(spot) = contraption.get(b.position) else {
                break;
            };
            b.evaluate(spot).and_then(|new| Some(beams.push(new)));
        }
        i += 1;
    }
    visited.iter().unique_by(|(pos, _)| *pos).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = find_energy_for_base_beam(
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
    fn it_works_pt_2() {
        let result = find_highest_energy_for_any_beam(
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
        assert_eq!(result, 51);
    }

    #[test]
    fn test_loop() {
        let result = find_energy_for_base_beam(
            r".-..\.
......
.\../.",
        );
        assert_eq!(result, 11);
    }

    #[test]
    fn test_loop_2() {
        let result = find_energy_for_base_beam(
            r".\/.-.\
.\../..
..\.../",
        );
        assert_eq!(result, 17);
    }

    #[test]
    fn test_loop_3() {
        let result = find_energy_for_base_beam(
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
        let result = find_energy_for_base_beam(
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
        let result = find_energy_for_base_beam(
            r"..-\.
..\/.",
        );
        assert_eq!(result, 6);
    }

    #[test]
    fn test_loop_6() {
        let result = find_energy_for_base_beam(
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
        let result = find_energy_for_base_beam(
            r"\
.",
        );
        assert_eq!(result, 2);
    }
}
