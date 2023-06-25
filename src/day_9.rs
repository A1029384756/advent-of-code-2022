use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::{all_consuming, map, value},
    sequence::{preceded, tuple},
    Finish, IResult,
};
use std::{collections::VecDeque, fmt, time::Duration};

use eframe::{egui, epaint::ahash::HashSet};
use egui::{Color32, Sense, Slider, Stroke, Vec2};

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

impl From<GridCoord> for Vec2 {
    fn from(value: GridCoord) -> Self {
        Vec2 {
            x: value.x as f32,
            y: value.y as f32,
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
    speed: u32,
    paused: bool,
    show_sidebar: bool,
    step: bool,
    view_origin: Vec2,
    zoom_level: f32,
}

impl Simulation {
    fn new() -> Self {
        let instructions = include_str!("test_files/day_9.txt")
            .lines()
            .map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1)
            .collect();

        Self {
            instructions,
            knots: [GridCoord { x: 0, y: 0 }; 10],
            tail_visited: HashSet::default(),
            speed: 1,
            paused: true,
            show_sidebar: true,
            step: false,
            view_origin: Vec2::default(),
            zoom_level: 1.0,
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

impl eframe::App for Simulation {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.style_mut().spacing.interact_size.y *= 1.4;
                ui.style_mut()
                    .text_styles
                    .get_mut(&egui::TextStyle::Button)
                    .unwrap()
                    .size *= 1.4;

                if ui.button("Reset").clicked() {
                    *self = Self::new();
                }
                if ui.button("Step").clicked() {
                    self.step = true;
                }

                let paused = self.paused;
                ui.toggle_value(&mut self.paused, if paused { "▶" } else { "⏸" });

                ui.toggle_value(&mut self.show_sidebar, "Sidebar");
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

        if self.show_sidebar {
            egui::SidePanel::right("side_panel").show(ctx, |ui| {
                ui.label(format!("{} places visited", self.tail_visited.len()));
                egui::ScrollArea::new([false, true]).show(ui, |ui| {
                    let mut it = self.instructions.iter();
                    for (i, ins) in it.by_ref().enumerate() {
                        if i >= 20 {
                            break;
                        }

                        let arrow = match ins.dir {
                            Direction::Up => "⬆",
                            Direction::Down => "⬇",
                            Direction::Right => "➡",
                            Direction::Left => "⬅",
                        };
                        let dist = ins.dist as usize;
                        if dist > 5 {
                            ui.label(format!("{}+{}", arrow.repeat(5), dist - 5));
                        } else {
                            ui.label(arrow.repeat(dist));
                        }
                    }
                    let remaining = it.count();

                    if remaining > 0 {
                        ui.label(format!("(+ {remaining} more)"));
                    }
                })
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut painter_size = ui.available_size_before_wrap();
            if !painter_size.is_finite() {
                painter_size = egui::vec2(500.0, 500.0);
            }

            const SIDE: f32 = 5.0;

            let (res, painter) = ui.allocate_painter(painter_size, Sense::drag());

            let scroll_delta = ui.input(|i| i.scroll_delta);
            if scroll_delta != Vec2::ZERO {
                self.zoom_level += scroll_delta.y * 0.0005;
            }

            if res.dragged_by(egui::PointerButton::Primary) {
                res.clone().on_hover_cursor(egui::CursorIcon::Grabbing);
                self.view_origin += res.drag_delta();
            }

            let center = res.rect.center().to_vec2();
            let zoom_clamped = self.zoom_level.clamp(0.1, 10.0);
            let zoom_vec = Vec2::new(zoom_clamped, zoom_clamped);

            let to_panel_pos = |pos: GridCoord| {
                ((Vec2::new(pos.x as f32 * SIDE, pos.y as f32 * SIDE) * zoom_vec)
                    + center
                    + self.view_origin)
                    .to_pos2()
            };

            self.tail_visited.iter().for_each(|coord| {
                let dot_pos = to_panel_pos(*coord);
                painter.circle_stroke(dot_pos, 2.0, Stroke::new(2.0, Color32::DARK_RED));
            });

            let num_knots = self.knots.len();

            for (i, knot_pos) in self.knots.iter().copied().enumerate() {
                let knot_pos = to_panel_pos(knot_pos);
                if i > 0 {
                    let prev_pos = to_panel_pos(self.knots[i - 1]);
                    painter.arrow(
                        prev_pos,
                        knot_pos - prev_pos,
                        Stroke::new(1.0, Color32::YELLOW),
                    )
                }
            }

            for (i, knot_pos) in self.knots.iter().copied().enumerate() {
                let knot_pos = to_panel_pos(knot_pos);
                painter.circle_filled(
                    knot_pos,
                    2.0,
                    Color32::from_rgb(
                        20,
                        60 + ((255.0 - 60.0) * (num_knots as f32 - i as f32) / num_knots as f32)
                            as u8,
                        20,
                    ),
                );
            }
        });
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "canvas",
                web_options,
                Box::new(|_cc| Box::new(Simulation::new())),
            )
            .await
            .expect("eframe failed to start");
    });
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
        Box::new(|_cc| Box::new(Simulation::new())),
    )
    .expect("eframe failed to start");
}
