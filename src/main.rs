use std::{env, fs};

use macroquad::prelude::Conf;

use life::game::Game;
use life::gamedata::GameState;

// start in fullscreen when not debugging
const START_FULLSCREEN: bool = !cfg!(debug_assertions);

fn window_conf() -> Conf {
    Conf {
        window_title: "Conway's Game Of Life".to_string(),
        window_width: 1200,
        window_height: 700,
        window_resizable: false,
        fullscreen: START_FULLSCREEN,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let grid_path = match env::args().nth(1) {
        Some(path) if fs::exists(&path).unwrap_or(false) => Some(path),
        _ => None,
    };
    let mut game = Game::new(START_FULLSCREEN, GameState::load(grid_path));

    while game.is_running {
        game.handle_controls();
        game.draw();
        game.update().await;
    }
}
