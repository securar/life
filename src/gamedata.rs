use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::string::FromUtf8Error;
use std::{fs, io};

use crate::game::default_zoom;
use crate::grid::Grid;

pub const GAMEDATA_DIR: &str = ".gamedata/";
pub const GRIDS_DIR_NAME: &str = "grids/";
pub const GAME_STATE_FILE: &str = "state.json";

#[derive(Debug)]
#[allow(dead_code)]
pub enum ReadingError {
    Io(io::Error),
    Utf8(FromUtf8Error),
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub grid: Grid,
    pub camera_zoom_x: f32,
    pub camera_zoom_y: f32,
    pub camera_target_x: f32,
    pub camera_target_y: f32,
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

impl GameState {
    fn read(path: &str) -> Result<String, ReadingError> {
        let bytes = fs::read(path)?;
        return Ok(String::from_utf8(bytes)?);
    }

    pub fn load(path: Option<String>) -> Self {
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
        if !state_json.is_ok() {
            return default_state;
        }
        return from_str::<GameState>(&state_json.unwrap()).unwrap_or(default_state);
    }

    pub fn save(&self) {
        let state_json = to_string(self).expect("Failed to dump game state");
        let path = GAMEDATA_DIR.to_string() + GAME_STATE_FILE;
        fs::write(path, state_json).expect("Failed to save game state to a file");
    }
}
