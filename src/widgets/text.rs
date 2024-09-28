use sdl2::{
    render::{Texture, TextureCreator},
    surface::Surface,
    ttf::Sdl2TtfContext,
    video::WindowContext,
};

use crate::models::job::Job;

pub fn render<'a>(
    ttf: &Sdl2TtfContext,
    job: &Job,
    texture_creature: &'a TextureCreator<WindowContext>,
) -> Result<(Texture<'a>, Surface<'a>), String> {
    let font = ttf.load_font(&job.style.font, job.style.font_size)?;
    let surface = font
        .render(&job.value)
        .blended(job.style.color)
        .map_err(|err| err.to_string())?;
    let texture = texture_creature
        .create_texture_from_surface(&surface)
        .map_err(|err| err.to_string())?;
    Ok((texture, surface))
}
