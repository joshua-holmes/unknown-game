use winit::event_loop::EventLoop;

mod event_handler;
pub mod state;

use crate::rendering::render_engine::RenderEngine;

use self::state::WindowState;

use super::game::Game;

pub fn run_game_loop(
    event_loop: EventLoop<()>,
    mut window_state: WindowState,
    mut render_engine: RenderEngine,
    mut game: Game,
) {
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        event_handler::handle_event(
            event,
            control_flow,
            window_state.window.clone(),
            &mut render_engine,
            &mut game,
            &mut window_state,
        );
    });
}
