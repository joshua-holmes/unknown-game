mod windowing;
mod geometry;
mod rendering;
mod game;

fn main() {
    let game_state = game::init();
    let game_window = windowing::init();
    let render_engine = rendering::init(
        &game_window.event_loop,
        game_window.window.clone(),
        &game_state.canvas
    );

    windowing::run_game_loop(game_window, render_engine, game_state);
}
