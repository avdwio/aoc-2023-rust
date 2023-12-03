fn main() {
    let input = include_str!("./input-1.txt");
    println!("Result: {:?}", part1(input));
}

fn part1(input: &str) -> u64 {
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let mut current_number: Option<PartNumber> = None;

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            match char {
                // if .
                '.' => {
                    if current_number.is_some() {
                        part_numbers.push(current_number.unwrap());
                        current_number = None;
                    }
                }
                // if numeric
                y if y.is_numeric() => match current_number {
                    Some(mut part_num) => {
                        part_num.number = part_num.number * 10 + char.to_digit(10).unwrap();
                        current_number = Some(part_num);
                    }
                    None => {
                        current_number = Some(PartNumber {
                            number: char.to_digit(10).unwrap(),
                            position: (
                                u32::try_from(j).unwrap() + 1,
                                u32::try_from(i).unwrap() + 1,
                            ),
                        })
                    }
                },
                // if symbol
                _ => {
                    symbols.push(Symbol {
                        symbol: char,
                        position: (u32::try_from(j).unwrap() + 1, u32::try_from(i).unwrap() + 1),
                    });
                    if current_number.is_some() {
                        part_numbers.push(current_number.unwrap());
                        current_number = None;
                    }
                }
            }
        }

        if current_number.is_some() {
            part_numbers.push(current_number.unwrap());
            current_number = None;
        }
    }

    let res = part_numbers
        .into_iter()
        .filter_map(|part_num| {
            let x = symbols
                .iter()
                .any(|symbol| is_valid_part_number(&part_num, symbol));
            match x {
                true => Some(part_num.number),
                false => None,
            }
        })
        .map(|x| u64::from(x))
        .inspect(|x| println!("WHAT?! {}", x))
        .collect::<Vec<u64>>();

    dbg!(&res.len(), res.iter().sum::<u64>());
    res.iter().sum()
    // .for_each(|x| {
    //     println!("{}", x);
    // })
}

#[derive(Debug)]
struct PartNumber {
    number: u32,
    position: (u32, u32),
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    position: (u32, u32),
}

fn is_valid_part_number(part_number: &PartNumber, symbol: &Symbol) -> bool {
    let digit_count: u32 = part_number.number.to_string().len().try_into().unwrap();
    let x_range = (part_number.position.0 - 1)..(part_number.position.0 + digit_count + 1);
    let y_range = (part_number.position.1 - 1)..(part_number.position.1 + 2);

    if (x_range.contains(&symbol.position.0)) && (y_range.contains(&symbol.position.1)) {
        // dbg!(&x_range, &y_range, &part_number, &symbol);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn it_works() {
        let result = part1(include_str!("./input-1-test.txt"));
        assert_eq!(result, 4361);
    }

    fn gets_correct_number_of_part_numbers() {
        let input = include_str!("./input-1-test.txt");
        let result = part1(input);
        assert_eq!(result, 4361);
    }

    #[rstest]
    fn check_is_valid_part_number() {
        let part_number = PartNumber {
            number: 234,
            position: (1, 1),
        };
        let symbol = Symbol {
            symbol: 'a',
            position: (4, 2),
        };
        assert_eq!(is_valid_part_number(&part_number, &symbol), true);
    }

    #[rstest]
    fn range_test() {
        assert!((0..2).contains(&1))
    }
}
