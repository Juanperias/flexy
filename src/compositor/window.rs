use crate::models::compositor_config::Screen;
use anyhow::Result;
use nannou::prelude::*;
use std::sync::{Mutex, OnceLock};

pub static SCREEN_STATE: OnceLock<Mutex<Screen>> = OnceLock::new();

pub fn remember_screen(screen: Screen) {
    SCREEN_STATE
        .set(Mutex::new(screen))
        .expect("Error: Mutex cannot be initialized");
}

pub fn render_window(screen: &Screen) -> Result<()> {
    remember_screen(screen.to_owned());
    nannou::app(model).run();

    Ok(())
}

pub fn model(app: &App) {
    let screen_static = SCREEN_STATE
        .get()
        .expect("Screen not initialized")
        .lock()
        .expect("Failed to lock the mutex");

    let mut window_builder = app.new_window().view(view);

    if let (Some(size_x), Some(size_y)) = (screen_static.size_x, screen_static.size_y) {
        window_builder = window_builder
            .size(size_x, size_y)
            .max_size(size_x, size_y)
            .min_size(size_x, size_y);
    }

    let _window_id = window_builder
        .build()
        .expect("Error: cannot build the window");
}

fn view(app: &App, _model: &(), frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let win_rect = app.main_window().rect().pad(20.0);

    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n\nResize the window to test dynamic layout.";

    //                         L     o     r     e     m           i    p    s    u    m
    let glyph_colors = vec![BLUE, BLUE, BLUE, BLUE, BLUE, BLACK, RED, RED, RED, RED, RED];

    draw.text(text)
        .color(BLACK)
        .glyph_colors(glyph_colors)
        .font_size(24)
        .wh(win_rect.wh());

    draw.to_frame(app, &frame).unwrap();
}
