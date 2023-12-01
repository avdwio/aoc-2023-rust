fn main() {
    let input = include_str!("./input-1.txt");
    let result = part2(input);
    dbg!(result);
}

fn part2(input: &str) -> u32 {
    let result = input
        .lines()
        .map(|calib| {
            let mut sliced = calib;
            let mut numbers = String::new();
            while !sliced.is_empty() {
                if sliced.starts_with("one") {
                    numbers.push('1');
                } else if sliced.starts_with("two") {
                    numbers.push('2');
                } else if sliced.starts_with("three") {
                    numbers.push('3');
                } else if sliced.starts_with("four") {
                    numbers.push('4');
                } else if sliced.starts_with("five") {
                    numbers.push('5');
                } else if sliced.starts_with("six") {
                    numbers.push('6');
                } else if sliced.starts_with("seven") {
                    numbers.push('7');
                } else if sliced.starts_with("eight") {
                    numbers.push('8');
                } else if sliced.starts_with("nine") {
                    numbers.push('9');
                } else {
                    let first_char = sliced.chars().next().unwrap();
                    if first_char.is_numeric() {
                        numbers.push(first_char);
                    }
                }
                sliced = &sliced[1..]
            }

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
        let result = part2(include_str!("./input-2-test.txt"));
        println!("{}", result.to_string());
        assert_eq!(result, 281);
    }
}
