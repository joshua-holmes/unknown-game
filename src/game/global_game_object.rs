use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::rendering::glsl_types::Resolution;

use super::{
    dot::Dot,
    enums::CoordConversion,
    geometry::Vec2,
    id_generator::{Id, IdGenerator},
    material::Material,
    DELAY_BETWEEN_DOTS, INITIAL_CANVAS_RESOLUTION,
};

pub struct Game {
    pub delta_time: Duration,
    pub canvas: Vec<Vec<Option<Dot>>>,
    pub palette: HashMap<Id, Dot>,
    pub last_dot_spawned: Instant,
    pub resolution: Resolution,
    last_frame_time: Instant,
    dot_id_generator: IdGenerator,
}
impl Game {
    pub fn new() -> Self {
        let Resolution { height, width } = INITIAL_CANVAS_RESOLUTION;
        let canvas = (0..height)
            .map(|_| (0..width).map(|_| None).collect())
            .collect();

        let mut dot_id_generator = IdGenerator::new();

        let dot = Dot::new(
            &mut dot_id_generator,
            Material::Sand,
            Vec2::new(250., 0.),
            Vec2::new(0., 100.),
        );
        let dot2 = Dot::new(
            &mut dot_id_generator,
            Material::Dirt,
            Vec2::new(250., 130.),
            Vec2::new(0., 50.),
        );

        // Set some dots for testing
        let mut palette = HashMap::with_capacity((height * width) as usize);
        palette.insert(dot.id, dot);
        palette.insert(dot2.id, dot2);

        Self {
            canvas,
            palette,
            resolution: INITIAL_CANVAS_RESOLUTION,
            delta_time: Duration::ZERO,
            last_frame_time: Instant::now(),
            last_dot_spawned: Instant::now(),
            dot_id_generator,
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
        let mut dots_to_modify = Vec::new();
        for dot in self.palette.values_mut() {
            dot.set_next_position(&self.resolution, &self.delta_time);
            let collision_check = dot.check_for_dot_collision(&mut self.canvas);
            if let Some(collided_dots) = collision_check {
                dots_to_modify.push(collided_dots.0);
                dots_to_modify.push(collided_dots.1);
            }
        }

        for dot_to_modify in dots_to_modify {
            let dot = self.palette.get_mut(&dot_to_modify.id).unwrap();
            dot.velocity = dot_to_modify.next_velocity;
            if let Some(next_pos) = dot_to_modify.next_position {
                dot.next_position = Some(next_pos);
            }
        }

        for dot in self.palette.values_mut() {
            dot.set_next_frame(&self.resolution, delta_time);
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
                    if self.canvas[coord.y.round() as usize][coord.x.round() as usize].is_none() {
                        let new_dot = Dot::new(
                            &mut self.dot_id_generator,
                            Material::Sand,
                            coord,
                            Vec2::new(0., 0.),
                        );
                        if let Some(old_dot) = self.palette.insert(new_dot.id, new_dot) {
                            panic!("Dot already exists in palette. Old dot:\n{:?}\nwas replaced with new dot:\n{:?}", old_dot, new_dot);
                        }
                        self.last_dot_spawned = Instant::now();
                    }
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
        for dot in self.palette.values_mut() {
            let row = dot.position.y.clone().round() as usize;
            let col = dot.position.x.clone().round() as usize;
            self.canvas[row][col] = Some(*dot);
        }
    }
}
