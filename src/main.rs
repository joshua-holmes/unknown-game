mod windowing;
mod geometry;
mod rendering;
mod game;

fn main() {
    let game_state = game::init_game();
    windowing::init_window(game_state);
}
