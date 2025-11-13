use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::clock::Clock;
use crate::grid::{CellPos, Grid};
use crate::scene::Scene;
use crate::state::GameState;
use crate::util::naive_dedent;

const CELL_SIZE: f32 = 1.0;

const BG_COLOR: Color = Color::new(0.04, 0.04, 0.04, 1.00);
const HOVERED_COLOR: Color = DARKGRAY;
const ALIVE_COLOR: Color = Color::new(0.95, 0.95, 0.95, 1.00);
const MESH_COLOR: Color = Color::new(0.12, 0.12, 0.12, 1.00);

const MIN_ZOOM_X: f32 = 0.00002;
const MAX_ZOOM_X: f32 = 1.0;
const DEFAULT_ZOOM_X: f32 = 0.03;
// amount the camera will move using the keyboard
const KB_MOVE_AMOUNT: f32 = 0.05;

const TEXT_SIDE_PAD: f32 = 10.0;

const FPS_CLOCK_INTERVAL: f64 = 0.25;
const GRID_CLOCK_INTERVAL: f64 = 0.15;

enum VisualState {
    Alive,
    Hovered,
}

pub fn default_zoom() -> Vec2 {
    vec2(DEFAULT_ZOOM_X, DEFAULT_ZOOM_X * screen_width() / screen_height())
}

fn default_text_params() -> TextParams<'static> {
    TextParams {
        font_size: 40,
        color: WHITE,
        ..Default::default()
    }
}

fn draw_shadowed_text(text: &str, x: f32, y: Option<f32>, params: TextParams) {
    let mut shadowed_text_params = params.clone();
    shadowed_text_params.color = BLACK;

    let y = y.unwrap_or(params.font_size as f32 - 6.0); // works like magic

    // draw shadow
    draw_multiline_text_ex(text, x + 2.0, y + 2.0, None, shadowed_text_params);
    // draw text
    draw_multiline_text_ex(text, x, y, None, params);
}

pub struct Game {
    pub is_running: bool,
    pub camera: Camera2D,
    is_paused: bool,
    show_debug: bool,
    show_saved_text: bool,
    fps: i32,
    fps_clock: Clock,
    pub grid: Grid,
    grid_clock: Clock,
    last_state: Option<GameState>,
}

impl Game {
    pub fn new(state: GameState) -> Self {
        let camera = Camera2D {
            zoom: Vec2::new(state.camera_zoom_x, state.camera_zoom_y),
            target: Vec2::new(state.camera_target_x, state.camera_target_y),
            ..Default::default()
        };
        Game {
            is_running: true,
            camera,
            is_paused: true,
            show_debug: false,
            show_saved_text: false,
            fps: get_fps(),
            fps_clock: Clock::new(FPS_CLOCK_INTERVAL),
            grid: state.grid,
            grid_clock: Clock::new(GRID_CLOCK_INTERVAL),
            last_state: None,
        }
    }

    fn move_camera_up(&mut self, amount: f32) {
        self.camera.target.y -= amount / self.camera.zoom.y
    }

    fn move_camera_down(&mut self, amount: f32) {
        self.camera.target.y += amount / self.camera.zoom.y
    }

    fn move_camera_left(&mut self, amount: f32) {
        self.camera.target.x -= amount / self.camera.zoom.x
    }

    fn move_camera_right(&mut self, amount: f32) {
        self.camera.target.x += amount / self.camera.zoom.x
    }

    fn get_world_mouse_point(&self) -> Vec2 {
        let (x, y) = mouse_position();
        self.camera.screen_to_world(Vec2::new(x, y))
    }

    fn get_hovered_cell_pos(&self) -> CellPos {
        let mouse_point = self.get_world_mouse_point();
        CellPos::new(mouse_point.x.floor() as i32, mouse_point.y.floor() as i32)
    }

    fn get_fps_clocked(&mut self) -> i32 {
        // get fps through clock so we can update it within interval
        // and debug screen will not look like epileptic's nightmare
        if self.fps_clock.can_update() {
            self.fps = get_fps();
        };
        return self.fps;
    }

    fn get_frustum(&self) -> Rect {
        // in the screen, the top-left corner is at {0; 0}
        // so we need to get coordinates of the top-left corner in the world
        let top_left = self.camera.screen_to_world(Vec2::new(0.0, 0.0));

        // same with the bottom-right corner, this way we define the world boundaries
        let bottom_right = self.camera.screen_to_world(Vec2::new(screen_width(), screen_height()));

        // width and height of the world is a bottom-right corner coordinates
        // subtracted from top-left corner coordinates (which are negative)
        let (width, height) = {
            let size_vec = (top_left - bottom_right).abs();
            (size_vec.x, size_vec.y)
        };

        // create a frustum rectangle
        Rect::new(top_left.x, top_left.y, width, height)
    }

