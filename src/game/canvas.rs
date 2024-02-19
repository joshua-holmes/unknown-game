use winit::dpi::PhysicalSize;

use crate::rendering::glsl_types::{Dot, Resolution};

pub struct Canvas {
    pub grid: Vec<Vec<Dot>>,
}
#[allow(dead_code)]
impl Canvas {
    pub fn new(resolution: PhysicalSize<u32>) -> Self {
        let grid = (0..resolution.height).map(|_| {
            (0..resolution.width).map(|_| {
                Dot::new(2)
            }).collect()
        }).collect();
        Self {
            grid,
        }
    }

    pub fn iter_materials<'a>(&'a self) -> impl Iterator<Item = u32> + 'a {
        self.grid.iter().flatten().map(|d| d.material)
    }

    pub fn resolution(&self) -> Resolution {
        Resolution {
            height: self.grid.len() as i32,
            width: self.grid[0].len() as i32
        }
    }
}

