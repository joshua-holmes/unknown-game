use std::sync::Arc;

use winit::{event::{Event, WindowEvent}, event_loop::ControlFlow, window::Window};

use crate::{rendering::render_engine::RenderEngine, game::state::GameState};

pub fn handle_event(event: Event<()>, control_flow: &mut ControlFlow, window: Arc<Window>, render_engine: &mut RenderEngine, game_state: &mut GameState) {
    match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            println!("User requested window to be closed");
            control_flow.set_exit();
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            render_engine.recreate_swapchain_and_resize_window(window.clone());
        }
        Event::MainEventsCleared => {
            game_state.set_time();
            game_state.canvas.set_next_frame(&game_state.delta_time);
            render_engine.display_next_frame(&mut game_state.canvas, window.clone());
        }
        _ => (),
    }
}