    fn get_cell_rect(&self, pos: &CellPos) -> Rect {
        let pos = Vec2::new(pos.x as f32, pos.y as f32);
        Rect::new(pos.x, pos.y, CELL_SIZE, CELL_SIZE)
    }

    fn draw_cell(&self, pos: &CellPos, state: VisualState, frustum: &Rect) {
        let color = match state {
            VisualState::Alive => ALIVE_COLOR,
            VisualState::Hovered => HOVERED_COLOR,
        };
        let rect = self.get_cell_rect(pos);
        // rectangle is visible only when its overlaps with frustum
        if rect.overlaps(frustum) {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
        }
    }

    fn evolve_clocked(&mut self) {
        if self.grid_clock.can_update() {
            self.grid.evolve();
        }
    }

    fn draw_static(&mut self) {
        if self.is_paused {
            let text_params = default_text_params();
            let text = "PAUSED";
            let text_width = measure_text(text, None, text_params.font_size, text_params.font_scale).width;
            let top_right_corner = screen_width() - text_width - TEXT_SIDE_PAD;
            draw_shadowed_text(text, top_right_corner, None, text_params);
        }

        if self.show_saved_text {
            let text_params = default_text_params();

            let text = "Saved!";
            draw_shadowed_text(text, TEXT_SIDE_PAD, Some(screen_height() - TEXT_SIDE_PAD), text_params);
        }

        if self.show_debug {
            let mouse_world_point = self.get_world_mouse_point();

            let fps = self.get_fps_clocked();
            let target_xy = (self.camera.target.x, self.camera.target.y);
            let mouse_world_xy = (mouse_world_point.x, mouse_world_point.y);
            let mouse_screen_xy = mouse_position();
            let zoom = self.camera.zoom.x;
            let (screen_w, screen_h) = (screen_width(), screen_height());
            let alive = self.grid.alive.len();
            let generation = self.grid.generation;
            let grid_clock_interval = self.grid_clock.interval;

            let text = naive_dedent(&format!(
                "FPS: {fps}
                Target XY: {target_xy:?}
                Mouse world XY: {mouse_world_xy:?}
                Mouse screen XY: {mouse_screen_xy:?}
                Zoom: {zoom}
                Screen: {screen_w}x{screen_h}
                Alive: {alive}
                Generation: {generation}
                GCI: {grid_clock_interval:.5}s."
            ));

            let mut text_params = default_text_params();
            text_params.font_size = 32;
            draw_shadowed_text(&text, TEXT_SIDE_PAD, None, text_params);
        }
    }

    fn draw_mesh(&self) {
        let pos = self.get_hovered_cell_pos();
        let radius = 8;
        let diameter = 2 * radius + 1;
        let thickness = 0.0085 / self.camera.zoom.length();

        for i in 0..diameter {
            for j in 0..diameter {
                let (x, y) = (i - radius, j - radius);
                let mesh_lines_pos = Vec2::new((pos.x + x) as f32, (pos.y + y) as f32);
                let distance = mesh_lines_pos.distance(pos.as_vec2()).ceil();

                if x * x + y * y > radius * radius + 1 {
                    // Skip if dot is not inside the circle
                    continue;
                }
                if distance == 0.0 {
                    continue;
                }

                let alpha = {
                    let mut divisor = distance;
                    if divisor == 1.0 {
                        // make mesh that is close to cursor more dark
                        divisor = 1.75;
                    }
                    1.0 / divisor
                };
                draw_rectangle_lines(
                    mesh_lines_pos.x,
                    mesh_lines_pos.y,
                    CELL_SIZE,
                    CELL_SIZE,
                    thickness,
                    MESH_COLOR.with_alpha(alpha),
                );
            }
        }
    }

    pub fn save_state(&mut self) {
        self.show_saved_text = true;
        let state = GameState::from_game(self);
        self.last_state = Some(state);
        self.last_state.as_mut().unwrap().save();
    }

    fn is_point_inside(&self, x: i32, y: i32, radius: i32) -> bool {
        x * x + y * y <= radius * radius + 1
    }

