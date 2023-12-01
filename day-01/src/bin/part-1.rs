fn main() {
    let input = include_str!("./input-1.txt");
    let result = part1(input);
    dbg!(result);
}

fn part1(input: &str) -> u32 {
    let result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(include_str!("./input-1-test.txt"));
        assert_eq!(result, 24000);
    }
}
