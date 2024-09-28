use sdl2::{rect::Rect, render::Canvas, ttf::Sdl2TtfContext, video::Window};

use crate::{context::globals::JOBS, models::job::Kind, widgets::text};

pub fn jobs(canvas: &mut Canvas<Window>, ttf: &Sdl2TtfContext) -> Result<(), String> {
    let jobs_guard = JOBS
        .get()
        .expect("Error: cannot get the jobs")
        .lock()
        .expect("Error: cannot lock the jobs");
    let jobs = &*jobs_guard;
    let texture_creature = canvas.texture_creator();

    for job in jobs {
        match &job.kind {
            Kind::Text => {
                let (texture, surface) = text::render(ttf, job, &texture_creature)?;
                canvas.copy(
                    &texture,
                    None,
                    Some(Rect::new(
                        job.style.pos_x,
                        job.style.pos_y,
                        surface.width(),
                        surface.height(),
                    )),
                )?;
            }
        }
    }

    Ok(())
}
