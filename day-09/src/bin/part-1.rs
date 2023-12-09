fn main() {
    let input = include_str!("./input-1.txt");
    println!("Result: {:?}", part1(input));
}

fn find_vec_prime(input: &Vec<i32>) -> Vec<i32> {
    input
        .iter()
        .zip(input[1..].iter())
        .map(|(x, y)| y - x)
        .collect::<Vec<_>>()
}

fn find_last_el(input: &Vec<i32>) -> i32 {
    println!("input: {:?}", input);
    // are all elements 0?
    let all_els_0 = input.iter().all(|&x| x == 0);
    let prev_el = if all_els_0 {
        println!("all els 0");
        0
    } else {
        println!("not root vec");
        // get next el of prime
        let vec_prime = find_vec_prime(input);
        let last_el_prime = find_last_el(&vec_prime);
        let prev_el = input.first().unwrap() - last_el_prime;
        println!(
            "prev_el =  {:?} - {:?} = {:?}",
            input.first().unwrap(),
            last_el_prime,
            prev_el
        );
        prev_el
    };
    prev_el
}

fn part1(input: &str) -> i32 {
    let result = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            println!("line: {:?}", i);
            let ints = line
                .split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            find_last_el(&ints)
        })
        .inspect(|x| println!("x: {:?}\n", x))
        .sum();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(include_str!("./input-1-test.txt"));
        assert_eq!(result, 2);
    }
}
