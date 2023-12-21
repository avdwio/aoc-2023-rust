use nom::bytes::complete::{tag, take_until};
use nom::character::complete::alpha1;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;

pub fn process(input: &'static str) -> u64 {
    let (instr_block, block) = input.split_once("\n\n").unwrap();

    let map = instr_block
        .lines()
        .map(|x| parse_workflow(x).unwrap().1)
        .collect::<HashMap<_, _>>();

    let parts = block.lines().map(parse_part).collect::<Vec<_>>();

    let x = parts
        .iter()
        .filter_map(|part| {
            let mut curr_instr = "in";
            let x = 'outer: loop {
                // println!("========");
                // println!("========");
                // println!("========");
                // println!("curr_instr: {}", curr_instr);

                let steps = map.get(curr_instr).unwrap();

                for step in steps {
                    // println!("========");
                    // println!("part: {:?}", part);
                    // println!("step: {:?}", step);

                    let do_action = if let Some(rule) = &step.check {
                        let part_val = match rule.part {
                            PartParam::X => part.x,
                            PartParam::M => part.m,
                            PartParam::A => part.a,
                            PartParam::S => part.s,
                        };

                        match rule.operation {
                            Operation::Less => part_val < rule.compare,
                            Operation::Greater => part_val > rule.compare,
                        }
                    } else {
                        true
                    };
                    if do_action {
                        match &step.pipe_to {
                            PipeTo::Next(next) => {
                                curr_instr = next;
                                continue 'outer;
                            }
                            PipeTo::Final(action) => match action {
                                FinalAction::Accept => {
                                    println!("accept: {:?}", part);
                                    break 'outer Some(part);
                                }
                                FinalAction::Reject => {
                                    println!("reject: {:?}", part);
                                    break 'outer None;
                                }
                            },
                        }
                    }
                    // println!("continuing...")
                }
            };
            x
        })
        .map(|x| x.get_absolute())
        .sum::<_>();

    println!("{:?}", parts[1]);
    x
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
                part: PartParam::try_from(p).unwrap(),
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

#[derive(Debug)]
enum Operation {
    Less,
    Greater,
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
    part: PartParam,
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

    // #[rstest]
    // fn test_parsing_workflows() {
    //     let result = process(include_str!("../input-1-test.txt"));
    //     assert_eq!(result, 19114);
    // }
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
