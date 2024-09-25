use crate::{compositor::render::render_jobs, models::compositor_config::Screen};
use nannou::prelude::*;
use nannou::text::Font;
use std::sync::{Mutex, OnceLock};

pub static SCREEN_STATE: OnceLock<Mutex<Screen>> = OnceLock::new();

pub fn remember_screen(screen: Screen) {
    SCREEN_STATE
        .set(Mutex::new(screen))
        .expect("Error: Mutex cannot be initialized");
}

pub fn render(screen: &Screen) {
    remember_screen(screen.to_owned());
    nannou::app(model).run();
}

struct Model {
    font: Font,
    screen: Screen,
}

pub fn model(app: &App) -> Screen {
    let screen_static = SCREEN_STATE
        .get()
        .expect("Screen not initialized")
        .lock()
        .expect("Failed to lock the mutex")
        .clone();

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

    screen_static
}

pub fn view(app: &App, model: &Screen, frame: Frame) {
    render_jobs(app, model, frame);
}
