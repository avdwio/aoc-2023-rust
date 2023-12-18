pub fn process(input: &'static str) -> u64 {
    part1(input)
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct DigCommand {
    direction: Direction,
    distance: i128,
}

struct DigBlueprint {
    commands: Vec<DigCommand>,
}

fn parse_into_command(input: &'static str) -> DigCommand {
    let mut spl = input.split(" ");
    let _ = spl.next();
    let _ = spl.next();
    let rest = spl
        .next()
        .unwrap()
        .split('#')
        .nth(1)
        .unwrap()
        .split(')')
        .next()
        .unwrap();

    let len = rest.len();

    DigCommand {
        direction: match &rest[len - 1..] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("unknown direction"),
        },
        distance: i128::from_str_radix(&rest[0..(len - 1)], 16).unwrap(),
    }
}

fn parse_input(input: &'static str) -> DigBlueprint {
    DigBlueprint {
        commands: input
            .lines()
            .map(|line| parse_into_command(line.trim()))
            .collect::<Vec<_>>(),
    }
}

fn part1(input: &'static str) -> u64 {
    let instructions = parse_input(input);

    let mut border_count = 0;
    let mut x_val = 0;
    let mut nominal_area = 0;

    instructions.commands.iter().for_each(|com| {
        border_count += com.distance;
        match com.direction {
            Direction::Left => x_val -= com.distance as i128,
            Direction::Right => x_val += com.distance as i128,
            Direction::Up => nominal_area -= x_val * (com.distance as i128),
            Direction::Down => nominal_area += x_val * (com.distance as i128),
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
        assert_eq!(result, 952408144115);
    }
}
