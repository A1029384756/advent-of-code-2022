use core::fmt;

use anyhow::Result;

#[derive(Copy, Clone)]
struct GridCoord {
    x: usize,
    y: usize,
}

struct Grid {
    contents: Vec<usize>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(grid: &str) -> Grid {
        Grid {
            contents: grid
                .lines()
                .map(|l| l.chars().map(|c| c as usize - '0' as usize))
                .flatten()
                .collect(),
            width: grid.lines().next().unwrap().len(),
            height: grid.lines().count(),
        }
    }

    fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    fn cell(&self, coord: GridCoord) -> Option<usize> {
        if self.in_bounds(coord) {
            Some(self.contents[coord.y * self.width + coord.x])
        } else {
            None
        }
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.contents
            .chunks(self.width)
            .map(|c| writeln!(f, "{}", c.iter().map(|v| v.to_string()).collect::<String>()))
            .collect()
    }
}

fn trees_in_direction(g: &Grid, c: GridCoord, (x, y): (isize, isize)) -> usize {
    let line = (1..).into_iter().map_while(|i| {
        let coord = GridCoord {
            x: c.x.checked_add_signed(x * i)?,
            y: c.y.checked_add_signed(y * i)?,
        };
        Some(g.cell(coord)?)
    });

    let mut total = 0;
    let height = g.cell(c).unwrap();
    for h in line {
        total += 1;
        if h >= height {
            break;
        }
    }

    total
}

fn get_score(g: &Grid, c: GridCoord) -> usize {
    let views = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    views
        .into_iter()
        .map(|(x, y)| trees_in_direction(g, c, (x, y)))
        .product()
}

fn part_1(g: &Grid) -> usize {
    let all_coords = (0..g.height)
        .into_iter()
        .flat_map(|y| (0..g.width).into_iter().map(move |x| GridCoord { x, y }));

    all_coords
        .filter(|&c| {
            let height = g.cell(c).unwrap();
            let views = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            views.iter().any(|&(x, y)| {
                let mut cells = (1..).into_iter().map_while(|i| {
                    let coord = GridCoord {
                        x: c.x.checked_add_signed(x * i)?,
                        y: c.y.checked_add_signed(y * i)?,
                    };
                    g.cell(coord)
                });
                cells.all(|h| h < height)
            })
        })
        .count()
}

fn part_2(g: &Grid) -> usize {
    let all_coords = (0..g.height)
        .into_iter()
        .flat_map(|y| (0..g.width).into_iter().map(move |x| GridCoord { x, y }));

    all_coords
        .map(|c| (c, get_score(&g, c)))
        .max_by_key(|(_, score)| *score)
        .unwrap().1
}

fn main() -> Result<()> {
    let input = &include_str!("test_files/day_8.txt");
    let grid = Grid::new(input);
    println!("{}", part_1(&grid));
    println!("{}", part_2(&grid));

    Ok(())
}

#[test]
fn test_grid_creation() {
    let input = "30373\n25512\n65332\n33549\n35390";
    let g = Grid::new(input);
    assert_eq!(format!("{g:?}"), "30373\n25512\n65332\n33549\n35390\n");
}
