use anyhow::Result;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Ord, Eq, PartialEq, PartialOrd)]
enum Cell {
    Start,
    End,
    Elevation(usize),
}

impl Cell {
    fn parse(c: char) -> Option<Self> {
        match c {
            'S' => Some(Cell::Start),
            'E' => Some(Cell::End),
            'a'..='z' => Some(Cell::Elevation(c as usize)),
            '\n' => None,
            _ => panic!("Invalid character"),
        }
    }

    fn get_height(&self) -> usize {
        match self {
            Cell::Start => 97,
            Cell::End => 122,
            Cell::Elevation(h) => *h,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Ord, Eq, PartialEq, PartialOrd)]
struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Coord {
            x: value.0,
            y: value.1,
        }
    }
}

type PrevCell = Option<Coord>;

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    visited: HashMap<Coord, PrevCell>,
    current: HashSet<Coord>,
    steps: usize,
}

impl Grid {
    fn parse(i: &str) -> Self {
        let width = i.lines().next().expect("Should not be empty").len();
        let height = i.lines().count();

        Grid {
            width,
            height,
            cells: i
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| Cell::parse(c))
                .filter_map(|c| match c {
                    Some(v) => Some(v),
                    None => None,
                })
                .collect(),
            visited: Default::default(),
            current: Default::default(),
            steps: 0,
        }
    }

    fn in_bounds(&self, c: Coord) -> bool {
        c.x < self.width && c.y < self.height
    }

    fn get_cell(&self, c: Coord) -> Option<&Cell> {
        self.cells.get(c.x + self.width * c.y)
    }

    fn get_end(&self) -> Coord {
        for x in 0..self.width {
            for y in 0..self.height {
                let coord = (x, y).into();
                if let Cell::End = self.get_cell(coord).unwrap() {
                    return coord;
                }
            }
        }

        #[allow(unreachable_code)]
        !unreachable!()
    }

    fn possible_neighbors(&self, c: Coord) -> Vec<Coord> {
        let current_height = self.get_cell(c).unwrap().get_height();
        let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        deltas
            .into_iter()
            .filter_map(move |(dx, dy)| {
                Some(Coord {
                    x: c.x.checked_add_signed(dx)?,
                    y: c.y.checked_add_signed(dy)?,
                })
                .filter(|&c| self.in_bounds(c))
                .filter(|&c| self.get_cell(c).unwrap().get_height() >= current_height - 1)
            })
            .collect()
    }

    fn step_part_1(&mut self) -> bool {
        if self.current.is_empty() {
            let end_coord = self.get_end();
            self.current.insert(end_coord);
            self.visited.insert(end_coord, PrevCell::from(None));
            return false;
        }

        let current = std::mem::take(&mut self.current);
        let mut next = HashSet::new();
        let mut visited = std::mem::take(&mut self.visited);

        for curr in current {
            for neighbor in self.possible_neighbors(curr) {
                if let Some(Cell::Start) = self.get_cell(neighbor) {
                    self.steps += 1;
                    return true;
                }

                if visited.contains_key(&neighbor) {
                    continue;
                }

                visited.insert(neighbor, PrevCell::from(Some(curr)));
                next.insert(neighbor);
            }
        }

        self.current = next;
        self.visited = visited;
        self.steps += 1;
        false
    }

    fn step_part_2(&mut self) -> bool {
        if self.current.is_empty() {
            let end_coord = self.get_end();
            self.current.insert(end_coord);
            self.visited.insert(end_coord, PrevCell::from(None));
            return false;
        }

        let current = std::mem::take(&mut self.current);
        let mut next = HashSet::new();
        let mut visited = std::mem::take(&mut self.visited);

        for curr in current {
            for neighbor in self.possible_neighbors(curr) {
                if self.get_cell(neighbor).unwrap().get_height() == Cell::Start.get_height() {
                    self.steps += 1;
                    return true;
                }

                if visited.contains_key(&neighbor) {
                    continue;
                }

                visited.insert(neighbor, PrevCell::from(Some(curr)));
                next.insert(neighbor);
            }
        }

        self.current = next;
        self.visited = visited;
        self.steps += 1;
        false
    }
}

fn part_1(i: &str) -> usize {
    let mut g = Grid::parse(i);
    while !g.step_part_1() {}

    g.steps
}

fn part_2(i: &str) -> usize {
    let mut g = Grid::parse(i);
    while !g.step_part_2() {}

    g.steps
}

fn main() -> Result<()> {
    let input = include_str!("test_files/day_12.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
    Ok(())
}

#[test]
fn parse_test() {
    let input = include_str!("test_files/day_12_test.txt");
    let g = Grid::parse(input);

    assert_eq!(g.width, 8);
    assert_eq!(g.height, 5);
    assert_eq!(g.get_end(), (5, 2).into());
}

#[test]
fn part_1_test() {
    let input = include_str!("test_files/day_12_test.txt");
    let result = part_1(input);

    assert_eq!(result, 31);
}

#[test]
fn part_2_test() {
    let input = include_str!("test_files/day_12_test.txt");
    let result = part_2(input);

    assert_eq!(result, 29);
}
