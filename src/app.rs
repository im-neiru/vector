use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowAttributes,
};

use crate::window_state::WindowState;

#[derive(Default)]
pub struct App {
    main_window: Option<WindowState>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.main_window.is_none() {
            self.main_window = Some(WindowState::new(
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

impl App {
    pub fn run(mut self) {
        let ev = EventLoop::new().unwrap();
        ev.run_app(&mut self).unwrap();
    }
}
