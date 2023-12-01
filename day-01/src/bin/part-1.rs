fn main() {
    let input = include_str!("./input-1.txt");
    let result = part1(input);
    dbg!(result);
}

fn part1(input: &str) -> u32 {
    let result = input
        .lines()
        .map(|calib| {
            let numbers: String = calib
                .split("")
                .filter(|char| match *char {
                    "0" => true,
                    "1" => true,
                    "2" => true,
                    "3" => true,
                    "4" => true,
                    "5" => true,
                    "6" => true,
                    "7" => true,
                    "8" => true,
                    "9" => true,
                    _ => false,
                })
                .collect();
            let firstnum = numbers.chars().next().unwrap();
            let lastnum = numbers.chars().last().unwrap();

            let mut number = String::new();
            number.push(firstnum);
            number.push(lastnum);
            return number.parse::<u32>().unwrap();
        })
        .sum();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(include_str!("./input-1-test.txt"));
        println!("{}", result.to_string());
        assert_eq!(result, 142);
    }
}
