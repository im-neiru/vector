use logging::UnwrapReport;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowAttributes,
};

use crate::window_state::WindowState;

use std::time::Instant;

pub struct DeltaCounter {
    last_frame: Instant,
}

impl DeltaCounter {
    pub fn new() -> Self {
        Self {
            last_frame: Instant::now(),
        }
    }

    pub fn tick(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame);
        self.last_frame = now;
        delta.as_secs_f32()
    }
}

pub struct App {
    wgpu_instance: wgpu::Instance,
    main_window: Option<WindowState>,
    counter: DeltaCounter,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            wgpu_instance: wgpu::Instance::new(
                &wgpu::InstanceDescriptor {
                    backends: wgpu::Backends::VULKAN,
                    flags: wgpu::InstanceFlags::default(),
                    backend_options:
                        wgpu::BackendOptions::default(),
                },
            ),
            main_window: None,
            counter: DeltaCounter::new(),
        }
    }

    pub fn run(
        mut self,
    ) -> Result<(), winit::error::EventLoopError> {
        logging::set_panic_hook();

        let ev = EventLoop::new()?;
        ev.set_control_flow(ControlFlow::Poll);
        ev.run_app(&mut self)
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.main_window.is_none() {
            event_loop.set_control_flow(ControlFlow::Poll);

            self.main_window = Some({
                use rwh_05::HasRawWindowHandle;

                let state = WindowState::new(
                    &self.wgpu_instance,
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
                        state
                            .draw_context
                            .resize(width, height);
                    }
                }
            }
            RedrawRequested => {
                if let Some(state) = self.main_window.as_mut() {
                    if state.is_matched(window_id) {
                        let delta = self.counter.tick();
                        if let Err(err) =
                            state.draw_context.draw(delta)
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
                                        .draw_context
                                        .resize(width, height);
                                }
                                OutOfMemory => {
                                    event_loop.exit();
                                }
                                _ => (),
                            }
                        }

                        state.window.request_redraw();
                    }
                }
            }
            _ => (),
        }
    }
}
