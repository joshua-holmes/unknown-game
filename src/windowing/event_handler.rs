use std::sync::Arc;

use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use crate::{game::state::GameState, rendering::render_engine::RenderEngine};

use super::state::WindowState;

pub fn handle_event(
    event: Event<()>,
    control_flow: &mut ControlFlow,
    window: Arc<Window>,
    render_engine: &mut RenderEngine,
    game_state: &mut GameState,
    window_state: &mut WindowState,
) {
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
        Event::WindowEvent {
            event: WindowEvent::CursorMoved { position, .. },
            ..
        } => {
            window_state.cursor_position = position;
        }
        Event::WindowEvent {
            event:
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                    ..
                },
            ..
        } => {
            game_state.canvas.spawn_dots(
                &window_state.cursor_position,
                &window_state.window.inner_size(),
            );
        }
        Event::MainEventsCleared => {
            game_state.set_time();
            game_state.canvas.set_next_frame(&game_state.delta_time);
            render_engine.display_next_frame(&mut game_state.canvas, window.clone());
        }
        _ => (),
    }
}
