use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window, error::OsError};

mod event_handler;

use crate::game::state::GameState;

use super::rendering::RenderEngine;

enum WindowingError {
    OsError(OsError),
}
impl From<OsError> for WindowingError {
    fn from(os_error: OsError) -> Self {
        WindowingError::OsError(os_error)
    }
}

pub fn init_window(mut game_state: GameState) {
    let event_loop = EventLoop::new();
    let window = {
        let window = Window::new(&event_loop).unwrap();
        Arc::new(window)
    };

    let mut render_engine = RenderEngine::new(&event_loop, window.clone(), &game_state.canvas);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        event_handler::handle_event(event, control_flow, &mut render_engine, &mut game_state); // TODO: handle error appropriately
    });
}
