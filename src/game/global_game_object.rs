use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::rendering::glsl_types::Resolution;

use super::{
    canvas::{Canvas, CanvasError},
    dot::{Dot, DotModification},
    enums::CoordConversion,
    geometry::Vec2,
    id_generator::{Id, IdGenerator},
    material::Material,
    CURSOR_SIZE, DELAY_BETWEEN_DOTS, INITIAL_CANVAS_RESOLUTION,
};

pub struct Game {
    pub delta_time: Duration,
    pub canvas: Canvas,
    pub palette: HashMap<Id, Dot>,
    pub last_dot_spawned: Instant,
    pub resolution: Resolution,
    last_frame_time: Instant,
    dot_id_generator: IdGenerator,
}
impl Game {
    pub fn new() -> Self {
        let Resolution { height, width } = INITIAL_CANVAS_RESOLUTION;
        let canvas = Canvas::new(INITIAL_CANVAS_RESOLUTION);

        let mut dot_id_generator = IdGenerator::new();

        // Set some dots for testing
        let mut palette = HashMap::with_capacity((height * width) as usize);

        // for i in 0..10 {
        //     for j in (0 + i)..(50 - i) {
        //         let dot = Dot::new(
        //             &mut dot_id_generator,
        //             Material::Water,
        //             Vec2::new(100. + j as f64, 498. - i as f64),
        //             Vec2::new(0., 0.),
        //         );
        //         palette.insert(dot.id, dot);
        //     }
        // }
        //
        let dot = Dot::new(
            &mut dot_id_generator,
            Material::Water,
            Vec2::new(100., 0.),
            Vec2::new(0., 100.),
        );
        palette.insert(dot.id, dot);
        let dot = Dot::new(
            &mut dot_id_generator,
            Material::Dirt,
            Vec2::new(100., 499.),
            Vec2::new(0., -200.),
        );
        palette.insert(dot.id, dot);

        let mut game = Self {
            canvas,
            palette,
            resolution: INITIAL_CANVAS_RESOLUTION,
            delta_time: Duration::ZERO,
            last_frame_time: Instant::now(),
            last_dot_spawned: Instant::now(),
            dot_id_generator,
        };

        game.write_dots_to_grid();

        game
    }

    pub fn set_time(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last_frame_time;
        self.last_frame_time = now;
    }

    pub fn set_next_frame(&mut self, delta_time: Duration) {
        // find velocity
        for dot in self.palette.values_mut() {
            dot.velocity = dot.find_next_velocity(delta_time);
        }

        // find position
        let mut dots_to_modify = Vec::new();
        for dot in self.palette.values_mut() {
            let offset_from_drag = dot.find_pos_offset_from_drag();
            let next_pos = dot.find_next_position(self.resolution, self.delta_time, offset_from_drag);
            if next_pos.to_rounded_isize() != dot.position.to_rounded_isize() {
                let collision_check = self.canvas.check_for_dot_collision(&dot, next_pos);
                if let Some(collided_dots) = collision_check {
                    dots_to_modify.push(collided_dots.this);
                    if let Some(other) = collided_dots.other {
                        dots_to_modify.push(other);
                    }
                    continue;
                }
            }
            dots_to_modify.push(DotModification {
                id: dot.id,
                delta_velocity: None,
                delta_position: Some(next_pos - dot.position),
            });
        }

        for dot_to_modify in dots_to_modify {
            let dot = self.palette.get_mut(&dot_to_modify.id).unwrap();
            if let Some(next_vel) = dot_to_modify.delta_velocity {
                dot.velocity += next_vel;
            }
            if let Some(next_pos) = dot_to_modify.delta_position {
                dot.position += next_pos;
            }
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
                    let radius = CURSOR_SIZE;
                    let top_left = (coord - radius)
                        .clamp_to_resolution(self.canvas.resolution)
                        .to_rounded_usize();
                    let bottom_right = (coord + radius)
                        .clamp_to_resolution(self.canvas.resolution)
                        .to_rounded_usize();
                    for x in top_left.x..=bottom_right.x {
                        for y in top_left.y..=bottom_right.y {
                            let point = Vec2::new(x, y);
                            let distance = (point.into_f64() - coord).pythagorean_theorem();

                            // dot can only spawn within radius of CURSOR_SIZE
                            if distance > radius {
                                continue;
                            }

                            // dot can't spawn if another dot is already there
                            if self.canvas.get(point).unwrap().is_some() {
                                continue;
                            }

                            // add new dot to palette, error if dot is already there
                            let new_dot = Dot::new(
                                &mut self.dot_id_generator,
                                Material::Sand,
                                point.into_f64(),
                                Vec2::new(0., 0.),
                            );
                            if let Some(old_dot) = self.palette.insert(new_dot.id, new_dot) {
                                panic!("Dot already exists in palette. Old dot:\n{:?}\nwas replaced with new dot:\n{:?}", old_dot, new_dot);
                            }
                        }
                    }
                    self.last_dot_spawned = Instant::now();
                }
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
        self.canvas.clear();
        for dot in self.palette.values_mut() {
            match self.canvas.set(dot.position.clamp_to_resolution(self.resolution).to_rounded_usize(), Some(dot.into())) {
                Err(CanvasError::CoordOutOfBounds) => println!(
                    "WARNING: Tried to write a dot to canvas that was out of bounds:\n{:?}",
                    dot
                ),
                Ok(_) => {}
            }
        }
    }
}
