use anyhow::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{one_of, space1},
    combinator::{all_consuming, map, value},
    multi::separated_list1,
    sequence::{preceded, tuple},
    Finish, IResult,
};

#[derive(Debug, Clone, Copy)]
enum Term {
    Old,
    Const(u64),
}

impl Term {
    fn value(self, old: u64) -> u64 {
        match self {
            Term::Old => old,
            Term::Const(val) => val,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Term, Term),
    Mult(Term, Term),
}

impl Operation {
    fn eval(self, old: u64) -> u64 {
        match self {
            Operation::Add(l, r) => l.value(old) + r.value(old),
            Operation::Mult(l, r) => l.value(old) * r.value(old),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items_inspected: u64,
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    receiver_if_true: usize,
    receiver_if_false: usize,
}

fn parse_term(i: &str) -> IResult<&str, Term> {
    alt((
        value(Term::Old, tag("old")),
        map(nom::character::complete::u64, Term::Const),
    ))(i)
}

fn parse_operation(i: &str) -> IResult<&str, Operation> {
    let (i, (l, op, r)) = preceded(
        tag("new = "),
        tuple((
            parse_term,
            preceded(space1, one_of("*+")),
            preceded(space1, parse_term),
        )),
    )(i)?;
    let op = match op {
        '*' => Operation::Mult(l, r),
        '+' => Operation::Add(l, r),
        _ => unreachable!(),
    };

    Ok((i, op))
}

fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, _) = tuple((tag("Monkey "), nom::character::complete::u64, tag(":\n")))(i)?;
    let (i, (_, _, items, _)) = tuple((
        space1,
        tag("Starting items: "),
        separated_list1(tag(", "), nom::character::complete::u64),
        tag("\n"),
    ))(i)?;
    let (i, (_, _, operation, _)) =
        tuple((space1, tag("Operation: "), parse_operation, tag("\n")))(i)?;
    let (i, (_, _, divisor, _)) = tuple((
        space1,
        tag("Test: divisible by "),
        nom::character::complete::u64,
        tag("\n"),
    ))(i)?;
    let (i, (_, _, receiver_if_true, _)) = tuple((
        space1,
        tag("If true: throw to monkey "),
        map(nom::character::complete::u64, |v| v as usize),
        tag("\n"),
    ))(i)?;
    let (i, (_, _, receiver_if_false, _)) = tuple((
        space1,
        tag("If false: throw to monkey "),
        map(nom::character::complete::u64, |v| v as usize),
        tag("\n"),
    ))(i)?;

    Ok((
        i,
        Monkey {
            items_inspected: 0,
            items,
            operation,
            divisor,
            receiver_if_true,
            receiver_if_false,
        },
    ))
}

fn parse_all_monkeys(i: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(nom::character::complete::multispace1, parse_monkey)(i)
}

fn round_part_1(m: &mut [Monkey]) {
    let monkey_count = m.len();

    for i in 0..monkey_count {
        let mc;
        {
            let monkey = &mut m[i];
            mc = monkey.clone();
            monkey.items_inspected += mc.items.len() as u64;
        }

        for mut item in mc.items.iter().copied() {
            item = mc.operation.eval(item);
            item /= 3;

            if item % mc.divisor == 0 {
                m[mc.receiver_if_true].items.push(item);
            } else {
                m[mc.receiver_if_false].items.push(item);
            }
        }

        m[i].items.clear();
    }
}

fn round_part_2(m: &mut [Monkey], divisors: u64) {
    let monkey_count = m.len();

    for i in 0..monkey_count {
        let mc;
        {
            let monkey = &mut m[i];
            mc = monkey.clone();
            monkey.items_inspected += mc.items.len() as u64;
        }

        for mut item in mc.items.iter().copied() {
            item %= divisors;
            item = mc.operation.eval(item);

            if item % mc.divisor == 0 {
                m[mc.receiver_if_true].items.push(item);
            } else {
                m[mc.receiver_if_false].items.push(item);
            }
        }

        m[i].items.clear();
    }
}

fn part_1(m: &Vec<Monkey>) -> u64 {
    let mut m = m.clone();
    (0..20).for_each(|_| round_part_1(&mut m));

    m.iter()
        .map(|m| m.items_inspected)
        .sorted_by_key(|&c| std::cmp::Reverse(c))
        .take(2)
        .product()
}

fn part_2(m: &Vec<Monkey>) -> u64 {
    let mut m = m.clone();
    let divisors = m.iter().map(|m| m.divisor).product::<u64>();
    (0..10000).for_each(|_| round_part_2(&mut m, divisors));

    m.iter()
        .map(|m| m.items_inspected)
        .sorted_by_key(|&c| std::cmp::Reverse(c))
        .take(2)
        .product()
}

fn main() -> Result<()> {
    let input = include_str!("test_files/day_11.txt");

    let monkeys = all_consuming(parse_all_monkeys)(&input).finish()?.1;

    println!("Part 1: {}", part_1(&monkeys));
    println!("Part 2: {}", part_2(&monkeys));

    Ok(())
}

#[test]
fn test_part_1() {
    let input = include_str!("test_files/day_11_test.txt");
    let monkeys = all_consuming(parse_all_monkeys)(&input).finish().unwrap().1;

    let result = part_1(&monkeys);

    assert_eq!(result, 10605);
}

#[test]
fn test_part_2() {
    let input = include_str!("test_files/day_11_test.txt");
    let monkeys = all_consuming(parse_all_monkeys)(&input).finish().unwrap().1;

    let result = part_2(&monkeys);

    assert_eq!(result, 2713310158);
}
