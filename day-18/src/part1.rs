pub fn process(input: &'static str) -> u64 {
    part1(input)
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
struct DigCommand {
    direction: Direction,
    distance: u8,
}

type DigBlueprint = Vec<DigCommand>;

fn parse_into_command(input: &'static str) -> DigCommand {
    let mut spl = input.split(" ");
    let direction = match spl.next() {
        Some("R") => Direction::Right,
        Some("L") => Direction::Left,
        Some("U") => Direction::Up,
        Some("D") => Direction::Down,
        _ => panic!("Invalid direction"),
    };
    let distance = spl.next().unwrap().parse::<u8>().unwrap();

    DigCommand {
        direction,
        distance,
    }
}

fn parse_input(input: &'static str) -> DigBlueprint {
    input
        .lines()
        .map(|line| parse_into_command(line.trim()))
        .collect::<Vec<_>>()
}

fn part1(input: &'static str) -> u64 {
    let instructions = parse_input(input);

    let mut border_count = 0;
    let mut x_val = 0;
    let mut nominal_area = 0;

    instructions.iter().for_each(|com| {
        border_count += com.distance as i32;
        match com.direction {
            Direction::Left => x_val -= com.distance as i32,
            Direction::Right => x_val += com.distance as i32,
            Direction::Up => nominal_area -= x_val * (com.distance as i32),
            Direction::Down => nominal_area += x_val * (com.distance as i32),
        }
    });
    (border_count / 2 + nominal_area + 1) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        );
        assert_eq!(result, 62);
    }
}
