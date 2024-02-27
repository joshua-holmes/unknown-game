use winit::event_loop::EventLoop;

mod game;
mod rendering;
mod windowing;

fn main() {
    let game_state = game::state::GameState::new();
    let event_loop = EventLoop::new();
    let window_state = windowing::state::WindowState::new(&event_loop);
    let render_engine = rendering::render_engine::RenderEngine::new(
        &event_loop,
        window_state.window.clone(),
        &game_state.canvas,
    );

    windowing::run_game_loop(event_loop, window_state, render_engine, game_state);
}
