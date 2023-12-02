fn main() {
    let input = include_str!("./input-1.txt");
    println!("Result: {:?}", part1(input));
}

fn part1(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(include_str!("./input-1-test.txt"));
        assert_eq!(result, Some(1));
    }
}
