use anyhow::Result;
use skia_safe::{
    gpu::{self, backend_render_targets, gl::FramebufferInfo},
    Surface,
};
use winit::window::Window;

pub fn create_surface(
    window: &Window,
    fb_info: FramebufferInfo,
    gr_context: &mut skia_safe::gpu::DirectContext,
    num_samples: usize,
    stencil_size: usize,
) -> Result<Surface> {
    let size = window.inner_size();
    let size = (size.width.try_into()?, size.height.try_into()?);
    let backend_render_target =
        backend_render_targets::make_gl(size, num_samples, stencil_size, fb_info);

    let surface = gpu::surfaces::wrap_backend_render_target(
        gr_context,
        &backend_render_target,
        gpu::SurfaceOrigin::BottomLeft,
        skia_safe::ColorType::BGRA8888,
        None,
        None,
    )
    .expect("Error: unable to create skia surface");

    Ok(surface)
}
