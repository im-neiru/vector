mod headless;
mod surfaced;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    target: Box<dyn Target>,
}

trait Target {
    fn resize(
        &mut self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
    );

    fn get_output(
        &self,
    ) -> Result<
        (Option<wgpu::SurfaceTexture>, wgpu::TextureView),
        wgpu::SurfaceError,
    >;
}

impl Renderer {
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
            surfaced::Surfaced::new(
                surface, &adapter, width, height,
            )
        } else {
            headless::Headless::new(&device, width, height)
        };

        #[cfg(debug_assertions)]
        println!("Backend: {}", adapter.get_info().backend);

        Ok(Self {
            device,
            queue,
            target,
        })
    }

    #[inline]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.target.resize(&self.device, width, height);
    }

    pub fn draw(&mut self) -> Result<(), wgpu::SurfaceError> {
        let (output, view) = self.target.get_output()?;

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

        self.queue.submit(std::iter::once(encoder.finish()));

        if let Some(output) = output {
            output.present();
        }

        Ok(())
    }
}
