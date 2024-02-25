use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window};

mod event_handler;
mod state;

use crate::rendering::render_engine::RenderEngine;

use self::state::WindowState;

use super::game::state::GameState;

pub fn init() -> WindowState {
    let event_loop = EventLoop::new();
    let window = Arc::new(Window::new(&event_loop).unwrap());

    WindowState { event_loop, window }
}

pub fn run_game_loop(
    window_state: WindowState,
    mut render_engine: RenderEngine,
    mut game_state: GameState,
) {
    window_state.event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        event_handler::handle_event(
            event,
            control_flow,
            window_state.window.clone(),
            &mut render_engine,
            &mut game_state,
        );
    });
}
