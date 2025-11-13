use macroquad::prelude::*;

use crate::scene::Scene;

#[derive(Clone, Copy)]
pub struct Button<'a> {
    pub text: &'a str,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
    pub border_color: Color,
    pub border_size: f32,
    pub font_size: u16,
    pub text_color: Color,
}

impl<'a> Button<'a> {
    fn new(
        text: &'a str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: Color,
        border_color: Color,
        border_size: f32,
        font_size: u16,
        text_color: Color,
    ) -> Self {
        Button {
            text,
            x,
            y,
            w,
            h,
            color,
            border_color,
            border_size,
            font_size,
            text_color,
        }
    }

    pub fn is_hovered(&self) -> bool {
        let button_rect = Rect::new(self.x, self.y, self.w, self.h);
        let (x, y) = mouse_position();
        button_rect.contains(Vec2::new(x, y))
    }

    pub fn is_pressed(&self) -> bool {
        self.is_hovered() && is_mouse_button_pressed(MouseButton::Left)
    }

    pub fn is_held(&self) -> bool {
        self.is_hovered() && is_mouse_button_down(MouseButton::Left)
    }

    pub fn is_released(&self) -> bool {
        self.is_hovered() && is_mouse_button_released(MouseButton::Left)
    }

    fn get_current_color(&self) -> Color {
        if !self.is_hovered() {
            return self.color;
        }

        // highlight when hovered with a mouse
        let color = Color::new(self.color.r + 0.1, self.color.g + 0.1, self.color.b + 0.1, self.color.a);

        if !self.is_held() {
            return color;
        }

        // highlight more when pressed
        Color::new(color.r + 0.05, color.g + 0.05, color.b + 0.05, color.a)
    }

    fn draw(&self) {
        // draw border first
        draw_rectangle(
            self.x - self.border_size,
            self.y - self.border_size,
            self.w + self.border_size * 2.0,
            self.h + self.border_size * 2.0,
            self.border_color,
        );

        let color = self.get_current_color();
        draw_rectangle(self.x, self.y, self.w, self.h, color);

        let text_dims = measure_text(
            self.text,
            None,
            self.font_size,
            1.0,
        );
        let text_x = self.x + (self.w - text_dims.width) / 2.0;
        let text_y = self.y + (self.h + text_dims.height) / 2.0;

        draw_text(self.text, text_x, text_y, self.font_size as f32, self.text_color);
    }
}

#[derive(Clone, Copy)]
pub struct MainMenu<'a> {
    pub start_button: Button<'a>,
    pub quit_button: Button<'a>,
}

impl<'a> MainMenu<'a> {
    pub fn new() -> Self {
        let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        let (w, h) = (360.0, 100.0);
        let padding = 8.0;

        let start_button = {
            Button::new(
                "Start",
                screen_center.x - (w / 2.0),
                screen_center.y * 1.1,
                w,
                h,
                Color::new(0.35, 0.35, 0.35, 1.00),
                Color::new(0.20, 0.20, 0.20, 1.00),
                6.0,
                96,
                WHITE,
            )
        };
        let quit_button = {
            Button::new(
                "Quit",
                start_button.x,
                start_button.y + h + padding,
                w,
                h,
                Color::new(0.35, 0.35, 0.35, 1.00),
                Color::new(0.20, 0.20, 0.20, 1.00),
                6.0,
                96,
                WHITE,
            )
        };
        MainMenu {
            start_button,
            quit_button,
        }
    }

    fn draw_title(&mut self) {
        let text = "Game Of Life";
        let font_size = 156;

        let center_x = {
            let dimensions = measure_text(text, None, font_size, 1.0);
            let screen_center = screen_width() / 2.0;
            let text_halfwidth = dimensions.width / 2.0;

            screen_center - text_halfwidth
        };

        draw_text(text, center_x, screen_height() / 3.0, font_size as f32, WHITE);
    }

    fn draw_buttons(&mut self) {
        self.start_button.draw();
        self.quit_button.draw();
    }

    fn draw(&mut self) {
        self.draw_title();
        self.draw_buttons();
    }
}

impl<'a> Scene for MainMenu<'a> {
    fn update(&mut self) {
        self.draw();
    }
}
