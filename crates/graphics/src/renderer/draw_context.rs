use wgpu::util::DeviceExt;

use crate::primitives::{
    Primitive, PrimitiveState, RoundedRectangle,
};

pub struct DrawContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    target: Box<dyn super::Target>,
    u_transform: wgpu::Buffer,
    rect1: crate::primitives::RoundedRectangleState,
    rect2: crate::primitives::RoundedRectangleState,
    update_transform: bool,
}

impl DrawContext {
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
            super::Surfaced::create(
                surface, &adapter, width, height,
            )
        } else {
            super::Headless::create(&device, width, height)
        };

        #[cfg(debug_assertions)]
        println!("Backend: {}", adapter.get_info().backend);

        let u_transform =
            super::TransformUniform::new(width, height);

        let u_transform = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("u_transform"),
                contents: bytemuck::cast_slice(&[u_transform]),
                usage: wgpu::BufferUsages::UNIFORM
                    | wgpu::BufferUsages::COPY_DST,
            },
        );

        let rect1 = RoundedRectangle {
            color: crate::Color::ORANGE_RED,
            position: crate::Vec2::new(8.0, 8.0),
            size: crate::Vec2::new(64.0, 96.0),
            top_left_radius: 128.,
            top_right_radius: 128.,
            bottom_left_radius: 128.,
            bottom_right_radius: 128.,
        };

        let rect2 = RoundedRectangle {
            color: crate::Color::ORANGE,
            position: crate::Vec2::new(150.0, 128.0),
            size: crate::Vec2::new(128.0, 32.0),
            top_left_radius: 32.,
            top_right_radius: 32.,
            bottom_left_radius: 32.,
            bottom_right_radius: 32.,
        };

        let rect1 =
            rect1.create_state(
                &device,
                target.format(),
                &u_transform,
                wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: std::num::NonZeroU64::new(
                        std::mem::size_of::<
                            super::TransformUniform,
                        >() as u64,
                    ),
                },
            );

        let rect2 =
            rect2.create_state(
                &device,
                target.format(),
                &u_transform,
                wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: std::num::NonZeroU64::new(
                        std::mem::size_of::<
                            super::TransformUniform,
                        >() as u64,
                    ),
                },
            );

        Ok(Self {
            device,
            queue,
            target,
            u_transform,
            rect1,
            rect2,
            update_transform: false,
        })
    }

    #[inline]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.target.resize(&self.device, width, height);
        self.update_transform = true;
    }

    pub fn draw(&mut self) -> Result<(), wgpu::SurfaceError> {
        let (output, view) = self.target.get_output()?;

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        if self.update_transform {
            let u_transform = self.target.u_transform();

            self.queue.write_buffer(
                &self.u_transform,
                0,
                bytemuck::cast_slice(&[u_transform]),
            );

            self.update_transform = false;
        }

        {
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(
                                    wgpu::Color {
                                        r: 0.122,
                                        g: 0.137,
                                        b: 0.208,
                                        a: 1.,
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

            self.rect1.draw(&mut render_pass);
            self.rect2.draw(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        if let Some(output) = output {
            output.present();
        }

        Ok(())
    }
}
