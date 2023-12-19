use nom::bytes::complete::{tag, take_until};
use nom::character::complete::alpha1;
use nom::sequence::tuple;
use nom::IResult;
use std::cmp::{max, min};
use std::collections::HashMap;

pub fn process(input: &'static str) -> u64 {
    let (instr_block, parts_block) = input.split_once("\n\n").unwrap();

    let map = instr_block
        .lines()
        .map(|x| parse_workflow(x).unwrap().1)
        .collect::<HashMap<_, _>>();

    let parts = parts_block.lines().map(parse_part).collect::<Vec<_>>();

    let mut parts2 = vec![("in", vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)])];

    let mut success_vec = Vec::<RangedPart>::new();

    while let Some((starting_instr, starting_part)) = parts2.pop() {
        let mut curr_instr = starting_instr;
        let steps = map.get(curr_instr).unwrap();

        steps.iter().try_fold(starting_part, |mut part, step| {
            let do_action = if let Some(rule) = &step.check {
                println!("part: {:?}", part);
                let part_range = part[rule.part];

                let (within, without) = bisect_range(part_range, rule.compare, &rule.operation);

                let mut new_part = part.clone();

                (
                    within.map(|x| {
                        new_part[rule.part] = x;
                        new_part
                    }),
                    without.map(move |x| {
                        part[rule.part] = x;
                        part
                    }),
                )
            } else {
                (Some(part), None)
            };
            if let Some(x) = do_action.0 {
                match &step.pipe_to {
                    PipeTo::Next(next) => {
                        println!("deferring to \"{}\": {:?}", next, x);
                        parts2.push((next, x));
                    }
                    PipeTo::Final(action) => match action {
                        FinalAction::Accept => {
                            println!("accept: {:?}", x);
                            success_vec.push(x);
                        }
                        FinalAction::Reject => {
                            println!("reject: {:?}", x);
                            // do nothing;
                        }
                    },
                }
            }
            do_action.1
        });
    }

    success_vec.iter().map(range_part_count).sum()
}

type RangePart = Vec<Range>;

fn range_part_count(range: &Vec<Range>) -> u64 {
    range.iter().map(|x| x.1 - x.0 + 1).product()
}

// {x=787,m=2655,a=1222,s=2876}

fn parse_part(input: &str) -> Part {
    let (_, input) = input.split_once("{").unwrap();
    let (input, _) = input.split_once("}").unwrap();
    let (_, input) = input.split_once("=").unwrap();
    let (x, input) = input.split_once(",").unwrap();
    let (_, input) = input.split_once("=").unwrap();
    let (m, input) = input.split_once(",").unwrap();
    let (_, input) = input.split_once("=").unwrap();
    let (a, input) = input.split_once(",").unwrap();
    let (_, s) = input.split_once("=").unwrap();

    Part {
        x: x.parse().unwrap(),
        m: m.parse().unwrap(),
        a: a.parse().unwrap(),
        s: s.parse().unwrap(),
    }
}

// a<2006:qkq,m>2090:A,rfg

fn parse_instruction(r_s: &'static str) -> IResult<&str, WorkflowStep> {
    let (rule, to_str) = if let Some((check, to)) = r_s.split_once(":") {
        let (p, check) = check.split_at(1);
        let (op, num_str) = check.split_at(1);
        (
            Some(Rule {
                part: str_to_xmas(p).unwrap(),
                compare: num_str.parse().unwrap(),
                operation: Operation::try_from(op).unwrap(),
            }),
            to,
        )
    } else {
        (None, r_s)
    };
    let to = if let Ok(action) = FinalAction::try_from(to_str) {
        PipeTo::Final(action)
    } else {
        PipeTo::Next(to_str)
    };
    Ok((
        r_s,
        WorkflowStep {
            check: rule,
            pipe_to: to,
        },
    ))
}

// px{a<2006:qkq,m>2090:A,rfg}

fn parse_workflow(input: &'static str) -> IResult<&str, (&str, Vec<WorkflowStep>)> {
    let (_, (key, _, instr_str)) = tuple((alpha1, tag("{"), take_until("}")))(input)?;

    let insts = instr_str
        .split(",")
        .map(|x| parse_instruction(x).unwrap().1)
        .collect::<Vec<_>>();

    Ok((input, (key, insts)))
}

#[derive(Debug)]
enum PartParam {
    X,
    M,
    A,
    S,
}

impl TryFrom<&str> for PartParam {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "x" => Ok(PartParam::X),
            "m" => Ok(PartParam::M),
            "a" => Ok(PartParam::A),
            "s" => Ok(PartParam::S),
            _ => Err("Invalid part param"),
        }
    }
}

