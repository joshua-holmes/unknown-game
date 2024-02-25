use std::time::Duration;

use winit::dpi::PhysicalSize;

use crate::rendering::glsl_types::Resolution;

use super::{dot::Dot, material::Material};

pub struct Canvas {
    pub grid: Vec<Vec<Material>>,
    pub dots: Vec<Dot>,
}
impl Canvas {
    pub fn new(resolution: PhysicalSize<u32>) -> Self {
        let grid = (0..resolution.height).map(|y| {
            (0..resolution.width).map(|x| 
                Material::EmptySpace
            ).collect()
        }).collect();
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
            width: self.grid[0].len() as i32
        }
    }

    pub fn set_next_frame(&mut self, delta_time: &Duration) {
        let res = self.resolution();
        for dot in self.dots.iter_mut() {
            dot.set_next_frame(&res, delta_time)
        }
        self.write_dots_to_grid();
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
