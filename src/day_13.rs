use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Num(u64),
    List(Vec<Node>),
}

impl Node {
    fn with_slice<T>(&self, f: impl FnOnce(&[Node]) -> T) -> T {
        match self {
            Node::Num(v) => f(&[Self::Num(*v)]),
            Node::List(v) => f(&v[..]),
        }
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Num(a), Node::Num(b)) => a.partial_cmp(b),
            (l, r) => l.with_slice(|l| r.with_slice(|r| l.partial_cmp(r))),
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(v) => write!(f, "{v}"),
            Self::List(v) => f.debug_list().entries(v).finish(),
        }
    }
}

fn part_1(i: &str) -> usize {
    let mut sum = 0;
    for (i, groups) in i.split("\n\n").enumerate() {
        let i = i + 1;
        let mut nodes = groups
            .lines()
            .map(|line| serde_json::from_str::<Node>(line).unwrap());
        let l = nodes.next().unwrap();
        let r = nodes.next().unwrap();

        if l < r {
            sum += i;
        }
    }

    sum
}

fn part_2(i: &str) -> usize {
    let dividers = vec![
        Node::List(vec![Node::Num(2)]),
        Node::List(vec![Node::Num(6)]),
    ];

    let mut packets = i
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str::<Node>(l).unwrap())
        .chain(dividers.iter().cloned())
        .collect::<Vec<_>>();

    packets.sort();

    dividers
        .iter()
        .map(|d| packets.binary_search(d).unwrap() + 1)
        .product::<usize>()
}

fn main() {
    let input = include_str!("test_files/day_13.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[test]
fn test_part_1() {
    let input = include_str!("test_files/day_13_test.txt");
    let result = part_1(input);

    assert_eq!(result, 13);
}

#[test]
fn test_part_2() {
    let input = include_str!("test_files/day_13_test.txt");
    let result = part_2(input);

    assert_eq!(result, 140);
}
