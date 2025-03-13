use winit::{
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

pub struct WindowState {
    pub(crate) window: Window,
    pub(crate) renderer: graphics::UiRenderer,
}

impl WindowState {
    pub fn new(
        instance: &graphics::Instance,
        window_attributes: WindowAttributes,
        event_loop: &ActiveEventLoop,
    ) -> logging::Result<Self> {
        let window = event_loop
            .create_window(window_attributes)
            .map_err(|e| {
                logging::ErrorKind::WindowCreation(e)
                    .into_error()
            })?;

        let surface = {
            let size = window.inner_size();
            instance.create_ui_renderer(
                &window,
                size.width,
                size.height,
            )?
        };

        Ok(Self {
            window,
            renderer: surface,
        })
    }

    #[inline(always)]
    pub fn is_matched(&self, id: WindowId) -> bool {
        self.window.id().eq(&id)
    }
}
