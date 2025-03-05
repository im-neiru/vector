use wgpu::util::DeviceExt;

use crate::shaders;

mod headless;
mod surfaced;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    target: Box<dyn Target>,
    rectangle_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    uniform_bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    color: crate::Color,
    center_tl: [f32; 2],
    center_tr: [f32; 2],
    center_bl: [f32; 2],
    center_br: [f32; 2],
    radius_tl: f32,
    radius_tr: f32,
    radius_bl: f32,
    radius_br: f32,
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

    fn format(&self) -> wgpu::TextureFormat;
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
            surfaced::Surfaced::create(
                surface, &adapter, width, height,
            )
        } else {
            headless::Headless::create(&device, width, height)
        };

        #[cfg(debug_assertions)]
        println!("Backend: {}", adapter.get_info().backend);

        let shader_module = shaders::create_rectangle(&device);

        let radius_tl: f32 = 0.04;
        let radius_tr: f32 = 0.04;
        let radius_br: f32 = 0.04;
        let radius_bl: f32 = 0.04;

        let center_tl =
            [2.0 * radius_tl - 1.0, 1.0 - 2.0 * radius_tl];
        let center_tr =
            [1.0 - 2.0 * radius_tr, 1.0 - 2.0 * radius_tr];
        let center_bl =
            [2.0 * radius_bl - 1.0, 2.0 * radius_bl - 1.0];
        let center_br =
            [1.0 - 2.0 * radius_br, 2.0 * radius_br - 1.0];

        let u_fragment_data = Uniforms {
            color: crate::Color::DARK_GREEN,
            center_tl,
            center_tr,
            center_bl,
            center_br,
            radius_tl: radius_tl * 2.0,
            radius_tr: radius_tr * 2.0,
            radius_br: radius_br * 2.0,
            radius_bl: radius_bl * 2.0,
        };

        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Fragment Uniform Buffer"),
                contents: bytemuck::cast_slice(&[
                    u_fragment_data,
                ]),
                usage: wgpu::BufferUsages::UNIFORM
                    | wgpu::BufferUsages::COPY_DST,
            },
        );

        use std::num::NonZeroU64;

        let uniform_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(std::mem::size_of::<Uniforms>() as u64),
                },
                count: None,
            }],
        });

        let uniform_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("Uniform Bind Group"),
                layout: &uniform_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer
                        .as_entire_binding(),
                }],
            },
        );

        let rectangle_pipeline = {
            let pipeline_layout = device
                .create_pipeline_layout(
                    &wgpu::PipelineLayoutDescriptor {
                        label: Some("Render Pipeline Layout"),
                        bind_group_layouts: &[
                            &uniform_bind_group_layout,
                        ],
                        push_constant_ranges: &[],
                    },
                );

            let vertex_buffer_layout =
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<[f32; 2]>(
                    )
                        as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                };

            device.create_render_pipeline(
                &wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader_module,
                        entry_point: Some("vs_main"),
                        buffers: &[vertex_buffer_layout],
                        compilation_options: wgpu::PipelineCompilationOptions::default()
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader_module,
                        entry_point: Some("fs_main"),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: target.format(),
                            blend: Some(
                                wgpu::BlendState {
                                    color: wgpu::BlendComponent {
                                        src_factor: wgpu::BlendFactor::SrcAlpha,
                                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                        operation: wgpu::BlendOperation::Add,
                                    },
                                    alpha: wgpu::BlendComponent {
                                        src_factor: wgpu::BlendFactor::One,
                                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                        operation: wgpu::BlendOperation::Add,
                                    },
                                }
                            ),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: wgpu::PipelineCompilationOptions::default()
                    }),
                    primitive: wgpu::PrimitiveState::default(),
                    depth_stencil: None,
                    multisample:
                        wgpu::MultisampleState::default(),
                    multiview: None,
                    cache: None,
                },
            )
        };

        let vertex_buffer = {
            let vertex_data = [
                crate::Vec2::new(-1., -1.),
                crate::Vec2::new(1., -1.),
                crate::Vec2::new(1., 1.),
                crate::Vec2::new(-1., 1.),
            ];

            device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(
                        &vertex_data,
                    ),
                    usage: wgpu::BufferUsages::VERTEX,
                },
            )
        };

        let (index_buffer, index_count) = {
            let index_data: &[u16] = &[0, 1, 2, 2, 3, 0];

            (
                device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(
                            index_data,
                        ),
                        usage: wgpu::BufferUsages::INDEX,
                    },
                ),
                index_data.len() as u32,
            )
        };

        Ok(Self {
            device,
            queue,
            target,
            rectangle_pipeline,
            vertex_buffer,
            index_buffer,
            index_count,
            uniform_bind_group,
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
                                        r: 0.95,
                                        g: 0.95,
                                        b: 0.95,
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

            render_pass.set_pipeline(&self.rectangle_pipeline);
            render_pass.set_vertex_buffer(
                0,
                self.vertex_buffer.slice(..),
            );

            render_pass.set_index_buffer(
                self.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );

            render_pass.set_bind_group(
                0,
                &self.uniform_bind_group,
                &[],
            );

            render_pass.draw_indexed(
                0..self.index_count,
                0,
                0..1,
            );
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        if let Some(output) = output {
            output.present();
        }

        Ok(())
    }
}
