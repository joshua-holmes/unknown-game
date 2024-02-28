use std::time::{Duration, Instant};

use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::rendering::glsl_types::Resolution;

use super::{dot::Dot, geometry::Vec2, material::Material, DELAY_BETWEEN_DOTS};

pub enum CoordConversion<T> {
    Converted(T),
    OutOfBounds,
}

pub struct Canvas {
    pub grid: Vec<Vec<Material>>,
    pub dots: Vec<Dot>,
    pub last_dot_spawned: Instant,
}
impl Canvas {
    pub fn new(resolution: PhysicalSize<u32>) -> Self {
        let grid = (0..resolution.height)
            .map(|_| {
                (0..resolution.width)
                    .map(|_| Material::EmptySpace)
                    .collect()
            })
            .collect();
        Self {
            grid,
            dots: Vec::with_capacity((resolution.height * resolution.width) as usize),
            last_dot_spawned: Instant::now(),
        }
    }

    pub fn iter_materials<'a>(&'a self) -> impl Iterator<Item = u8> + 'a {
        self.grid.iter().flatten().map(|m| *m as u8)
    }

    pub fn resolution(&self) -> Resolution {
        Resolution {
            height: self.grid.len() as i32,
            width: self.grid[0].len() as i32,
        }
    }

    pub fn set_next_frame(&mut self, delta_time: &Duration) {
        let res = self.resolution();
        for dot in self.dots.iter_mut() {
            dot.set_next_frame(&res, delta_time)
        }
        self.write_dots_to_grid();
    }

    pub fn spawn_dots(
        &mut self,
        cursor_position: &PhysicalPosition<f64>,
        window_resolution: &PhysicalSize<u32>,
    ) {
        if self.last_dot_spawned.elapsed() >= DELAY_BETWEEN_DOTS {
            match self.physical_position_to_game_coordinates(cursor_position, window_resolution) {
                CoordConversion::Converted(coord) => {
                    self.dots.push(Dot::new(Material::Dirt, coord, Vec2::new(0., 0.)));
                    self.last_dot_spawned = Instant::now();
                },
                CoordConversion::OutOfBounds => println!("WARNING! Clicked outside of game space"),
            }
        }
    }

    pub fn physical_position_to_game_coordinates(
        &self,
        physical_position: &PhysicalPosition<f64>,
        window_resolution: &PhysicalSize<u32>,
    ) -> CoordConversion<Vec2<f64>> {
        let win_res = Vec2::new(
            window_resolution.width as f64,
            window_resolution.height as f64,
        );
        let can_res: Vec2<f64> = self.resolution().into();
        let win_ar = win_res.x / win_res.y;
        let can_ar = can_res.x / can_res.y;

        let corrected_win_res = if win_ar > can_ar {
            Vec2::new(win_res.y / can_ar, win_res.y)
        } else {
            Vec2::new(win_res.x, win_res.x * can_ar)
        };
        let offset = (win_res - corrected_win_res) / 2.;
        let corrected_position = Vec2::from(physical_position) - offset;

        let game_coord = corrected_position * can_res / corrected_win_res;
        if game_coord.x < 0.
            || game_coord.y < 0.
            || game_coord.x > can_res.x
            || game_coord.y > can_res.y
        {
            CoordConversion::OutOfBounds
        } else {
            CoordConversion::Converted(game_coord)
        }
    }

    fn write_dots_to_grid(&mut self) {
        for material in self.grid.iter_mut().flatten() {
            *material = Material::EmptySpace;
        }
        for dot in self.dots.iter() {
            let row = dot.position.y.clone().round() as usize;
            let col = dot.position.x.clone().round() as usize;
            self.grid[row][col] = dot.material;
        }
    }
}
