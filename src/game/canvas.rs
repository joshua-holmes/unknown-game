use std::time::Duration;

use winit::dpi::PhysicalSize;

use crate::rendering::glsl_types::Resolution;

use super::{dot::Dot, geometry::Vec2};

pub struct Canvas {
    pub grid: Vec<Vec<Option<Dot>>>,
}
#[allow(dead_code)]
impl Canvas {
    pub fn new(resolution: PhysicalSize<u32>) -> Self {
        let grid = (0..resolution.height).map(|y| {
            (0..resolution.width).map(|x| {
                if y == 0 {
                    Some(Dot::new(1, Vec2::new(x as f64, y as f64)))
                } else {
                    None
                }
            }).collect()
        }).collect();
        Self {
            grid,
        }
    }

    pub fn iter_materials<'a>(&'a self) -> impl Iterator<Item = u8> + 'a {
        self.grid.iter().flatten().map(|d| {
            match d {
                Some(dot) => dot.material,
                None => 0,
            }
        })
    }

    pub fn resolution(&self) -> Resolution {
        Resolution {
            height: self.grid.len() as i32,
            width: self.grid[0].len() as i32
        }
    }

    pub fn set_next_frame(&mut self, delta_time: &Duration) {
        let res = self.resolution();
        for row in 0..res.height as usize {
            for col in 0..res.width as usize {
                if let Some(mut dot) = self.grid[row][col].take() {
                    dot.set_next_frame(&res, &delta_time);

                    let new_row = dot.position.y.clone().round() as usize;
                    let new_col = dot.position.x.clone().round() as usize;
                    self.grid[new_row][new_col] = Some(dot);
                }
            }
        }
    }
}

