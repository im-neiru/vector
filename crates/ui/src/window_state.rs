use winit::{
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

use crate::GraphicsState;

pub struct WindowState<'a> {
    pub window: Window,
    pub graphics: GraphicsState<'a>,
}

impl WindowState<'_> {
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

        let graphics = pollster::block_on(GraphicsState::new(
            &window, instance,
        ))?;

        Ok(Self { window, graphics })
    }

    #[inline(always)]
    pub fn is_matched(&self, id: WindowId) -> bool {
        self.window.id().eq(&id)
    }
}