fn str_to_xmas(input: &str) -> Result<usize, &str> {
    match input {
        "x" => Ok(0),
        "m" => Ok(1),
        "a" => Ok(2),
        "s" => Ok(3),
        _ => Err("Invalid part param"),
    }
}

#[derive(Debug)]
enum Operation {
    Less,
    Greater,
}

// . (7,10) ,< 12 --> (7,10), None
// . (7,10) ,< 10 --> (7,9), (10,10)
// . (7,10) ,< 8 --> (7,7), (9,10)
// . (7,10) ,< 7 --> None, (9,10)

type Range = (u64, u64);

/// Bisect a range into two ranges, based on the bisector and operation.
/// returns (within, without)
/// within satisfies the rule
/// without does not
fn bisect_range(
    range: Range,
    bisector: u64,
    operation: &Operation,
) -> (Option<Range>, Option<Range>) {
    let (within, without) = match operation {
        Operation::Less => (
            (range.0, min(bisector - 1, range.1)),
            (max(bisector, range.0), range.1),
        ),
        Operation::Greater => (
            (max(bisector + 1, range.0), range.1),
            (range.0, min(bisector, range.1)),
        ),
    };

    fn valid(range: Range) -> Option<Range> {
        (range.0 <= range.1).then(|| range)
    }

    (valid(within), valid(without))
}

impl TryFrom<&str> for Operation {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "<" => Ok(Operation::Less),
            ">" => Ok(Operation::Greater),
            _ => Err("Invalid operation"),
        }
    }
}

#[derive(Debug)]
enum FinalAction {
    Accept,
    Reject,
}

impl TryFrom<&str> for FinalAction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(FinalAction::Accept),
            "R" => Ok(FinalAction::Reject),
            _ => Err("Invalid action"),
        }
    }
}

#[derive(Debug)]
enum PipeTo {
    Next(&'static str),
    Final(FinalAction),
}

#[derive(Debug)]
struct Rule {
    part: usize,
    operation: Operation,
    compare: u64,
}

#[derive(Debug)]
struct WorkflowStep {
    check: Option<Rule>,
    pipe_to: PipeTo,
}

struct Workflow {
    steps: Vec<WorkflowStep>,
}

type Workflows = HashMap<&'static str, Workflow>;

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn get_absolute(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

type RangedPart = Vec<Range>;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn it_works() {
        let result = process(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );
        assert_eq!(result, 19114);
    }

    // . (7,10) ,< 12 --> (7,10), None
    // . (7,10) ,< 10 --> (7,9), (10,10)
    // . (7,10) ,< 8 --> (7,7), (8,10)
    // . (7,10) ,< 7 --> None, (7,10)

    #[rstest]
    #[case((7, 10), 12, Operation::Less, (Some((7, 10)), None))]
    #[case((7, 10), 10, Operation::Less, (Some((7, 9)), Some((10, 10))))]
    #[case((7, 10), 8, Operation::Less, (Some((7, 7)), Some((8, 10))))]
    #[case((7, 10), 7, Operation::Less, (None, Some((7, 10))))]
    #[case((7, 10), 10, Operation::Greater, (None, Some((7, 10))))]
    #[case((7, 10), 9, Operation::Greater, (Some((10,10)), Some((7,9))))]
    #[case((7, 10), 7, Operation::Greater, (Some((8,10)), Some((7,7))))]
    #[case((7, 10), 4, Operation::Greater, ( Some((7, 10)),None))]
    fn test_bisect_range(
        #[case] range: Range,
        #[case] bisector: u64,
        #[case] operation: Operation,
        #[case] expected: (Option<Range>, Option<Range>),
    ) {
        let result = bisect_range(range, bisector, &operation);
        assert_eq!(result, expected);
    }
}

// px{a<2006:qkq,m>2090:A,rfg}
// pv{a>1716:R,A}
// lnx{m>1548:A,A}
// rfg{s<537:gd,x>2440:R,A}
// qs{s>3448:A,lnx}
// qkq{x<1416:A,crn}
// crn{x>2662:A,R}
// in{s<1351:px,qqz}
// qqz{s>2770:qs,m<1801:hdj,R}
// gd{a>3333:R,R}
// hdj{m>838:A,pv}

// {x=787,m=2655,a=1222,s=2876}
// {x=1679,m=44,a=2067,s=496}
// {x=2036,m=264,a=79,s=2244}
// {x=2461,m=1339,a=466,s=291}
// {x=2127,m=1623,a=2188,s=1013}
