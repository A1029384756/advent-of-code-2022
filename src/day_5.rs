use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while1};
use nom::combinator::{all_consuming, map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, tuple};
use nom::{Finish, IResult};

struct Instruction {
    src: usize,
    dest: usize,
    n: usize,
}

#[derive(Debug)]
struct Containers {
    stacks: Vec<Vec<char>>,
}

impl Containers {
    fn from_picture(pic: &str) -> Containers {
        let v: Vec<_> = pic
            .lines()
            .by_ref()
            .map_while(|line| {
                all_consuming(parse_crate_line)(line)
                    .finish()
                    .ok()
                    .map(|(_, line)| line)
            })
            .collect();

        Containers {
            stacks: transpose(v),
        }
    }

    fn move_containers_one_by_one(&mut self, m: &Instruction) {
        let src_stack = &mut self.stacks[m.src];
        let tmp: Vec<_> = src_stack.drain((src_stack.len() - m.n)..).rev().collect();

        let dest_stack = &mut self.stacks[m.dest];
        dest_stack.extend(tmp);
    }

    fn move_containers_in_bulk(&mut self, m: &Instruction) {
        let src_stack = &mut self.stacks[m.src];
        let tmp: Vec<_> = src_stack.drain((src_stack.len() - m.n)..).collect();

        let dest_stack = &mut self.stacks[m.dest];
        dest_stack.extend(tmp);
    }

    fn perform_instructions_p1(&mut self, instructions: &Vec<Instruction>) {
        instructions.iter().for_each(|inst| {
            self.move_containers_one_by_one(inst);
        });
    }

    fn perform_instructions_p2(&mut self, instructions: &Vec<Instruction>) {
        instructions.iter().for_each(|inst| {
            self.move_containers_in_bulk(inst);
        });
    }

    fn get_top_stacks(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| match stack.last() {
                Some(top) => top,
                None => &' ',
            })
            .collect::<String>()
    }
}

fn parse_crate(input: &str) -> IResult<&str, char> {
    let first_char = |s: &str| s.chars().next().unwrap();
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(input)
}

fn parse_hole(input: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(input)
}

fn parse_crate_or_hole(input: &str) -> IResult<&str, Option<char>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(input)
}

fn parse_crate_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(tag(" "), parse_crate_or_hole)(input)
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(input)
}

fn parse_pile_number(input: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(n, src, dest)| Instruction { src, dest, n },
    )(input)
}

fn transpose<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn create_container_and_instructions(input: &str) -> (Containers, Vec<Instruction>) {
    let (picture, moves) = input
        .split("\n\n")
        .collect_tuple::<(&str, &str)>()
        .expect("Input should have exactly two sections");

    let containers = Containers::from_picture(picture);
    let instructions: Vec<_> = moves
        .lines()
        .map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1)
        .collect();

    (containers, instructions)
}

fn part_1(input: &str) -> String {
    let (mut containers, instructions) = create_container_and_instructions(input);
    containers.perform_instructions_p1(&instructions);
    containers.get_top_stacks()
}

fn part_2(input: &str) -> String {
    let (mut containers, instructions) = create_container_and_instructions(input);
    containers.perform_instructions_p2(&instructions);
    containers.get_top_stacks()
}

fn main() {
    let input = &include_str!("test_files/day_5.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[test]
fn test_part_1() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    assert_eq!(part_1(input), "CMZ");
}

#[test]
fn test_part_2() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    assert_eq!(part_2(input), "MCD");
}
