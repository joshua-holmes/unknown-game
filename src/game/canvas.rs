use std::time::Duration;

use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::rendering::glsl_types::Resolution;

use super::{dot::Dot, geometry::Vec2, material::Material};

pub struct Canvas {
    pub grid: Vec<Vec<Material>>,
    pub dots: Vec<Dot>,
}
impl Canvas {
    pub fn new(resolution: PhysicalSize<u32>) -> Self {
        let grid = (0..resolution.height)
            .map(|y| {
                (0..resolution.width)
                    .map(|x| Material::EmptySpace)
                    .collect()
            })
            .collect();
        Self {
            grid,
            dots: Vec::with_capacity((resolution.height * resolution.width) as usize),
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
        self.physical_position_to_game_coordinates(cursor_position, window_resolution);
    }

    pub fn physical_position_to_game_coordinates(
        &self,
        physical_position: &PhysicalPosition<f64>,
        window_resolution: &PhysicalSize<u32>,
    ) -> Option<Vec2<f64>> {
        let win_res = Vec2::new(
            window_resolution.width as f64,
            window_resolution.height as f64,
        );
        let can_res: Vec2<f64> = self.resolution().into();
        let win_ar = win_res.x / win_res.y;
        let can_ar = can_res.x / can_res.y;

        let corrected_win_res = if win_ar > can_ar {
            Vec2::new(win_res.y / can_ar, 0.)
        } else {
            Vec2::new(0., win_res.x * can_ar)
        };

        let offset = win_res - corrected_win_res;
        let position = Vec2::from(physical_position) - offset;

        println!("position {:?}", position);
        if position.x < 0.
            || position.y < 0.
            || position.x > corrected_win_res.x
            || position.y > corrected_win_res.y
        {
            None
        } else {
            Some(position * can_res / win_res)
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
