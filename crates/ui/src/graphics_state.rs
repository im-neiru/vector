use crate::error::Error;

pub struct GraphicsState<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl GraphicsState<'_> {
    pub async fn new(
        window: &winit::window::Window,
        instance: &wgpu::Instance,
    ) -> Result<Self, Error> {
        let surface = unsafe {
            use rwh_05::{
                HasRawDisplayHandle, HasRawWindowHandle,
            };

            instance.create_surface_unsafe(
                wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle:
                        crate::utils::convert_display_handle_06(
                            window.raw_display_handle(),
                        ),
                    raw_window_handle:
                        crate::utils::convert_window_handle_06(
                            window.raw_window_handle(),
                        ),
                },
            )?
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference:
                    wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(crate::error::Error::NoWgpuAdapter)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let size = window.inner_size();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Ok(Self {
            surface,
            device,
            queue,
            config,
        })
    }

    pub fn resize(
        &mut self,
        size: winit::dpi::PhysicalSize<u32>,
    ) {
        if (self.config.width == size.width
            && self.config.height == size.height)
            || size.width == 0
            || size.height == 0
        {
            return;
        }

        self.config.width = size.width;
        self.config.height = size.height;

        self.surface.configure(&self.device, &self.config);
    }

    pub fn draw(
        &mut self,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(
            &wgpu::TextureViewDescriptor::default(),
        );

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        {
            let _render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(
                                    wgpu::Color {
                                        r: 0.0,
                                        g: 0.1,
                                        b: 0.2,
                                        a: 1.0,
                                    },
                                ),
                                store: wgpu::StoreOp::Store,
                            },
                        },
                    )],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                },
            );
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
