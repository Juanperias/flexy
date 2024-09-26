use nannou::prelude::*;

use crate::{
    context::globals::JOBS,
    models::{compositor_config::Screen, job::Kind, styles::ToColor},
    widgets::text,
};

pub fn render_jobs(app: &App, model: &Screen, frame: Frame) {
    let jobs_guard = JOBS
        .get()
        .expect("Error: cannot get the jobs")
        .lock()
        .expect("Error: cannot lock the jobs");
    let jobs = &*jobs_guard;

    let draw = app.draw();
    let color = model.bg_color.clone().unwrap_or("ffffff".to_string());
    let rgb_color = color.to_color().unwrap_or(WHITE.into_format());
    draw.background().color(rgb_color);

    for job in jobs {
        let win_rect = app.main_window().rect().pad(20.0);

        match job.kind {
            Kind::Text => {
                text::render(&draw, win_rect, job);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
