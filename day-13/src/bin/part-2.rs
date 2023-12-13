use std::iter;

fn main() {
    let input = include_str!("./input-1.txt");
    println!("Result: {:?}", part1(input));
    println!("Result: {:?}", part2(input));
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Symmetry {
    Perfect = 0,
    Imperfect = 1,
    Asymmetric = 2,
}

impl std::ops::Add for Symmetry {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match self as u8 + rhs as u8 {
            0 => Self::Perfect,
            1 => Self::Imperfect,
            _ => Self::Asymmetric,
        }
    }
}

impl iter::Sum for Symmetry {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::Perfect, |acc, x| acc + x)
    }
}

fn is_match(a: &char, b: &char) -> Symmetry {
    match a.eq(b) {
        true => Symmetry::Perfect,
        false => Symmetry::Imperfect,
    }
}

fn is_line_partially_symmetric_at_point(line: &str, pt: &usize) -> Symmetry {
    (&line[0..*pt])
        .chars()
        .rev()
        .zip((&line[*pt..]).chars())
        .map(|(a, b)| is_match(&a, &b))
        .sum::<Symmetry>()
}

fn find_line_partial_symmetry(line: &str) -> Vec<(usize, Symmetry)> {
    (1..line.len())
        .filter_map(|n| match is_line_partially_symmetric_at_point(line, &n) {
            Symmetry::Asymmetric => None,
            x => Some((n, x)),
        })
        .collect::<Vec<_>>()
}

fn find_area_partial_vertical_symmetry(area: &str) -> Vec<(usize, Symmetry)> {
    let (first, rest) = area.split_once("\n").unwrap();
    let symmetry = find_line_partial_symmetry(first);
    // println!("Symm: {:?}", &symmetry);
    let ans = symmetry
        .into_iter()
        .filter_map(|(refl_pt, init_sym)| {
            let sym = rest.lines().try_fold(init_sym, |acc, line| {
                match is_line_partially_symmetric_at_point(line, &refl_pt) + acc {
                    Symmetry::Asymmetric => return None,
                    n => Some(n),
                }
            });
            // println!("sym: {:?}, refl_pt: {:?}", sym, refl_pt);
            match sym {
                Some(sym) => Some((refl_pt, sym)),
                None => None,
            }
        })
        .collect();
    ans
}

fn find_area_partial_horizontal_symmetry(area: &str) -> Vec<(usize, Symmetry)> {
    let line_count = area.lines().count();

    let last = (1..line_count)
        .filter_map(|n| {
            let x = area
                .lines()
                .rev()
                .skip(line_count - n)
                .zip(area.lines().skip(n))
                .map(|(a, b)| {
                    a.chars()
                        .zip(b.chars())
                        .map(|(a, b)| is_match(&a, &b))
                        .sum::<Symmetry>()
                })
                .sum::<Symmetry>();

            match x {
                Symmetry::Asymmetric => None,
                r => Some((n, r)),
            }
        })
        .collect::<Vec<_>>();
    last
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

fn part2(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|area| {
            find_area_partial_horizontal_symmetry(area)
                .iter()
                .filter_map(|(n, sym)| match *sym {
                    Symmetry::Imperfect => return Some(*n * 100),
                    _ => None,
                })
                .chain(
                    find_area_partial_vertical_symmetry(area)
                        .iter()
                        .filter_map(|(n, sym)| match *sym {
                            Symmetry::Imperfect => return Some(*n),
                            _ => None,
                        }),
                )
                .sum::<usize>()
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
    #[test]
    fn it_works_vert() {
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
    fn it_works_no_dect_vert() {
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
    fn it_works_hori() {
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
    fn it_works_no_dect_hori() {
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

    // part 2
    #[test]
    fn it_works_2() {
        let result = part2(
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
        assert_eq!(result, 400);
    }
    #[test]
    fn it_works_vert_partial() {
        // I modified example to get this to work
        let compvec = vec![(5 as usize, Symmetry::Imperfect)];
        let result = find_area_partial_vertical_symmetry(
            "#.##..##.
..#.#..#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );
        assert_eq!(result, compvec);
    }
    #[test]
    fn it_works_hori_partial_1() {
        let compvec = vec![(3 as usize, Symmetry::Imperfect)];
        let result = find_area_partial_horizontal_symmetry(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );
        assert_eq!(result, compvec);
    }

    #[test]
    fn it_works_hori_partial_2() {
        let compvec = vec![
            (1 as usize, Symmetry::Imperfect),
            (4 as usize, Symmetry::Perfect),
        ];
        let result = find_area_partial_horizontal_symmetry(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, compvec);
    }
    #[test]
    fn adding_syms() {
        use Symmetry::*;
        assert_eq!(Perfect + Perfect, Perfect);
        assert_eq!(Perfect + Imperfect, Imperfect);
        assert_eq!(Perfect + Asymmetric, Asymmetric);

        assert_eq!(Imperfect + Perfect, Imperfect);
        assert_eq!(Imperfect + Imperfect, Asymmetric);
        assert_eq!(Imperfect + Asymmetric, Asymmetric);

        assert_eq!(Asymmetric + Perfect, Asymmetric);
        assert_eq!(Asymmetric + Imperfect, Asymmetric);
        assert_eq!(Asymmetric + Asymmetric, Asymmetric);
    }
}
