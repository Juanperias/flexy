use std::{ffi::CString, num::NonZeroU32, time::Instant};

use anyhow::Result;
use colored::Colorize;
use gl;
use gl::types::*;
use glutin::{
    config::{ConfigTemplateBuilder, GlConfig},
    context::{ContextApi, ContextAttributesBuilder, NotCurrentGlContext},
    display::{GetGlDisplay, GlDisplay},
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use skia_safe::gpu::gl::FramebufferInfo;
use winit::{dpi::LogicalSize, event::Modifiers, event_loop::EventLoop, window::WindowAttributes};

use crate::{
    compositor::surface::create_surface,
    models::skia_env::{SkiaApplication, SkiaEnv},
};

pub fn render_window() -> Result<()> {
    let event_loop = EventLoop::new()?;

    let window_attributes = WindowAttributes::default()
        .with_title("Flexy widget")
        .with_inner_size(LogicalSize::new(100, 100))
        .with_max_inner_size(LogicalSize::new(100, 100))
        .with_min_inner_size(LogicalSize::new(100, 100));

    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(true);

    let display_builder = DisplayBuilder::new().with_window_attributes(window_attributes.into());

    let (window, gl_config) = display_builder
        .build(&event_loop, template, |configs| {
            configs
                .reduce(|accum, config| {
                    let transparency_check = config.supports_transparency().unwrap_or(false)
                        & !accum.supports_transparency().unwrap_or(false);

                    if transparency_check || config.num_samples() < accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .expect("Error: the screen cannot be built");

    println!(
        "[{}] The selected config is {}",
        "DEBUG".blue(),
        gl_config.num_samples()
    );

    let window = window.expect("Error: window could not be created");

    let window_handle = window.window_handle()?;

    let ctx_attributes = ContextAttributesBuilder::new().build(Some(window_handle.into()));

    let fallback_ctx_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(Some(window_handle.into()));

    let not_current_gl_context = unsafe {
        gl_config
            .display()
            .create_context(&gl_config, &ctx_attributes)
            .unwrap_or_else(|_| {
                gl_config
                    .display()
                    .create_context(&gl_config, &fallback_ctx_attributes)
                    .expect("Error: the context could not be created")
            })
    };

    let (width, height): (u32, u32) = window.inner_size().into();

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        window_handle.into(),
        NonZeroU32::new(width).expect("Error: Width must be greater than 0"),
        NonZeroU32::new(height).expect("Error: Height must be greater than 0"),
    );

    let gl_surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &attrs)?
    };

    let gl_context = not_current_gl_context
        .make_current(&gl_surface)
        .expect("Could not make GL context current when setting up skia renderer");

    gl::load_with(|s| {
        gl_config
            .display()
            .get_proc_address(CString::new(s).unwrap().as_c_str())
    });

    let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
        if name == "eglGetCurrentDisplay" {
            return std::ptr::null();
        }

        gl_config
            .display()
            .get_proc_address(CString::new(name).unwrap().as_c_str())
    })
    .expect("Error: the interface could not be loaded");

    println!("[{}] interface loaded correctly", "DEBUG".blue());

    let mut gr_context = skia_safe::gpu::direct_contexts::make_gl(interface, None)
        .expect("error: the context could not be created");

    let fb_info = {
        let mut fboid: GLint = 0;
        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

        FramebufferInfo {
            fboid: fboid.try_into()?,
            format: skia_safe::gpu::gl::Format::RGBA8.into(),
            ..Default::default()
        }
    };

    let num_samples = gl_config.num_samples() as usize;
    let stencil_size = gl_config.stencil_size() as usize;

    let surface = create_surface(&window, fb_info, &mut gr_context, num_samples, stencil_size)?;

    let env = SkiaEnv {
        surface,
        gl_surface,
        gl_context,
        gr_context,
        window,
    };

    let mut app = SkiaApplication {
        env,
        fb_info,
        num_samples,
        stencil_size,
        modifiers: Modifiers::default(),
        frame: 0,
        previus_frame_start: Instant::now(),
    };

    event_loop.run_app(&mut app)?;

    Ok(())
}
