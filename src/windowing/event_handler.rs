use std::sync::Arc;

use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use crate::{game::Game, rendering::render_engine::RenderEngine};

use super::state::WindowState;

pub fn handle_event(
    event: Event<()>,
    control_flow: &mut ControlFlow,
    window: Arc<Window>,
    render_engine: &mut RenderEngine,
    main_game_obj: &mut Game,
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
                    state,
                    ..
                },
            ..
        } => {
            window_state.left_mouse_btn = state;
        }
        Event::MainEventsCleared => {
            if let ElementState::Pressed = window_state.left_mouse_btn {
                main_game_obj.canvas.spawn_dots(
                    &window_state.cursor_position,
                    &window_state.window.inner_size(),
                );
            }
            main_game_obj.set_time();
            main_game_obj.canvas.set_next_frame(&main_game_obj.delta_time);
            render_engine.display_next_frame(&mut main_game_obj.canvas, window.clone());
        }
        _ => (),
    }
}
