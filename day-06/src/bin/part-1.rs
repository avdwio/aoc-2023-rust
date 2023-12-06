fn main() {
    let input = include_str!("./input.txt");
    println!("Result: {:?}", process(input));
}

fn process(input: &str) -> u64 {
    let points = parse(input);
    let ans = points
        .iter()
        .map(|x| calculate_times_range(x))
        .product::<u64>();
    ans
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines();
    let time_line = lines.next().unwrap();
    let distance_line = lines.next().unwrap();

    fn parse_line(line: &str) -> Vec<u64> {
        line.split(" ")
            .into_iter()
            .filter_map(|s| match s.parse::<u64>() {
                Ok(n) => Some(n),
                _ => None,
            })
            .collect::<Vec<_>>()
    }
    let time = parse_line(time_line);
    let distance = parse_line(distance_line);
    time.iter()
        .zip(distance.iter())
        .map(|(i, j)| (*i, *j))
        .collect::<Vec<_>>()
}

fn calculate_times_range(tuple: &(u64, u64)) -> u64 {
    let time = tuple.0;
    let dist = tuple.1;

    dbg!((0..time)
        .into_iter()
        .filter_map(|i| {
            let res = (time - i) * i;
            match res > dist {
                true => Some(res),
                false => None,
            }
        })
        .count())
    .try_into()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = process(include_str!("./input-test.txt"));
        assert_eq!(result, 288);
    }
}
