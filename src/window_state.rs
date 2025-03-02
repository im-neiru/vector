use winit::{
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

pub(crate) struct WindowState {
    window: Window,
}

impl WindowState {
    pub(crate) fn new(window_attributes: WindowAttributes, event_loop: &ActiveEventLoop) -> Self {
        let window = event_loop.create_window(window_attributes).unwrap();

        Self { window }
    }

    pub(crate) fn is_matched(&self, id: WindowId) -> bool {
        self.window.id().eq(&id)
    }
}
