use nannou::prelude::*;

use crate::{
    context::globals::JOBS,
    models::{compositor_config::Screen, job::Kind, styles::ToColor},
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
                if let Some(font) = &job.style.font {
                    draw.text(&job.value)
                        .font(font.to_owned())
                        .color(job.style.color)
                        .font_size(job.style.font_size)
                        .x_y_z(job.style.pos_x, job.style.pos_y, 0.0)
                        .wh(win_rect.wh());
                } else {
                    draw.text(&job.value)
                        .color(job.style.color)
                        .font_size(job.style.font_size)
                        .x_y_z(job.style.pos_x, job.style.pos_y, 0.0)
                        .wh(win_rect.wh());
                }
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
