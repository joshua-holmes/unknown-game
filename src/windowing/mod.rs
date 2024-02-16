use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window};

mod event_handler;
mod game_window;

use crate::rendering::render_engine::RenderEngine;

use self::game_window::GameWindow;

use super::game::state::GameState;

pub fn init() -> GameWindow {
    let event_loop = EventLoop::new();
    let window = Arc::new(Window::new(&event_loop).unwrap());

    GameWindow {
        event_loop,
        window,
    }
}

pub fn run_game_loop(event_loop: EventLoop<()>, mut render_engine: RenderEngine, mut game_state: GameState) {
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        event_handler::handle_event(event, control_flow, &mut render_engine, &mut game_state); // TODO: handle error appropriately
    });
}
