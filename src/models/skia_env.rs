use colored::Colorize;
use glutin::{
    context::PossiblyCurrentContext,
    surface::{GlSurface, Surface as GlutinSurface, WindowSurface},
};
use skia_safe::{gpu::gl::FramebufferInfo, Color, Surface as SkiaSurface};
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, Modifiers, WindowEvent},
    window::Window,
};

pub struct SkiaEnv {
    pub surface: SkiaSurface,
    pub gl_surface: GlutinSurface<WindowSurface>,
    pub gr_context: skia_safe::gpu::DirectContext,
    pub gl_context: PossiblyCurrentContext,
    pub window: Window,
}

pub struct SkiaApplication {
    pub env: SkiaEnv,
    pub fb_info: FramebufferInfo,
    pub num_samples: usize,
    pub stencil_size: usize,
    pub modifiers: Modifiers,
    pub frame: usize,
    pub previus_frame_start: Instant,
}

impl ApplicationHandler for SkiaApplication {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("[{}] application resumed", "DEBUG".blue());
    }
    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        if let winit::event::StartCause::ResumeTimeReached { .. } = cause {
            self.env.window.request_redraw();
        }
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let mut draw_frame = false;
        let frame_start = Instant::now();

        match event {
            WindowEvent::CloseRequested => {
                println!(
                    "[{}] You cannot close a widget, you must kill the process",
                    "ERROR".red()
                );
            }
            WindowEvent::Resized(..) => {}
            WindowEvent::ModifiersChanged(new_modifer) => self.modifiers = new_modifer,
            WindowEvent::KeyboardInput {
                event: KeyEvent { .. },
                ..
            } => {}
            WindowEvent::RedrawRequested => {}
            _ => (),
        }

        let expected_frame_len_seconds = 1.0 / 20.0;
        let frame_duration = Duration::from_secs_f32(expected_frame_len_seconds);

        if frame_start - self.previus_frame_start > frame_duration {
            draw_frame = true;
            self.previus_frame_start = frame_start;
        }

        if draw_frame {
            self.frame += 1;
            let canvas = self.env.surface.canvas();
            canvas.clear(Color::WHITE);
            self.env.gr_context.flush_and_submit();
            self.env
                .gl_surface
                .swap_buffers(&self.env.gl_context)
                .expect("Error: failed to swap buffers");
        }

        event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
            self.previus_frame_start + frame_duration,
        ));
    }
}
