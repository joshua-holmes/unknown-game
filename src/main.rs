mod windowing;
mod rendering;
mod game;

fn main() {
    let game_state = game::init();
    let window_state = windowing::init();
    let render_engine = rendering::init(
        &window_state.event_loop,
        window_state.window.clone(),
        &game_state.canvas
    );

    windowing::run_game_loop(window_state, render_engine, game_state);
}
