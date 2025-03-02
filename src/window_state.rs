use winit::{
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

pub(crate) struct WindowState {
    window: Window,
}

impl WindowState {
    pub(crate) fn new(window_attributes: WindowAttributes, event_loop: &ActiveEventLoop) -> Self {
        match event_loop.create_window(window_attributes) {
            Ok(window) => Self { window },
            Err(os_error) => {
                let error = crate::error::Error::WindowCreationFailed(os_error);

                error.show_no_owner();
                unreachable!();
            }
        }
    }

    pub(crate) fn is_matched(&self, id: WindowId) -> bool {
        self.window.id().eq(&id)
    }
}
