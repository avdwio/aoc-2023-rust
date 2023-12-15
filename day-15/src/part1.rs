pub fn process(input: &str) -> u64 {
    input.split(",").map(|x| get_hash(x)).sum::<_>()
}

fn get_hash(input: &str) -> u64 {
    input.chars().fold(0, |acc, c| {
        let mut new = acc + c as u64;
        new *= 17;
        new = new % 256;
        new
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn it_works() {
        let result = process("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, 1320);
    }

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_hash(#[case] input: &str, #[case] expected: u64) {
        let result = get_hash(input);
        assert_eq!(result, expected);
    }
}
