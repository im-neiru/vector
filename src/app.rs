use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowAttributes,
};

use crate::window_state::WindowState;

pub struct App<'a> {
    wgpu_instance: wgpu::Instance,
    main_window: Option<WindowState<'a>>,
}

impl App<'_> {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn run(
        mut self,
    ) -> Result<(), winit::error::EventLoopError> {
        let ev = EventLoop::new()?;
        ev.run_app(&mut self)
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.main_window.is_none() {
            self.main_window = Some(WindowState::new(
                &self.wgpu_instance,
                WindowAttributes::default()
                    .with_title("Vector")
                    .with_active(true)
                    .with_maximized(true),
                event_loop,
            ));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if event == winit::event::WindowEvent::CloseRequested {
            if let Some(state) = self.main_window.as_ref() {
                if state.is_matched(window_id) {
                    self.main_window = None;
                }

                if self.main_window.is_none() {
                    event_loop.exit();
                }
            }
        }
    }
}
