use core::panic;
use std::str::FromStr;

fn main() {
    let input = include_str!("./input-1.txt");
    let cubes_bag = Cubes {
        red: 12,
        blue: 14,
        green: 13,
    };
    println!("Result: {:?}", part1(input, &cubes_bag));
}

#[derive(Debug, Default, PartialEq)]
struct Cubes {
    red: u32,
    blue: u32,
    green: u32,
}

impl Cubes {
    fn expand(&mut self, cubes: &Cubes) {
        self.red = self.red.max(cubes.red);
        self.blue = self.blue.max(cubes.blue);
        self.green = self.green.max(cubes.green);
    }

    fn fits_into(&self, max: &Cubes) -> bool {
        return self.red <= max.red && self.blue <= max.blue && self.green <= max.green;
    }
}

impl FromStr for Cubes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes = Cubes::default();
        s.split(",").for_each(|x| {
            let mut split = x.trim().split(" ");
            let value = split.next().unwrap().parse::<u32>().unwrap();
            let color = split.next().unwrap();
            match color {
                "red" => cubes.red += value,
                "blue" => cubes.blue += value,
                "green" => cubes.green += value,
                _ => panic!("Unknown color"),
            }
        });
        return Ok(cubes);
    }
}

fn part1(input: &str, comp_cubes: &Cubes) -> Option<u32> {
    let ans = input
        .lines()
        .filter_map(|game| {
            // get the ID of the game
            let mut first_split = game.split(":");
            let id = first_split
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            // find the minimum possible cubes in bag

            let min_cubes = Cubes::default();

            let new_min_cubes =
                first_split
                    .next()
                    .unwrap()
                    .split(";")
                    .fold(min_cubes, |mut acc, _x| {
                        let this_round_cubes = Cubes::from_str(_x).unwrap();
                        acc.expand(&this_round_cubes);
                        acc
                    });
            if new_min_cubes.fits_into(&comp_cubes) {
                return Some(id);
            } else {
                return None;
            }
        }) // .filter(|x|);
        .sum::<u32>();
    return Some(ans);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let cubes_bag = Cubes {
            red: 12,
            blue: 14,
            green: 13,
        };
        let result = part1(include_str!("./input-1-test.txt"), &cubes_bag);
        assert_eq!(result, Some(8));
    }

    // fn test() {
    //     let data = " 2 red";
    //     dbg!(data.split(" "));
    //     assert_eq!(1, 2)
    // }

    #[test]
    fn can_create_cubes() {
        let cubes = Cubes::default();
        assert_eq!(
            Cubes {
                red: 0,
                blue: 0,
                green: 0
            },
            cubes
        );
    }

    #[test]
    fn can_expand_bag() {
        let mut cubes = Cubes {
            red: 4,
            blue: 1,
            green: 2,
        };
        let compare_cubes = Cubes {
            red: 1,
            blue: 5,
            green: 2,
        };
        cubes.expand(&compare_cubes);
        assert_eq!(
            cubes,
            Cubes {
                red: 4,
                blue: 5,
                green: 2
            }
        );
    }

    #[test]
    fn can_parse_str_to_cubes() {
        let cubes = Cubes::from_str("3 blue, 4 red");
        assert_eq!(
            cubes,
            Ok(Cubes {
                red: 4,
                blue: 3,
                green: 0
            })
        );
    }
}
