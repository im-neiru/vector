use graphics::Renderer;
use winit::{
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

pub struct WindowState {
    pub window: Window,
    pub renderer: Renderer,
}

impl WindowState {
    pub fn new(
        instance: &wgpu::Instance,
        window_attributes: WindowAttributes,
        event_loop: &ActiveEventLoop,
    ) -> logging::Result<Self> {
        let window = event_loop
            .create_window(window_attributes)
            .map_err(|e| {
                logging::ErrorKind::WindowCreation(e)
                    .into_error()
            })?;

        let renderer = pollster::block_on({
            let winit::dpi::PhysicalSize { width, height } =
                window.inner_size();

            let target = unsafe {
                wgpu::SurfaceTargetUnsafe::from_window(&window)
                    .unwrap()
            };

            Renderer::create(
                instance,
                Some(target),
                width,
                height,
            )
        })?;

        Ok(Self { window, renderer })
    }

    #[inline(always)]
    pub fn is_matched(&self, id: WindowId) -> bool {
        self.window.id().eq(&id)
    }
}
