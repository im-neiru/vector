use wgpu::util::DeviceExt;

use crate::renderer::{
    binding_group_layouts::BindingGroupLayouts,
    pipelines::Pipelines,
    primitive_store::PrimitiveStore,
    primitives::{self, Primitive},
    shaders::{
        ColorTargetStates, FragmentStates, ShaderModules,
        VertexStates,
    },
};

use super::primitives::PrimitiveState;

pub struct DrawContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    target: Box<dyn super::Target>,
    projection_buffer: wgpu::Buffer,
    update_projection: bool,
    rect: primitives::RoundedRectangle,
    rect_state: primitives::RoundedRectangleState,
    rotate: f32,
    binding_group_layouts: BindingGroupLayouts,
    pipelines: Pipelines,
    primitives: super::PrimitiveStore,
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

        let projection_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("u_transform"),
                contents: bytemuck::cast_slice(&[
                    target.projection()
                ]),
                usage: wgpu::BufferUsages::UNIFORM
                    | wgpu::BufferUsages::COPY_DST,
            },
        );

        let rotate = 0.0f32;
        let rect = primitives::RoundedRectangle {
            color: crate::Color::DODGER_BLUE,
            position: crate::Vec2::splat(128.) * 0.5
                + crate::Vec2::new(512., 360.),
            size: crate::Size::square(400.),
            radius: crate::BorderRadius::all(16.),
            z: 0.,
            transform: crate::Mat3::rotation_z(
                (rotate).to_radians(),
            ),
        };

        let shader_modules = ShaderModules::new(&device);
        let targets = ColorTargetStates::new(target.format());

        let mut binding_group_layouts =
            BindingGroupLayouts::new(&device);

        let vertex_states = VertexStates::new(&shader_modules);
        let fragment_states =
            FragmentStates::new(&shader_modules, &targets);

        let pipelines = Pipelines::new(
            &device,
            &binding_group_layouts,
            &vertex_states,
            &fragment_states,
        );

        let rect_state = rect.create_state(
            &device,
            &projection_buffer,
            &mut binding_group_layouts,
        )?;

        Ok(Self {
            device,
            queue,
            target,
            projection_buffer,
            update_projection: false,
            binding_group_layouts,
            rect,
            rect_state,
            pipelines,
            rotate,
            primitives: PrimitiveStore::new(),
        })
    }

    #[inline]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.target.resize(&self.device, width, height);
        self.update_projection = true;
    }

    pub fn draw(
        &mut self,
        delta: f32,
    ) -> Result<(), wgpu::SurfaceError> {
        let (output, view) = self.target.get_output()?;

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        if self.update_projection {
            let projection = self.target.projection();

            self.queue.write_buffer(
                &self.projection_buffer,
                0,
                bytemuck::cast_slice(&[projection]),
            );

            self.update_projection = false;
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

            self.rect_state.draw(
                &mut render_pass,
                &mut self.binding_group_layouts,
                &mut self.pipelines,
            );
        }

        {
            self.rotate += (15. * delta).clamp(0., 360.);

            self.queue.write_buffer(
                &self.rect_state.emit_quad_uv,
                0,
                bytemuck::cast_slice(&[
                    super::uniforms::EmitQuadUv {
                        transform: (crate::Mat3::rotation_y(
                            self.rotate.to_radians(),
                        )
                            * crate::Mat3::rotation_z(
                                self.rotate.to_radians(),
                            ))
                        .into(),
                        position: self.rect.position,
                        z: self.rect.z,
                        struct_pad: 0.,
                    },
                ]),
            );
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        if let Some(output) = output {
            output.present();
        }

        Ok(())
    }

    #[inline]
    pub fn rounded_rectangle(
        &mut self,
        rounded_rectangle: crate::RoundedRectangle,
    ) -> logging::Result<()> {
        self.primitives.add(
            &self.device,
            &self.projection_buffer,
            &mut self.binding_group_layouts,
            rounded_rectangle,
        )
    }
}
