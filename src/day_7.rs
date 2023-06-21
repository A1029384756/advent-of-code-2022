use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::sequence::{preceded, separated_pair};
use nom::Finish;
use nom::IResult;
use std::cell::RefCell;
use std::rc::Rc;

type NodeContainer = Rc<RefCell<Node>>;

#[derive(Default, Clone)]
struct Node {
    pub size: u32,
    pub children: HashMap<PathBuf, NodeContainer>,
    pub parent: Option<NodeContainer>,
}

impl Node {
    fn total_size(&self) -> u32 {
        self.children
            .values()
            .map(|item| item.borrow().total_size())
            .sum::<u32>()
            + self.size
    }

    fn is_dir(&self) -> bool {
        !self.children.is_empty() && self.size == 0
    }
}

struct Ls;
struct Cd(PathBuf);
enum Command {
    Ls,
    Cd(PathBuf),
}

impl From<Ls> for Command {
    fn from(_: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_path(input: &str) -> IResult<&str, PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(input)
}

fn parse_ls(input: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(input)
}

fn parse_cd(input: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(input)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

enum Entry {
    Dir(PathBuf),
    File(u32, PathBuf),
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u32, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );

    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(input)
}

enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(input)
}

fn get_subdirs(n: NodeContainer) -> Box<dyn Iterator<Item = NodeContainer>> {
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();

    Box::new(
        std::iter::once(n).chain(
            children
                .into_iter()
                .filter_map(|c| {
                    if c.borrow().is_dir() {
                        Some(get_subdirs(c))
                    } else {
                        None
                    }
                })
                .flatten(),
        ),
    )
}

fn create_tree(input: &str) -> NodeContainer {
    let lines = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);
    let root = NodeContainer::default();
    let mut node = root.clone();

    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {}
                Command::Cd(path) => match path.to_str() {
                    Some("/") => {}
                    Some("..") => {
                        let parent = node.borrow().parent.clone().unwrap();
                        node = parent;
                    }
                    _ => {
                        let child = node
                            .as_ref()
                            .borrow_mut()
                            .children
                            .entry(path)
                            .or_default()
                            .clone();
                        node = child;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(dir) => {
                    let entry = node
                        .as_ref()
                        .borrow_mut()
                        .children
                        .entry(dir)
                        .or_default()
                        .clone();
                    entry.as_ref().borrow_mut().parent = Some(node.clone());
                }
                Entry::File(size, file) => {
                    let entry = node
                        .as_ref()
                        .borrow_mut()
                        .children
                        .entry(file)
                        .or_default()
                        .clone();
                    entry.as_ref().borrow_mut().size = size;
                    entry.as_ref().borrow_mut().parent = Some(node.clone());
                }
            },
        }
    }

    root
}

fn part_1(fs: NodeContainer) -> u32 {
    get_subdirs(fs)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s <= 100000)
        .sum()
}

fn part_2(fs: NodeContainer) -> u32 {
    let total_space = 70000000;
    let used_space = fs.borrow().total_size();
    let free_space = total_space - used_space;
    let needed_free_space = 30000000;
    let reclaim_min_amount = needed_free_space - free_space;

    get_subdirs(fs).map(|d| d.borrow().total_size())
        .filter(|&s| s >= reclaim_min_amount)
        .min().unwrap()
}

fn main() {
    let input = &read_to_string("./test_files/day_7.txt").expect("File does not exist");
    let root = create_tree(input);
    println!("Part 1: {}", part_1(root.clone()));
    println!("Part 2: {}", part_2(root));
}
