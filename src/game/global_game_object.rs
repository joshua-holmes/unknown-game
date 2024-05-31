use std::time::{Duration, Instant};

use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::windowing::state::MouseState;

use super::{
    canvas::Canvas,
    material::Material,
    Vec2, CURSOR_SIZE, DELAY_BETWEEN_DOTS, INITIAL_CANVAS_RESOLUTION,
};

pub enum CoordConversion<T> {
    Converted(T),
    OutOfBounds,
}

pub struct Game {
    pub delta_time: Duration,
    pub canvas: Canvas,
    pub last_dot_spawned: Instant,
    last_frame_time: Instant,
    frame_count: u128,
}
impl Game {
    pub fn new() -> Self {
        let mut canvas = Canvas::new(INITIAL_CANVAS_RESOLUTION);

        canvas.spawn_dot(Material::Blue, Vec2::new(100., 100.), Vec2::new(50., -50.));
        canvas.spawn_dot(Material::Orange, Vec2::new(300., 167.), Vec2::new(-100., -100.));

        let mut game = Self {
            canvas,
            delta_time: Duration::ZERO,
            last_frame_time: Instant::now(),
            last_dot_spawned: Instant::now(),
            frame_count: 0,
        };

        game.canvas.write_dots_to_grid();

        game
    }

    pub fn set_time(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last_frame_time;
        self.last_frame_time = now;
    }

    pub fn set_next_frame(&mut self, delta_time: Duration) {
        self.canvas.calculate_physics(delta_time);
        self.canvas.write_dots_to_grid();
        self.frame_count += 1;
    }

    pub fn handle_spawn_dots(
        &mut self,
        cursor_position: &PhysicalPosition<f64>,
        window_resolution: &PhysicalSize<u32>,
        mouse_state: &MouseState,
    ) {
        if self.last_dot_spawned.elapsed() < DELAY_BETWEEN_DOTS {
            return;
        }
        let material = match mouse_state {
            MouseState::Released => return,
            MouseState::LeftPressed => Material::Orange,
            MouseState::RightPressed => Material::Blue,
        };
        match self.physical_position_to_game_coordinates(cursor_position, window_resolution) {
            CoordConversion::Converted(coord) => {
                self.canvas.spawn_circle_of_dots(CURSOR_SIZE, coord, material);
                self.last_dot_spawned = Instant::now();
            }
            CoordConversion::OutOfBounds => println!("WARNING! Clicked outside of game space"),
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
        let can_res: Vec2<f64> = self.canvas.resolution.into();
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

}
