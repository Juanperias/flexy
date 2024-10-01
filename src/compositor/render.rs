use crate::{context::globals::JOBS, models::job::Kind, widgets::text};
use sdl2::{rect::Rect, render::Canvas, ttf::Sdl2TtfContext, video::Window};

pub fn jobs(
    canvas: &mut Canvas<Window>,
    ttf: &Sdl2TtfContext,
) -> Result<Vec<(Rect, String)>, String> {
    let jobs_guard = JOBS
        .get()
        .expect("Error: cannot get the jobs")
        .lock()
        .expect("Error: cannot lock the jobs");
    let jobs = &*jobs_guard;
    let texture_creator = canvas.texture_creator();
    let mut job_rects = Vec::new();

    for job in jobs {
        match &job.kind {
            Kind::Text => {
                let (texture, surface) = text::render(ttf, job, &texture_creator)?;
                let rect = Rect::new(
                    job.style.pos_x,
                    job.style.pos_y,
                    surface.width(),
                    surface.height(),
                );
                canvas.copy(&texture, None, Some(rect))?;
                job_rects.push((rect, job.value.clone()));
            }
        }
    }

    Ok(job_rects)
}
