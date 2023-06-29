#![feature(extract_if)]
#![feature(generators)]
#![feature(iter_from_generator)]

use std::{fmt, time::Duration};

use egui::{ColorImage, Slider, TextureOptions};

use image::ImageBuffer;
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
    speed: u32,
    paused: bool,
    step: bool,
    img: Option<egui::TextureHandle>,
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
        min_x = 300;
        max_x = 700;
        max_y = floor_y;
        lines.push(Line {
            points: vec![
                Coord {
                    x: min_x,
                    y: floor_y,
                },
                Coord {
                    x: max_x,
                    y: floor_y,
                },
            ],
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
            speed: 1,
            paused: true,
            step: false,
            img: None,
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
        if matches!(self.get_unit(Coord { x: 500, y: 0 }).unwrap(), Unit::Sand) {
            return;
        }

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

impl eframe::App for Grid {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Reset").clicked() {
                    *self = Self::new();
                }

                if ui.button("Step").clicked() {
                    self.step();
                }

                let paused = self.paused;
                ui.toggle_value(&mut self.paused, if paused { "▶" } else { "⏸" });
            });

            ui.horizontal(|ui| {
                ui.label("Speed: ");
                ui.add(Slider::new(&mut self.speed, 1..=20).prefix("x"));
            });
        });

        if self.step {
            self.step();
            self.step = false;
        } else if !self.paused {
            (0..self.speed).for_each(|_| {
                self.step();
            });
            ctx.request_repaint_after(Duration::from_millis(25));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut buff = ImageBuffer::new(self.width as _, self.height as _);

            for pixel in buff.pixels_mut() {
                *pixel = image::Rgba([255, 20, 20, 255]);
            }

            let style = &ctx.style().visuals;
            let air = style.window_fill();

            let air_color: [u8; 4] = [air.r(), air.g(), air.b(), air.a()];
            let rock_color: [u8; 4] = [160, 160, 160, 255];
            let sand_color: [u8; 4] = [130, 127, 88, 255];
            let curr_color: [u8; 4] = [245, 206, 49, 255];

            for (x, y, pixel) in buff.enumerate_pixels_mut() {
                let coord = Coord {
                    x: x as _,
                    y: y as _,
                } + self.origin;

                let unit = self.get_unit(coord).unwrap();
                let color = match unit {
                    Unit::Air => &air_color,
                    Unit::Rock => &rock_color,
                    Unit::Sand => &sand_color,
                };

                *pixel = image::Rgba(*color);
            }

            for grain in self.grains.iter().copied() {
                let Coord { x, y } = grain - self.origin;
                buff.put_pixel(x as _, y as _, image::Rgba(curr_color));
            }

            let img =
                ColorImage::from_rgba_unmultiplied([buff.width() as _, buff.height() as _], &buff);

            self.img = Some(ui.ctx().load_texture("", img, TextureOptions::NEAREST));

            if let Some(img) = self.img.as_ref() {
                ui.image(img, ui.available_size());
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Advent of Code 2022 - Day 9",
        options,
        Box::new(|_cc| Box::new(Grid::new())),
    )
    .expect("eframe failed to start");
}
