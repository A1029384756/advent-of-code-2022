#![feature(extract_if)]
#![feature(generators)]
#![feature(iter_from_generator)]

use std::fmt;

use nom::{
    bytes::complete::tag, character::complete as cc, multi::separated_list1, sequence::tuple,
    Finish, IResult,
};

const SPAWN_POINT: Coord = Coord { x: 500, y: 0 };

#[derive(Copy, Clone)]
enum Unit {
    Air,
    Rock,
    Sand,
}

#[derive(Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

fn parse_coord(i: &str) -> IResult<&str, Coord> {
    let (i, (x, _, y)) = tuple((cc::i32, tag(","), cc::i32))(i)?;
    Ok((i, Coord { x, y }))
}

impl Coord {
    fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

impl std::ops::Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Line {
    points: Vec<Coord>,
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    let (i, points) = separated_list1(tag(" -> "), parse_coord)(i)?;
    Ok((i, Line { points }))
}

impl Line {
    fn path_points(&self) -> impl Iterator<Item = Coord> + '_ {
        std::iter::from_generator(|| {
            let mut pts = self.points.iter().copied();
            let Some(mut a) = pts.next() else { return };
            yield a;

            loop {
                let Some(b) = pts.next() else { return };
                let delta = (b - a).signum();

                loop {
                    a += delta;
                    yield a;
                    if a == b {
                        break;
                    }
                }
            }
        })
    }
}

struct Grid {
    origin: Coord,
    width: usize,
    height: usize,
    data: Vec<Unit>,
    grains: Vec<Coord>,
    settled: i32,
}

impl Grid {
    fn new() -> Self {
        let input = include_str!("test_files/day_14.txt");

        let mut lines = input
            .lines()
            .map(|l| parse_line(l).finish().unwrap().1)
            .collect::<Vec<_>>();

        let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);

        for point in lines
            .iter()
            .flat_map(|p| p.points.iter())
            .chain(std::iter::once(&Coord { x: 500, y: 0 }))
        {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }
        
        let floor_y = max_y + 2;
        min_x = 200;
        max_x = 800;
        max_y = floor_y;
        lines.push(Line {
            points: vec![
                Coord { x: min_x, y:floor_y },
                Coord { x: max_x, y:floor_y },
            ]
        });

        let origin = Coord { x: min_x, y: min_y };
        let width: usize = (max_x - min_x + 1).try_into().unwrap();
        let height: usize = (max_y - min_y + 1).try_into().unwrap();

        let mut grid = Self {
            origin,
            width,
            height,
            data: vec![Unit::Air; width * height],
            grains: vec![],
            settled: 0,
        };

        for point in lines.iter().flat_map(|p| p.path_points()) {
            *grid.get_unit_mut(point).unwrap() = Unit::Rock;
        }

        grid
    }

    fn unit_idx(&self, c: Coord) -> Option<usize> {
        let Coord { x, y } = c - self.origin;
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    fn get_unit_mut(&mut self, c: Coord) -> Option<&mut Unit> {
        let cell_idx = self.unit_idx(c)?;
        Some(&mut self.data[cell_idx])
    }

    fn get_unit(&self, c: Coord) -> Option<&Unit> {
        Some(&self.data[self.unit_idx(c)?])
    }

    fn step(&mut self) {
        let mut grains = std::mem::take(&mut self.grains);
        let _ = grains
            .extract_if(|grain| {
                let straight_down = *grain + Coord { x: 0, y: 1 };
                let down_left = *grain + Coord { x: -1, y: 1 };
                let down_right = *grain + Coord { x: 1, y: 1 };
                let options = [straight_down, down_left, down_right];

                if let Some(p) = options
                    .into_iter()
                    .find(|pos| matches!(self.get_unit(*pos), Some(Unit::Air)))
                {
                    *grain = p;
                    return false;
                }

                if options.into_iter().any(|pos| self.get_unit(pos).is_none()) {
                    return true;
                }

                self.settled += 1;
                *self.get_unit_mut(*grain).unwrap() = Unit::Sand;
                true
            })
            .count();
        self.grains = grains;
        self.grains.push(SPAWN_POINT);
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord {
                    x: x as _,
                    y: y as _,
                } + self.origin;
                let unit = self.get_unit(coord).unwrap();
                let u = match unit {
                    Unit::Air => '.',
                    Unit::Rock => '#',
                    Unit::Sand => 'o',
                };
                write!(f, "{u}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve() -> i32 {
    let mut g = Grid::new();

    loop {
        g.step();

        if matches!(g.get_unit(Coord { x: 500, y: 0 }).unwrap(), Unit::Sand) {
            break;
        }
    }

    g.settled
}

fn main() {
    println!("{}", solve());
}
