use std::time::{Duration, Instant};

use winit::dpi::PhysicalSize;

use crate::rendering::glsl_types::Resolution;

use super::{dot::Dot, geometry::Vec2};

pub struct Canvas {
    pub grid: Vec<Vec<Option<Dot>>>,
    start_time: Option<Instant>,
    old_velocity: Vec2<f64>,
    pub check: Check,
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
            start_time: None,
            old_velocity: Vec2::new(0., 0.),
            check: Check { time_sum: 0., vel_sum: 0. }
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

                    let now = Instant::now();
                    if let Some(start_time) = self.start_time.as_mut() {
                        let delta = now - *start_time;
                        if delta.as_secs_f64() >= 1. {
                            println!("{:?} | {:?}", dot.velocity.y - self.old_velocity.y, delta);
                            println!("{:?}\n", self.check);
                            *start_time = now;
                            self.old_velocity = dot.velocity;
                            self.check.vel_sum = 0.;
                            self.check.time_sum = 0.;
                        }
                    } else {
                        self.start_time = Some(now);
                    }
                    dot.set_next_frame(&res, &delta_time, &mut self.check);

                    let new_row = dot.position.y.clone().round() as usize;
                    let new_col = dot.position.x.clone().round() as usize;
                    self.grid[new_row][new_col] = Some(dot);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Check {
    pub time_sum: f64,
    pub vel_sum: f64,
}
