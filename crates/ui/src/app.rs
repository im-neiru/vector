use logging::UnwrapReport;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowAttributes,
};

use crate::window_state::WindowState;

pub struct App<'a> {
    wgpu_instance: wgpu::Instance,
    main_window: Option<WindowState<'a>>,
}

impl App<'_> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            wgpu_instance: wgpu::Instance::new(
                &wgpu::InstanceDescriptor {
                    backends: wgpu::Backends::PRIMARY,
                    flags: wgpu::InstanceFlags::default(),
                    backend_options:
                        wgpu::BackendOptions::default(),
                },
            ),
            main_window: None,
        }
    }

    pub fn run(
        mut self,
    ) -> Result<(), winit::error::EventLoopError> {
        logging::set_panic_hook();

        let ev = EventLoop::new()?;
        ev.set_control_flow(ControlFlow::Wait);
        ev.run_app(&mut self)
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.main_window.is_none() {
            self.main_window = Some({
                use rwh_05::HasRawWindowHandle;

                let state = WindowState::new(
                    &self.wgpu_instance,
                    WindowAttributes::default()
                        .with_title("Vector")
                        .with_active(true)
                        .with_min_inner_size(
                            winit::dpi::PhysicalSize::new(
                                800, 600,
                            ),
                        )
                        .with_maximized(true),
                    event_loop,
                )
                .unwrap_report();

                logging::set_dialog_box_owner(Some(
                    state.window.raw_window_handle(),
                ));

                state
            })
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        use winit::event::WindowEvent::*;

        match event {
            CloseRequested => {
                if let Some(state) = self.main_window.as_ref() {
                    if state.is_matched(window_id) {
                        self.main_window = None;
                    }

                    if self.main_window.is_none() {
                        event_loop.exit();
                    }
                }
            }
            Resized(PhysicalSize { width, height }) => {
                if let Some(state) = self.main_window.as_mut() {
                    if state.is_matched(window_id) {
                        state.renderer.resize(width, height);
                    }
                }
            }
            RedrawRequested => {
                if let Some(state) = self.main_window.as_mut() {
                    if state.is_matched(window_id) {
                        if let Err(err) = state.renderer.draw()
                        {
                            use wgpu::SurfaceError::*;
                            match err {
                                Outdated | Lost => {
                                    let PhysicalSize {
                                        width,
                                        height,
                                    } = state
                                        .window
                                        .inner_size();

                                    state
                                        .renderer
                                        .resize(width, height);
                                }
                                OutOfMemory => {
                                    event_loop.exit();
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
