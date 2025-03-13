use logging::UnwrapReport;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowAttributes,
};

use crate::window_state::WindowState;
pub struct App {
    instance: Option<graphics::Instance>,
    main_window: Option<WindowState>,
}

impl App {
    pub fn new() -> logging::Result<Self> {
        Ok(Self {
            instance: None,
            main_window: None,
        })
    }

    pub fn run(
        mut self,
    ) -> Result<(), winit::error::EventLoopError> {
        let ev = EventLoop::new()?;
        ev.set_control_flow(ControlFlow::Poll);
        ev.run_app(&mut self)
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.main_window.is_none() {
            let instance = match self.instance.as_mut() {
                Some(instance) => instance,
                None => {
                    self.instance = Some(
                        graphics::Instance::new(event_loop)
                            .unwrap_report(),
                    );

                    unsafe {
                        self.instance
                            .as_mut()
                            .unwrap_unchecked()
                    }
                }
            };

            self.main_window = Some({
                use winit::raw_window_handle_05::HasRawWindowHandle;

                let state = WindowState::new(
                    instance,
                    WindowAttributes::default()
                        .with_title("Vector")
                        .with_active(true)
                        .with_min_inner_size(
                            winit::dpi::PhysicalSize::new(
                                620, 465,
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
                        // state
                        //     .draw_context
                        //     .resize(width, height);
                    }
                }
            }
            RedrawRequested => {}
            _ => (),
        }
    }
}
