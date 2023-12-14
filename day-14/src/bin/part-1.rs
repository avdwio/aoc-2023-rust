use std::iter;

fn main() {
    let input = include_str!("./input-1.txt");
    println!("Result: {:?}", part1(input));
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Space {
    Empty,
    Block,
    Round,
}

impl Space {
    fn to_string(&self) -> String {
        match self {
            Space::Empty => ".".to_string(),
            Space::Block => "#".to_string(),
            Space::Round => "O".to_string(),
        }
    }
}

impl TryFrom<char> for Space {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Space::Empty),
            '#' => Ok(Space::Block),
            'O' => Ok(Space::Round),
            _ => Err(()),
        }
    }
}

type Board = Vec<Vec<Space>>;

fn print_board(board: &Board) {
    println!("{}", board_to_string(board));
}

fn board_to_string(board: &Board) -> String {
    board
        .iter()
        .map(|line| {
            line.iter()
                .map(|space| space.to_string())
                .chain(std::iter::once("\n".to_string()))
                .collect::<String>()
        })
        .collect::<String>()
}

fn parse_to_board(input: &str) -> Board {
    input
        .lines()
        .map(|line| line.chars().map(|c| Space::try_from(c).unwrap()).collect())
        .collect()
}

fn calculate_load(board: &Board) -> u64 {
    let height = board.len();

    board
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let x = line
                .iter()
                .filter(|space| match space {
                    Space::Round => true,
                    _ => false,
                })
                .count()
                * (height - i);
            u64::try_from(x).unwrap()
        })
        .sum::<u64>()
}

enum Either {
    Left,
    Right,
}

fn rotate_board(board: &Board, either: Either) -> Board {
    let new_height = board[0].len();
    let new_width = board.len();

    let mut new_board: Board = iter::repeat(Vec::with_capacity(new_width))
        .take(new_height)
        .collect::<Vec<_>>();

    board.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, space)| match either {
            Either::Left => new_board[new_height - j - 1].push(*space),
            Either::Right => new_board[j].insert(0, *space),
        })
    });
    new_board
}

fn move_board(og_board: &Board) -> Board {
    og_board.iter().map(|line| line.clone()).collect()
}

fn print_line(line: &Vec<Space>) {
    line.iter()
        .for_each(|space| print!("{}", space.to_string()));
    println!();
}

fn tilt_left(board: &Board) -> Board {
    board
        .iter()
        .map(|line| {
            // println!("New Line");
            // print_line(line);
            let mut start = 0;
            let mut new_line: Vec<Space> = Vec::with_capacity(line.len());
            while start < line.len() {
                let (rounds, empties): (Vec<u32>, Vec<u32>) = line[start..]
                    .into_iter()
                    .take_while(|x| **x != Space::Block)
                    .map(|x| match x {
                        Space::Empty => (0, 1),
                        Space::Round => (1, 0),
                        Space::Block => panic!("Should not happen"),
                    })
                    .unzip::<_, _, _, _>();
                let num_rounds = rounds.iter().sum::<u32>();
                let num_empties = empties.iter().sum::<u32>();
                (0..num_rounds).for_each(|_| new_line.push(Space::Round));
                (0..num_empties).for_each(|_| new_line.push(Space::Empty));
                start += num_rounds as usize + num_empties as usize + 1;
                if start > line.len() {
                    break;
                }
                new_line.push(Space::Block);
            }
            // print_line(&new_line);
            // println!();
            new_line
        })
        .collect::<_>()
}

fn part1(input: &str) -> u64 {
    let board = parse_to_board(input);

    let rot_left = rotate_board(&board, Either::Left);
    // print_board(&rot_left);
    let tilted = tilt_left(&rot_left);
    // print_board(&tilted);
    let rot_right = rotate_board(&tilted, Either::Right);

    let load = calculate_load(&rot_right);

    load
}

#[cfg(test)]
mod tests {
    use super::*;
    const ORIGINAL_BOARD: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    const MOVED_BOARD: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    #[test]
    fn it_works() {
        let result = part1(include_str!("./input-1-test.txt"));
        assert_eq!(result, 136);
    }

    // #[test]
    // fn can_move_board() {
    //     let board = parse_to_board(ORIGINAL_BOARD);
    //     let rot_left = rotate_board(&board, Either::Left);
    //     let tilted = tilt_left(&rot_left);
    //     let rot_right = rotate_board(&tilted, Either::Right);

    //     let moved = parse_to_board(MOVED_BOARD);
    //     assert_eq!(rot_right, moved);
    // }

    #[test]
    fn test_calculate_load() {
        let result = parse_to_board(MOVED_BOARD);
        let load = calculate_load(&result);
        assert_eq!(load, 136);
    }
}
