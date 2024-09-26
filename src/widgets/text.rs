use nannou::prelude::*;

use crate::models::job::Job;

pub fn render(draw: &Draw, win_rect: Rect, job: &Job) {
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
