use winit::{
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

use crate::graphics_state::GraphicsState;

pub(crate) struct WindowState<'a> {
    window: Window,
    graphics: GraphicsState<'a>,
}

impl WindowState<'_> {
    pub(crate) fn new(
        instance: &wgpu::Instance,
        window_attributes: WindowAttributes,
        event_loop: &ActiveEventLoop,
    ) -> Self {
        match event_loop.create_window(window_attributes) {
            Ok(window) => {
                match pollster::block_on(GraphicsState::new(
                    &window, instance,
                )) {
                    Ok(graphics) => Self { window, graphics },
                    Err(error) => {
                        error.show_with_owner(&window);
                        unreachable!()
                    }
                }
            }
            Err(os_error) => {
                let error = crate::error::Error::WindowCreation(
                    os_error,
                );

                error.show_no_owner();
                unreachable!();
            }
        }
    }

    pub(crate) fn is_matched(&self, id: WindowId) -> bool {
        self.window.id().eq(&id)
    }
}
