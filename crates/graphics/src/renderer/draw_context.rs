use wgpu::util::DeviceExt;

use crate::renderer::{
    binding_group_layouts::BindingGroupLayouts,
    pipelines::Pipelines,
    primitive_store::PrimitiveStore,
    shaders::{
        ColorTargetStates, FragmentStates, ShaderModules,
        VertexStates,
    },
};

pub struct DrawContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    target: Box<dyn super::Target>,
    projection_buffer: wgpu::Buffer,
    update_projection: bool,
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

        let shader_modules = ShaderModules::new(&device);
        let targets = ColorTargetStates::new(target.format());

        let binding_group_layouts =
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

        Ok(Self {
            device,
            queue,
            target,
            projection_buffer,
            update_projection: false,
            binding_group_layouts,
            pipelines,
            primitives: PrimitiveStore::new(),
        })
    }

    #[inline]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.target.resize(&self.device, width, height);
        self.update_projection = true;
    }

    pub fn draw(&mut self) -> Result<(), wgpu::SurfaceError> {
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

            self.primitives.render(
                &mut render_pass,
                &self.binding_group_layouts,
            );
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        if let Some(output) = output {
            output.present();
        }

        Ok(())
    }

    #[inline]
    pub fn push<P>(
        &mut self,
        primitive: P,
    ) -> logging::Result<()>
    where
        P: crate::Primitive + 'static,
    {
        self.primitives.add(
            &self.device,
            &self.projection_buffer,
            &mut self.binding_group_layouts,
            &self.pipelines,
            primitive,
        )
    }
}
