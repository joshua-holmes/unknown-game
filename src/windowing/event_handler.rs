use winit::{event::{Event, WindowEvent}, event_loop::ControlFlow};

use crate::{rendering::RenderEngine, game::state::GameState};

pub fn handle_event(event: Event<()>, control_flow: &mut ControlFlow, render_engine: &mut RenderEngine, game_state: &mut GameState) {
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
            render_engine.recreate_swapchain_and_resize_window();
        }
        Event::MainEventsCleared => {
            render_engine.display_next_frame(&mut game_state.canvas);
        }
        _ => (),
    }
}