    fn handle_erasure(&mut self) {
        let pos = self.get_hovered_cell_pos();
        let radius = 10;
        let diameter = 2 * radius + 1;

        if is_key_down(KeyCode::LeftShift) && get_keys_down().len() == 1 {
            // kill cells in radius of 10, kinda like eraser
            // work only when shift is the only keyboard key down
            for i in 0..diameter {
                for j in 0..diameter {
                    let (x, y) = (i - radius, j - radius);

                    if !self.is_point_inside(x, y, radius) {
                        continue;
                    }

                    if is_mouse_button_down(MouseButton::Right) {
                        self.grid.kill(&CellPos::new(pos.x + x, pos.y + y));
                    }
                    draw_rectangle(
                        (pos.x + x) as f32,
                        (pos.y + y) as f32,
                        CELL_SIZE,
                        CELL_SIZE,
                        HOVERED_COLOR.with_alpha(0.15),
                    );
                }
            }
        }
    }

    fn handle_controls(&mut self) {
        // camera movement
        if is_key_down(KeyCode::W) {
            self.move_camera_up(KB_MOVE_AMOUNT);
        }
        if is_key_down(KeyCode::S) && !is_key_down(KeyCode::LeftControl) {
            self.move_camera_down(KB_MOVE_AMOUNT);
        }
        if is_key_down(KeyCode::A) {
            self.move_camera_left(KB_MOVE_AMOUNT);
        }
        if is_key_down(KeyCode::D) {
            self.move_camera_right(KB_MOVE_AMOUNT);
        }
        if (is_key_down(KeyCode::LeftControl) && is_mouse_button_down(MouseButton::Left))
            || is_mouse_button_down(MouseButton::Middle)
        {
            let mouse_delta = mouse_delta_position();
            self.move_camera_right(mouse_delta.x);
            self.move_camera_down(mouse_delta.y);
        }

        if is_key_down(KeyCode::R) && !is_key_down(KeyCode::LeftControl) {
            let pos = self.get_hovered_cell_pos();
            let range = 25;

            for x in (pos.x - range)..(pos.x + range) {
                for y in (pos.y - range)..=(pos.y + range) {
                    if gen_range(0, 15) == gen_range(0, 15) {
                        self.grid.revive(CellPos::new(x, y));
                    }
                }
            }
        }
        if is_key_pressed(KeyCode::L) {
            let pos = self.get_hovered_cell_pos();
            let range = 75;
            for x in (pos.x - range)..(pos.x + range) {
                self.grid.revive(CellPos::new(x, pos.y));
            }
        }
        if is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::R) {
            self.grid.reset();
        }

        // state control
        if is_key_pressed(KeyCode::Space) {
            self.is_paused = !self.is_paused;
        }

        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
            self.save_state();
        }
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            self.save_state();
            self.is_running = false;
        }

        if is_key_down(KeyCode::Up) {
            if self.grid_clock.interval > 0.0 {
                self.grid_clock.interval -= 0.001;
            }
        } else if is_key_down(KeyCode::Down) {
            self.grid_clock.interval += 0.001;
        } else if is_key_pressed(KeyCode::Equal) {
            self.grid_clock.interval = GRID_CLOCK_INTERVAL;
        }

        let wheel_y = mouse_wheel().1;
        if wheel_y != 0.0 {
            let zoom = self.camera.zoom * 1.1_f32.powf(wheel_y);
            if zoom.x >= MIN_ZOOM_X && zoom.x <= MAX_ZOOM_X {
                self.camera.zoom = zoom
            }
        }

        if is_key_pressed(KeyCode::F1) {
            self.show_debug = !self.show_debug;
        }
    }

    fn draw(&mut self) {
        clear_background(BG_COLOR);
        set_camera(&self.camera);
        let frustum = self.get_frustum();

        let pos = self.get_hovered_cell_pos();
        self.draw_cell(&pos, VisualState::Hovered, &frustum);

        if is_mouse_button_down(MouseButton::Left) && !is_key_down(KeyCode::LeftControl) {
            self.grid.revive(pos);
        } else if is_mouse_button_down(MouseButton::Right) {
            self.grid.kill(&pos);
        }

        self.draw_mesh();

        for pos in &self.grid.alive {
            self.draw_cell(pos, VisualState::Alive, &frustum);
        }

        if !self.is_paused {
            self.evolve_clocked();
        } else if is_key_pressed(KeyCode::N) && self.is_paused {
            // evolve immediately
            self.grid.evolve();
        }

        if self
            .last_state
            .as_ref()
            .is_some_and(|s| *s != GameState::from_game(self))
        {
            self.show_saved_text = false;
        }
    }
}

impl Scene for Game {
    fn update(&mut self) {
        self.handle_controls();
        self.draw();
        self.handle_erasure();

        set_default_camera();
        self.draw_static();
    }
}
