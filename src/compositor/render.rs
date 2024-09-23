use nannou::prelude::*;

use crate::{context::globals::JOBS, models::job::Kind};

pub fn render_jobs(app: &App, _model: &(), frame: Frame) {
    let jobs_guard = JOBS
        .get()
        .expect("Error: cannot get the jobs")
        .lock()
        .expect("Error: cannot lock the jobs");
    let jobs = &*jobs_guard;

    let draw = app.draw();
    draw.background().color(WHITE);

    for job in jobs {
        let win_rect = app.main_window().rect().pad(20.0);

        match job.kind {
            Kind::Text => {
                draw.text(&job.value)
                    .color(job.style.color)
                    .font_size(job.style.font_size)
                    .x_y_z(job.style.pos_x, job.style.pos_y, 0.0)
                    .wh(win_rect.wh());
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
