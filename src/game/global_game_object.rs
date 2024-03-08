use std::time::{Duration, Instant};

use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::rendering::glsl_types::Resolution;

use super::{
    dot::Dot, enums::CoordConversion, geometry::Vec2, material::Material, DELAY_BETWEEN_DOTS,
    INITIAL_CANVAS_RESOLUTION,
};

pub struct Game {
    pub delta_time: Duration,
    pub canvas: Vec<Vec<Option<Dot>>>,
    pub palette: Vec<Dot>,
    pub last_dot_spawned: Instant,
    pub resolution: Resolution,
    last_frame_time: Instant,
}
impl Game {
    pub fn new() -> Self {
        let Resolution { height, width } = INITIAL_CANVAS_RESOLUTION;
        let canvas = (0..height)
            .map(|_| (0..width).map(|_| None).collect())
            .collect();

        // Set some dots for testing
        let mut palette = Vec::with_capacity((height * width) as usize);
        palette.push(Dot::new(
            Material::Sand,
            Vec2::new(0., 0.),
            Vec2::new(100., 0.),
        ));

        Self {
            canvas,
            palette,
            resolution: INITIAL_CANVAS_RESOLUTION,
            delta_time: Duration::ZERO,
            last_frame_time: Instant::now(),
            last_dot_spawned: Instant::now(),
        }
    }

    pub fn set_time(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last_frame_time;
        self.last_frame_time = now;
    }

    pub fn iter_materials_as_bytes<'a>(&'a self) -> impl Iterator<Item = u8> + 'a {
        self.canvas
            .iter()
            .flatten()
            .map(|maybe_dot| maybe_dot.map_or(Material::EmptySpace as u8, |dot| dot.material as u8))
    }

    pub fn set_next_frame(&mut self, delta_time: &Duration) {
        for dot in self.palette.iter_mut() {
            dot.set_next_frame(&self.resolution, delta_time)
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
                    self.palette
                        .push(Dot::new(Material::Sand, coord, Vec2::new(0., 0.)));
                    self.last_dot_spawned = Instant::now();
                }
                CoordConversion::OutOfBounds => println!("WARNING! Clicked outside of game space"),
            }
        }
    }

    pub fn clear_canvas(&mut self) {
        for maybe_dot in self.canvas.iter_mut().flatten() {
            *maybe_dot = None;
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
        let can_res: Vec2<f64> = self.resolution.into();
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
        self.clear_canvas();
        for dot in self.palette.iter() {
            let row = dot.position.y.clone().round() as usize;
            let col = dot.position.x.clone().round() as usize;
            self.canvas[row][col] = Some(*dot);
        }
    }
}
