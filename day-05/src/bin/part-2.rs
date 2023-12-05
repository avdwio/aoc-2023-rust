use core::ops::Range;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    println!("Result: {:?}", process(input));
}

#[derive(Debug)]
struct SeedRange {
    start: u64,
    range: u64,
}

impl SeedRange {
    fn get_range_iter(&self) -> Range<u64> {
        let x = (self.start..(self.start + self.range)).into_iter();
        x
    }
}

#[derive(Debug, PartialEq)]
struct CategoryMapper {
    to: u64,
    from: u64,
    range: u64,
}

impl From<(u64, u64, u64)> for CategoryMapper {
    fn from(value: (u64, u64, u64)) -> Self {
        CategoryMapper::new(value.0, value.1, value.2)
    }
}

impl CategoryMapper {
    fn new(to: u64, from: u64, range: u64) -> Self {
        Self { from, to, range }
    }

    fn convert(&self, value: u64) -> Option<u64> {
        match value {
            x if x >= self.from && x < self.from + self.range => Some(x + self.to - self.from),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct CategoryMapperStage {
    items: Vec<CategoryMapper>,
}

impl CategoryMapperStage {
    fn new(items: Vec<CategoryMapper>) -> Self {
        Self { items }
    }

    fn convert(&self, value: u64) -> u64 {
        match &self.items.iter().find_map(|x| x.convert(value)) {
            Some(x) => *x,
            None => value,
        }
    }
}

fn seeds_parser(input: &str) -> Option<Vec<SeedRange>> {
    Some(
        input
            .split(": ")
            .last()
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|i| SeedRange {
                start: i[0],
                range: i[1],
            })
            .collect::<Vec<_>>(),
    )
}

fn category_mapper_stage_parser(input: &str) -> Option<CategoryMapperStage> {
    let mut it = input.lines();
    it.next();
    Some(CategoryMapperStage {
        items: it
            .map(|x| {
                CategoryMapper::from(
                    x.split(" ")
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect_tuple::<(u64, u64, u64)>()
                        .unwrap(),
                )
            })
            .collect::<Vec<CategoryMapper>>(),
    })
}

fn process1(input: &str) -> u64 {
    let mut iter = input.split("\n\n");

    let seed_ranges = seeds_parser(iter.next().unwrap()).unwrap();

    dbg!(&seed_ranges);

    let stage_vec = iter
        .map(|x| category_mapper_stage_parser(x).unwrap())
        .collect::<Vec<_>>();

    let n = seed_ranges
        .iter()
        .flat_map(|seed_range| seed_range.get_range_iter())
        .map(|seed| {
            stage_vec.iter().fold(seed, |acc, stage| {
                let m = stage.convert(acc);
                m
            })
        })
        .enumerate()
        .inspect(|(x, y)| {
            if x % 100000 == 0 {
                println!("steps: {}, {}", x / 100000, y);
            }
        })
        .map(|(_, x)| x)
        .min();

    n.unwrap()
}

fn process(input: &str) -> u64 {
    process1(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = process1(include_str!("./input-test.txt"));
        assert_eq!(result, 27);
    }

    // #[test]
    // fn test_seeds_parser() {
    //     let result = seeds_parser("seeds: 79 14 55 13");
    //     assert_eq!(result, Some(vec![79, 14, 55, 13]));
    // }

    #[test]
    fn test_category_mapper_parser() {
        let result = category_mapper_stage_parser(
            "seed-to-soil map:
50 98 2
52 50 48",
        )
        .unwrap();
        let ans = CategoryMapperStage::new(vec![
            CategoryMapper::new(50, 98, 2),
            CategoryMapper::new(52, 50, 48),
        ]);
        assert_eq!(result, ans);
    }

    #[test]
    fn test_category_mapper_stage_convert() {
        let seed_to_soil = CategoryMapperStage::new(vec![
            CategoryMapper::new(50, 98, 2),
            CategoryMapper::new(52, 50, 48),
        ]);

        let ans = vec![79, 14, 55, 13]
            .iter()
            .map(|x| seed_to_soil.convert(*x))
            .collect::<Vec<_>>();

        assert_eq!(vec![81, 14, 57, 13], ans);
    }
}
