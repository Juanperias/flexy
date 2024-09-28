extern crate sdl2;

use std::time::Duration;

use crate::models::{compositor_config::Screen, styles::ToColor};
use sdl2::event::Event;

use super::render::render_jobs;

pub fn render(screen: &Screen) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|err| err.to_string())?;

    let window = video_subsystem
        .window(
            &screen.name,
            screen.size_x.unwrap_or(800),
            screen.size_y.unwrap_or(600),
        )
        .always_on_top()
        .borderless()
        .build()
        .map_err(|err| err.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|err| err.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        if let Some(bg_color) = &screen.bg_color {
            let color = bg_color.to_color().map_err(|err| err.to_string())?;
            canvas.set_draw_color(color);
        } else {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        }

        canvas.clear();
        render_jobs(&mut canvas, &ttf_context)?;
        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
