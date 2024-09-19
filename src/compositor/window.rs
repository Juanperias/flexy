use crate::models::compositor_config::Screen;
use anyhow::Result;
use nannou::prelude::*;
use std::sync::{Mutex, OnceLock};

use super::render::render_jobs;

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

    let mut window_builder = app.new_window().view(render_jobs);

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
