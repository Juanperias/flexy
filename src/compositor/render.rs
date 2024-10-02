use crate::{
    models::job::{Job, Kind},
    widgets::text,
};
use sdl2::{rect::Rect, render::Canvas, ttf::Sdl2TtfContext, video::Window};

pub fn jobs(
    canvas: &mut Canvas<Window>,
    ttf: &Sdl2TtfContext,
    jobs: &Vec<Job>,
) -> Result<Vec<(Rect, String)>, String> {
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
