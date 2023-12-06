fn main() {
    let input = include_str!("./input.txt");
    println!("Result: {:?}", process(input));
}

fn process(input: &str) -> u64 {
    let points = parse(input);
    let ans = calculate_times_range(&points);
    ans
}

fn parse(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let time_line = lines.next().unwrap();
    let distance_line = lines.next().unwrap();

    fn parse_line(line: &str) -> u64 {
        let (_, y) = line.split_once(":").unwrap();

        y.chars()
            .into_iter()
            .filter_map(|s| s.to_digit(10 as u32))
            .fold(0 as u64, |acc, x| acc * 10 + x as u64)
    }

    (parse_line(time_line), parse_line(distance_line))
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
        assert_eq!(result, 71503);
    }
}
