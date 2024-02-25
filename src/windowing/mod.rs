use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window, dpi::PhysicalPosition};

mod event_handler;
mod state;

use crate::rendering::render_engine::RenderEngine;

use self::state::WindowState;

use super::game::state::GameState;

pub fn init(event_loop: &EventLoop<()>) -> WindowState {
    let window = Arc::new(Window::new(event_loop).unwrap());

    WindowState { window, cursor_position: PhysicalPosition::new(0., 0.) }
}

pub fn run_game_loop(
    event_loop: EventLoop<()>,
    mut window_state: WindowState,
    mut render_engine: RenderEngine,
    mut game_state: GameState,
) {
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        event_handler::handle_event(
            event,
            control_flow,
            window_state.window.clone(),
            &mut render_engine,
            &mut game_state,
            &mut window_state,
        );
    });
}
