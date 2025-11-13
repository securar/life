use std::{env, fs};

use life::scene::Scene;
use life::scenes::Game;
use life::scenes::main_menu::MainMenu;
use life::state::GameState;

use macroquad::prelude::*;
use resolution::current_resolution;

fn window_conf() -> Conf {
    let (width, height) = {
        match current_resolution() {
            Ok((w, h)) if !cfg!(debug_assertions) => (w, h),
            _ => (1280, 720),
        }
    };
    Conf {
        window_title: "Conway's Game Of Life".to_string(),
        window_width: width,
        window_height: height,
        window_resizable: false,
        fullscreen: !cfg!(debug_assertions),
        ..Default::default()
    }
}

async fn draw_transition(current_scene: Box<&mut dyn Scene>, next_scene: Option<Box<&mut dyn Scene>>) {
    let color = BLACK;
    let mut alpha = 0.0;
    let step = 0.04;
    let end = (1.0 / step) as i32;

    for _ in 0..end {
        current_scene.update();
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), color.with_alpha(alpha));

        alpha += step;
        next_frame().await;
    }

    if next_scene.is_none() {
        return;
    }
    let next_scene = next_scene.unwrap();

    let mut alpha = 1.0;
    for _ in 0..end {
        next_scene.update();
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), color.with_alpha(alpha));

        alpha -= step;
        next_frame().await;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let state_path = match env::args().nth(1) {
        Some(path) if fs::exists(&path).unwrap_or(false) => Some(path),
        _ => None,
    };

    let mut main_menu = MainMenu::new();

    let (state, is_continue) = GameState::load(state_path);
    let mut game = Game::new(state);

    if is_continue {
        main_menu.start_button.text = "Continue";
    }
    loop {
        if main_menu.start_button.is_released() {
            game.is_running = true;

            draw_transition(Box::new(&mut main_menu), Some(Box::new(&mut game))).await;
            while game.is_running {
                game.update();
                next_frame().await;
            }
            main_menu.start_button.text = "Continue";
            draw_transition(Box::new(&mut game), Some(Box::new(&mut main_menu))).await;
        }

        if main_menu.quit_button.is_released() {
            game.save_state();

            draw_transition(Box::new(&mut main_menu), None).await;
            return;
        }

        main_menu.update();
        next_frame().await;
    }
}
