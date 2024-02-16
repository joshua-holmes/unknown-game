mod windowing;
mod rendering;
mod game;

fn main() {
    let game = game::init();
    let game_window = windowing::init();
    let render_engine = rendering::init(
        &game_window.event_loop,
        game_window.window.clone(),
        &game.canvas
    );

    windowing::run_game_loop(game_window, render_engine, game);
}
