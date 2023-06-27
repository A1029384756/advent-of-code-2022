use egui::{Color32, Rect, Rounding, Sense, Slider, Stroke, Vec2};
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

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

            if res.dragged_by(egui::PointerButton::Primary) {
                res.clone().on_hover_cursor(egui::CursorIcon::Grabbing);
            }

            let side = painter_size / Vec2::new(self.width as f32, self.height as f32);

            let remap = |val: f32, from: (f32, f32), to: (f32, f32)| {
                to.0 + (val - from.0) * (to.1 - to.0) / (from.1 - from.0)
            };

            let to_panel_pos =
                |pos: Coord| (Vec2::new(pos.x as f32 * side.x, pos.y as f32 * side.y)).to_pos2();

            let to_tile_color = |height: usize| {
                Color32::from_gray(remap(
                    height as f32,
                    (
                        Cell::Start.get_height() as f32,
                        Cell::End.get_height() as f32,
                    ),
                    (0.0, 255.0),
                ) as u8)
            };

            for x in 0..self.width {
                for y in 0..self.height {
                    let height = self.get_cell((x, y).into()).unwrap().get_height();
                    let rect = Rect::from_center_size(to_panel_pos((x, y).into()), side);
                    painter.rect_filled(rect, Rounding::same(0.0), to_tile_color(height));
                }
            }

            for v in self.visited.iter() {
                match v.1 {
                    Some(prev) => {
                        let curr_pos = to_panel_pos(*v.0);
                        let prev_pos = to_panel_pos(*prev);
                        painter.arrow(
                            prev_pos,
                            curr_pos - prev_pos,
                            Stroke::new(1.0, Color32::YELLOW),
                        )
                    }
                    None => {}
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
