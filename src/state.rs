use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::string::FromUtf8Error;
use std::{fs, io};

use crate::grid::Grid;
use crate::scenes::Game;
use crate::scenes::game::default_zoom;

pub const GAMEDATA_DIR: &str = ".gamedata/";
pub const GRIDS_DIR_NAME: &str = "grids/";
pub const GAME_STATE_FILE: &str = "state.json";

#[derive(Debug)]
#[allow(dead_code)]
pub enum ReadingError {
    Io(io::Error),
    Utf8(FromUtf8Error),
}

impl From<io::Error> for ReadingError {
    fn from(value: io::Error) -> Self {
        ReadingError::Io(value)
    }
}

impl From<FromUtf8Error> for ReadingError {
    fn from(value: FromUtf8Error) -> Self {
        ReadingError::Utf8(value)
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub grid: Grid,
    pub camera_zoom_x: f32,
    pub camera_zoom_y: f32,
    pub camera_target_x: f32,
    pub camera_target_y: f32,
}

impl GameState {
    fn read(path: &str) -> Result<String, ReadingError> {
        let bytes = fs::read(path)?;
        Ok(String::from_utf8(bytes)?)
    }

    pub fn load(path: Option<String>) -> (Self, bool) {
        if !fs::exists(GAMEDATA_DIR).is_ok_and(|e| e) {
            fs::create_dir(GAMEDATA_DIR).expect(&format!("Failed to create {GAMEDATA_DIR} directory"));
        }
        let default_state = {
            let zoom = default_zoom();
            GameState {
                grid: Grid::new(),
                camera_zoom_x: zoom.x,
                camera_zoom_y: zoom.y,
                camera_target_x: 0.0,
                camera_target_y: 0.0,
            }
        };
        let default_path = GAMEDATA_DIR.to_string() + GAME_STATE_FILE;

        let state_json = GameState::read(&path.unwrap_or(default_path));
        if state_json.is_err() {
            return (default_state, false);
        }
        (
            from_str::<GameState>(&state_json.unwrap()).unwrap_or(default_state),
            true,
        )
    }

    pub fn save(&self) {
        let state_json = to_string(self).expect("Failed to dump game state");
        let path = GAMEDATA_DIR.to_string() + GAME_STATE_FILE;
        fs::write(path, state_json).expect("Failed to save game state to a file");
    }

    pub fn from_game(game: &Game) -> Self {
        GameState {
            grid: game.grid.clone(),
            camera_zoom_x: game.camera.zoom.x,
            camera_zoom_y: game.camera.zoom.y,
            camera_target_x: game.camera.target.x,
            camera_target_y: game.camera.target.y,
        }
    }
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.camera_target_x == other.camera_target_x
            && self.camera_target_y == other.camera_target_y
            && self.camera_zoom_x == other.camera_zoom_x
            && self.camera_zoom_y == other.camera_zoom_y
    }
}
