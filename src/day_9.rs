use anyhow::Result;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::{all_consuming, map, value},
    sequence::{preceded, tuple},
    Finish, IResult,
};
use std::{
    collections::{HashSet, VecDeque},
    fmt,
    fs::read_to_string,
};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct GridCoord {
    x: i32,
    y: i32,
}

impl fmt::Debug for GridCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add for GridCoord {
    type Output = GridCoord;

    fn add(self, rhs: GridCoord) -> GridCoord {
        GridCoord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for GridCoord {
    fn add_assign(&mut self, rhs: Self) {
        *self = GridCoord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for GridCoord {
    type Output = GridCoord;

    fn sub(self, rhs: Self) -> GridCoord {
        GridCoord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            value(Direction::Up, tag("U")),
            value(Direction::Down, tag("D")),
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
        ))(i)
    }

    fn delta(self) -> GridCoord {
        match self {
            Direction::Up => GridCoord { x: 0, y: -1 },
            Direction::Down => GridCoord { x: 0, y: 1 },
            Direction::Left => GridCoord { x: -1, y: 0 },
            Direction::Right => GridCoord { x: 1, y: 0 },
        }
    }
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    dist: u32,
}

impl Instruction {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                Direction::parse,
                preceded(space1, nom::character::complete::u32),
            )),
            |(dir, dist)| Self { dir, dist },
        )(i)
    }
}

struct Simulation {
    instructions: VecDeque<Instruction>,
    knots: [GridCoord; 10],
    tail_visited: HashSet<GridCoord>,
}

impl Simulation {
    fn new(i: &str) -> Self {
        let instructions = i
            .lines()
            .map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1)
            .collect();

        Self {
            instructions,
            knots: [GridCoord { x: 0, y: 0 }; 10],
            tail_visited: HashSet::default(),
        }
    }

    fn step(&mut self) {
        let Some(inst) = self.instructions.front_mut() else { return; };
        self.knots[0] += inst.dir.delta();

        for i in 1..self.knots.len() {
            let diff = self.knots[i - 1] - self.knots[i];
            let (dx, dy) = match (diff.x, diff.y) {
                (0, 0) => (0, 0),
                (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
                (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
                (0, 2) => (0, 1),
                (0, -2) => (0, -1),
                (2, 0) => (1, 0),
                (-2, 0) => (-1, 0),
                (2, 1) => (1, 1),
                (2, -1) => (1, -1),
                (-2, 1) => (-1, 1),
                (-2, -1) => (-1, -1),
                (1, 2) => (1, 1),
                (-1, 2) => (-1, 1),
                (1, -2) => (1, -1),
                (-1, -2) => (-1, -1),
                (-2, -2) => (-1, -1),
                (-2, 2) => (-1, 1),
                (2, -2) => (1, -1),
                (2, 2) => (1, 1),
                _ => panic!("Should never happen: {diff:?}"),
            };

            self.knots[i].x += dx;
            self.knots[i].y += dy;
            if i == self.knots.len() - 1 {
                self.tail_visited.insert(self.knots[i]);
            }
        }
        inst.dist -= 1;
        if inst.dist == 0 {
            self.instructions.pop_front();
        }
    }
}

fn simulate(i: &str) -> usize {
    let mut sim = Simulation::new(i);
    while !sim.instructions.is_empty() {
        sim.step();
    }

    sim.tail_visited.len()
}

fn main() -> Result<()> {
    let input = &read_to_string("./test_files/day_9.txt")?;
    println!("Result: {}", simulate(input));
    Ok(())
}
