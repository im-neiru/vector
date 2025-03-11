use winit::{
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

pub struct WindowState {
    pub(crate) window: Window,
    pub(crate) surface: graphics::Surface,
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

        let surface =
            instance.create_surface_with_window(&window)?;

        // let mut draw_context = pollster::block_on({
        //     let winit::dpi::PhysicalSize { width, height } =
        //         window.inner_size();

        //     let target = unsafe {
        //         wgpu::SurfaceTargetUnsafe::from_window(&window)
        //             .unwrap()
        //     };

        //     DrawContext::create(
        //         instance,
        //         Some(target),
        //         width,
        //         height,
        //     )
        // })?;

        // draw_context.push(graphics::RoundedRectangle {
        //     color: graphics::Color::ROSY_BROWN,
        //     position: graphics::Vec2::splat(330.),
        //     size: graphics::Size::square(400.),
        //     radius: graphics::BorderRadius {
        //         top_left: 40.,
        //         top_right: 0.,
        //         bottom_left: 0.,
        //         bottom_right: 40.,
        //     },
        //     z: 1.,
        //     transform: graphics::Mat3::IDENTITY,
        // })?;

        Ok(Self { window, surface })
    }

    #[inline(always)]
    pub fn is_matched(&self, id: WindowId) -> bool {
        self.window.id().eq(&id)
    }
}
