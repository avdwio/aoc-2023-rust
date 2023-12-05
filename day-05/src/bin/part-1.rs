use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    println!("Result: {:?}", process(input));
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
        dbg!(self.from, self.range, self.to, value);
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

fn seeds_parser(input: &str) -> Option<Vec<u64>> {
    Some(
        input
            .split(": ")
            .last()
            .unwrap()
            .split(" ")
            .map(|x| x.parse().unwrap())
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

fn process1(input: &str) -> Vec<u64> {
    let mut iter = input.split("\n\n");

    let seeds = seeds_parser(iter.next().unwrap()).unwrap();

    iter.map(|x| category_mapper_stage_parser(x).unwrap())
        .fold(seeds, |acc, stage| {
            acc.iter().map(|y| stage.convert(*y)).collect::<Vec<_>>()
        })
}

fn process(input: &str) -> u64 {
    *process1(input).iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = process1(include_str!("./input-test.txt"));
        assert_eq!(result, vec![82, 43, 86, 35]);
    }

    #[test]
    fn test_seeds_parser() {
        let result = seeds_parser("seeds: 79 14 55 13");
        assert_eq!(result, Some(vec![79, 14, 55, 13]));
    }

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
