fn main() {
    let input = include_str!("./input-1.txt");
    println!("Result: {:?}", part1(input));
}

enum Loc {
    Ash,
    Rock,
}

impl TryFrom<&str> for Loc {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "#" => Ok(Loc::Rock),
            "." => Ok(Loc::Ash),
            _ => Err("Invalid location"),
        }
    }
}

fn is_line_symmetric_at_point(line: &str, pt: &usize) -> bool {
    (&line[0..*pt])
        .chars()
        .rev()
        .zip((&line[*pt..]).chars())
        .all(|(a, b)| a.eq(&b))
}

fn find_line_symmetry(line: &str) -> Vec<usize> {
    (1..line.len())
        .filter(|n| is_line_symmetric_at_point(line, n))
        .collect::<Vec<_>>()
}

fn find_area_vertical_symmetry(area: &str) -> Option<usize> {
    let (first, rest) = area.split_once("\n").unwrap();
    let symmetry = find_line_symmetry(first);
    let ans = symmetry.into_iter().find(|refl_pt| {
        rest.lines()
            .all(|line| is_line_symmetric_at_point(line, refl_pt))
    });
    ans
}

fn find_area_horizontal_symmetry(area: &str) -> Option<usize> {
    let line_count = area.lines().count();

    (1..line_count).find_map(|n| {
        match area
            .lines()
            .rev()
            .skip(line_count - n)
            .zip(area.lines().skip(n))
            .inspect(|(a, b)| {
                println!("{} | {}", a, b);
            })
            .all(|(a, b)| a.eq(b))
        {
            true => Some(n),
            false => None,
        }
    })
}

fn part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|area| {
            if let Some(ans) = find_area_horizontal_symmetry(area) {
                return ans * 100;
            }
            if let Some(ans) = find_area_vertical_symmetry(area) {
                return ans;
            }
            panic!("No symmetry found")
        })
        .sum::<usize>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, 405);
    }
    // #[test]
    fn it_works_hori() {
        let result = find_area_vertical_symmetry(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );
        assert_eq!(result, Some(5));
    }
    #[test]
    fn it_works_no_dect_hori() {
        let result = find_area_vertical_symmetry(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, None);
    }

    #[test]
    fn it_works_vert() {
        let result = find_area_horizontal_symmetry(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, Some(4));
    }
    #[test]
    fn it_works_no_dect_vert() {
        let result = find_area_horizontal_symmetry(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );
        assert_eq!(result, None);
    }
}
