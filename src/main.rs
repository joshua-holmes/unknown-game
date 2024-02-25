use winit::event_loop::EventLoop;

mod windowing;
mod rendering;
mod game;

fn main() {
    let game_state = game::init();
    let event_loop = EventLoop::new();
    let window_state = windowing::init(&event_loop);
    let render_engine = rendering::init(
        &event_loop,
        window_state.window.clone(),
        &game_state.canvas
    );

    windowing::run_game_loop(event_loop, window_state, render_engine, game_state);
}
