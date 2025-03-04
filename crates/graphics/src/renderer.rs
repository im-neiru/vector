pub struct Renderer<'a> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    target: Target<'a>,
}

enum Target<'a> {
    Surface {
        surface: wgpu::Surface<'a>,
        config: wgpu::SurfaceConfiguration,
    },
    Headless {
        width: u32,
        height: u32,
    },
}

impl Renderer<'_> {
    pub async fn create(
        instance: &wgpu::Instance,
        target: Option<wgpu::SurfaceTargetUnsafe>,
        width: u32,
        height: u32,
    ) -> logging::Result<Self> {
        let surface = if let Some(target) = target {
            Some(unsafe {
                instance.create_surface_unsafe(target).map_err(
                    |e| {
                        logging::ErrorKind::CreateSurface(e)
                            .into_error()
                    },
                )?
            })
        } else {
            None
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference:
                    wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: surface.as_ref(),
            })
            .await
            .ok_or(
                logging::ErrorKind::NoWgpuAdapter.into_error(),
            )?;

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
            .or(
                logging::ErrorKind::NoWgpuAdapter.into_result()
            )?;

        let target = if let Some(surface) = surface {
            let surface_caps =
                surface.get_capabilities(&adapter);

            let surface_format = surface_caps
                .formats
                .iter()
                .find(|f| f.is_srgb())
                .copied()
                .unwrap_or(surface_caps.formats[0]);

            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width,
                height,
                present_mode: surface_caps.present_modes[0],
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };

            Target::Surface { surface, config }
        } else {
            Target::Headless { width, height }
        };

        #[cfg(debug_assertions)]
        println!("Backend: {}", adapter.get_info().backend);

        Ok(Self {
            device,
            queue,
            target,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        match &mut self.target {
            Target::Surface { surface, config } => {
                if (config.width == width
                    && config.height == height)
                    || width == 0
                    || height == 0
                {
                    return;
                }

                config.width = width;
                config.height = height;

                surface.configure(&self.device, config);
            }
            Target::Headless {
                width: current_width,
                height: current_height,
            } => {
                if (*current_width == width
                    && *current_height == height)
                    || width == 0
                    || height == 0
                {
                    return;
                }

                *current_width = width;
                *current_height = height;
            }
        }
    }

    pub fn draw(&mut self) -> Result<(), wgpu::SurfaceError> {
        match &self.target {
            Target::Surface { surface, config: _ } => {
                let output = surface.get_current_texture()?;

                let view = output.texture.create_view(
                    &wgpu::TextureViewDescriptor::default(),
                );

                let mut encoder =
                    self.device.create_command_encoder(
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
                                        r: 0.012,
                                        g: 0.01,
                                        b: 0.022,
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

                self.queue
                    .submit(std::iter::once(encoder.finish()));
                output.present();
                Ok(())
            }
            Target::Headless {
                width: _,
                height: _,
            } => {
                // TODO: Headless
                Ok(())
            }
        }
    }
}
