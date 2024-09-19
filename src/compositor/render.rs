use nannou::prelude::*;

use crate::{context::globals::JOBS, models::job::JobKind};

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
            JobKind::Text => {
                draw.text(&job.value)
                    .color(BLACK)
                    .font_size(20)
                    .wh(win_rect.wh());
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
