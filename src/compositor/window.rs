extern crate sdl2;

use crate::models::{compositor_config::Screen, styles::ToColor};
use colored::Colorize;
use mlua::{Function, Lua};
use sdl2::event::{Event, WindowEvent};
use std::env;
use std::time::Duration;

use super::render;

pub fn render(screen: &Screen, can_close: bool, lua: &Lua) -> Result<(), String> {
    if env::var("WAYLAND_DISPLAY").is_ok() {
        env::set_var("SDL_VIDEODRIVER", "wayland");
        println!("[{}] video driver set to wayland", "DEBUG".blue());
    }

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
        .accelerated()
        .build()
        .map_err(|err| err.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    if can_close {
                        break 'main;
                    }
                    println!("[{}] cannot close this window", "ERROR".red());
                }
                Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event,
                } => match win_event {
                    WindowEvent::Enter {} => {
                        let on_enter: Function = match lua.globals().get("OnEnter") {
                            Ok(func) => func,
                            Err(_) => lua
                                .create_function(|_, ()| Ok(()))
                                .map_err(|e| e.to_string())?,
                        };

                        on_enter.call::<(), ()>(()).map_err(|err| err.to_string())?;
                    }
                    _ => {}
                },
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
        let update: Function = match lua.globals().get("Update") {
            Ok(func) => func,
            Err(_) => lua
                .create_function(|_, ()| Ok(()))
                .map_err(|e| e.to_string())?,
        };
        update.call::<(), ()>(()).map_err(|err| err.to_string())?;
        render::jobs(&mut canvas, &ttf_context)?;
        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
