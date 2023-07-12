use std::{collections::HashSet, ops::RangeInclusive};

use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete as cc, sequence::tuple, Finish, IResult};

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Sensor {
    loc: Position,
    beacon: Position,
}

impl Sensor {
    fn parse(i: &str) -> IResult<&str, Sensor> {
        let (i, (_, x_l, _, y_l, _, x_b, _, y_b)) = tuple((
            tag("Sensor at x="),
            cc::i64,
            tag(", y="),
            cc::i64,
            tag(": closest beacon is at x="),
            cc::i64,
            tag(", y="),
            cc::i64,
        ))(i)?;

        Ok((
            i,
            Self {
                loc: Position { x: x_l, y: y_l },
                beacon: Position { x: x_b, y: y_b },
            },
        ))
    }

    fn dist(&self) -> i64 {
        (self.loc.x.abs_diff(self.beacon.x) + self.loc.y.abs_diff(self.beacon.y)) as i64
    }
}

fn parse_all_sensors(i: &str) -> Vec<Sensor> {
    i.lines()
        .map(|l| Sensor::parse(l).finish().unwrap().1)
        .collect()
}

fn get_ranges(sensors: &Vec<Sensor>, y: i64) -> impl Iterator<Item = RangeInclusive<i64>> {
    let mut ranges = vec![];
    for sensor in sensors {
        let radius = sensor.dist();
        let y_dist = (y - sensor.loc.y).abs();
        if y_dist > radius {
            continue;
        }
        let d = radius - y_dist;
        let mid = sensor.loc.x;
        let start = mid - d;
        let end = mid + d;
        ranges.push(start..=end);
    }

    ranges.sort_by_key(|r| *r.start());
    ranges.into_iter().coalesce(|a, b| {
        if b.start() - 1 <= *a.end() {
            if b.end() > a.end() {
                Ok(*a.start()..=*b.end())
            } else {
                Ok(a)
            }
        } else {
            Err((a, b))
        }
    })
}

fn get_clamped_ranges(
    sensors: &Vec<Sensor>,
    y: i64,
    x_range: RangeInclusive<i64>,
) -> impl Iterator<Item = RangeInclusive<i64>> {
    get_ranges(sensors, y).filter_map(move |r| {
        let r = *r.start().max(x_range.start())..=*r.end().min(x_range.end());
        if r.start() > r.end() {
            None
        } else {
            Some(r)
        }
    })
}

fn impossible_beacons(sensors: &Vec<Sensor>, y: i64) -> usize {
    let beacon_x = sensors
        .iter()
        .filter(|s| s.beacon.y == y)
        .map(|s| s.beacon.x)
        .collect::<HashSet<_>>();

    get_ranges(sensors, y)
        .map(|r| {
            let size = (r.end() - r.start() + 1) as usize;
            let beacons_in_range = beacon_x.iter().filter(|x| r.contains(x)).count();
            size - beacons_in_range
        })
        .sum()
}

fn beacon_position(
    sensors: &Vec<Sensor>,
    x_range: &RangeInclusive<i64>,
    y_range: &RangeInclusive<i64>,
) -> Option<Position> {
    y_range.clone().find_map(|y| {
        get_clamped_ranges(sensors, y, x_range.clone())
            .nth(1)
            .map(|r| Position {
                x: r.start() - 1,
                y,
            })
    })
}

fn main() {
    let input = include_str!("test_files/day_15.txt");
    let sensors = parse_all_sensors(input);
    let part_1 = impossible_beacons(&sensors, 2000000);
    println!("Part 1: {part_1}");
    let pt = beacon_position(&sensors, &(0..=4000000), &(0..=4000000)).unwrap();
    let part_2 = pt.x * 4000000 + pt.y;
    println!("Part 2: {part_2}");
}
