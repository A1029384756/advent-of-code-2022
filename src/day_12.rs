use egui::{Color32, Rect, Rounding, Sense, Slider, Stroke, Vec2};
use itertools::izip;
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

trait Interpolate {
    type T;
    fn lerp(v: Self::T, f: (Self::T, Self::T), t: (Self::T, Self::T)) -> Self::T;
}

impl Interpolate for Color32 {
    type T = Color32;

    fn lerp(
        v: Self::T,
        (f_min, f_max): (Self::T, Self::T),
        (t_min, t_max): (Self::T, Self::T),
    ) -> Self::T {
        let map_range = |val: f32, from: (f32, f32), to: (f32, f32)| {
            to.0 + (val - from.0) * (to.1 - to.0) / (from.1 - from.0)
        };

        let arr = izip!(
            f_min.to_array(),
            f_max.to_array(),
            t_min.to_array(),
            t_max.to_array(),
            v.to_array()
        )
        .map(|(f_min, f_max, t_min, t_max, v)| {
            map_range(
                v as f32,
                (f_min as f32, f_max as f32),
                (t_min as f32, t_max as f32),
            ) as u8
        })
        .collect::<Vec<u8>>();

        Color32::from_rgb(arr[0], arr[1], arr[2])
    }
}

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
    speed: u32,
    paused: bool,
    step: bool,
    finished: bool,
}

impl Grid {
    fn new() -> Self {
        let i = include_str!("test_files/day_12.txt");
        Self::parse(i)
    }

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
            speed: 1,
            paused: true,
            step: false,
            finished: false,
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

    fn step(&mut self) {
        if self.finished {
            return;
        }

        if self.current.is_empty() {
            let end_coord = self.get_end();
            self.current.insert(end_coord);
            self.visited.insert(end_coord, PrevCell::from(None));
            return;
        }

        let current = std::mem::take(&mut self.current);
        let mut next = HashSet::new();
        let mut visited = std::mem::take(&mut self.visited);

        for curr in current {
            for neighbor in self.possible_neighbors(curr) {
                if self.get_cell(neighbor).unwrap().get_height() == Cell::Start.get_height() {
                    self.steps += 1;
                    self.finished = true;
                    self.visited = visited;
                    return;
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
            let mut painter_size = ui.available_size_before_wrap();
            if !painter_size.is_finite() {
                painter_size = egui::vec2(500.0, 500.0);
            }

            let (res, painter) = ui.allocate_painter(painter_size, Sense::drag());

            let tile_max_size = Vec2::new(
                res.rect.width() / self.width as f32,
                res.rect.height() / self.height as f32,
            );

            let side = tile_max_size.min_elem();

            let anchor = (res.rect.right_bottom().to_vec2()
                - Vec2::new(side * self.width as f32, side * self.height as f32))
                / 2.;

            let to_panel_pos = |pos: Coord| {
                ((Vec2::new(pos.x as f32 * side, pos.y as f32 * side)) + anchor).to_pos2()
            };

            let style = &ctx.style().visuals;
            painter.rect_filled(res.rect, Rounding::same(0.0), style.window_fill());

            let to_tile_color = |height: usize| {
                let bg = style.window_fill();
                let fg = style.text_color();
                let from_bg = Color32::from_gray(Cell::Start.get_height() as u8);
                let from_fg = Color32::from_gray(Cell::End.get_height() as u8);
                let tile = Color32::from_gray(height as u8);

                Color32::lerp(tile, (from_bg, from_fg), (bg, fg))
            };

            for x in 0..self.width {
                for y in 0..self.height {
                    let rect = Rect::from_center_size(
                        to_panel_pos((x, y).into()),
                        Vec2::new(side + 1., side + 1.),
                    );
                    let height = self.get_cell((x, y).into()).unwrap().get_height();
                    painter.rect_filled(rect, Rounding::same(0.0), to_tile_color(height));
                }
            }

            let arrow_color = Color32::YELLOW;
            for v in self.visited.iter() {
                match v.1 {
                    Some(prev) => {
                        let curr_pos = to_panel_pos(*v.0);
                        let prev_pos = to_panel_pos(*prev);
                        painter.circle_filled(curr_pos, side * 0.1, arrow_color);
                        painter.arrow(prev_pos, curr_pos - prev_pos, Stroke::new(1.0, arrow_color))
                    }
                    None => {
                        let pos = to_panel_pos(*v.0);
                        painter.circle_filled(pos, side * 0.3, arrow_color);
                    }
                }
            }
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
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
