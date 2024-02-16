use winit::dpi::PhysicalSize;

use crate::rendering::glsl_types::{Dot, Resolution};

pub struct Canvas {
    pub grid: Vec<Vec<Dot>>,
}
#[allow(dead_code)]
impl Canvas {
    pub fn new(resolution: PhysicalSize<u32>) -> Self {
        let grid = (0..resolution.height).map(|j| {
            (0..resolution.width).map(|i| {
                Dot { dot_value: if (i + j % 2) % 2 == 0 { 9 } else { 0 } }
                // Dot { dot_value: 9 }
            }).collect()
        }).collect();
        Self {
            grid,
        }
    }

    pub fn to_vec_of_dots(&self) -> Vec<Dot> {
        self.grid.iter().flatten().cloned().collect()
    }

    pub fn resolution(&self) -> Resolution {
        Resolution {
            height: self.grid.len() as i32,
            width: self.grid[0].len() as i32
        }
    }
}

