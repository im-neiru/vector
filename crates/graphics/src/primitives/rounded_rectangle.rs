use std::{num::NonZeroU64, ops::Mul};
use wgpu::util::DeviceExt;

pub struct RoundedRectangle {
    pub color: crate::Color,
    pub position: crate::Vec2,
    pub size: crate::Size,
    pub top_left_radius: f32,
    pub top_right_radius: f32,
    pub bottom_left_radius: f32,
    pub bottom_right_radius: f32,
}

pub struct RoundedRectangleState {
    fs_uniform: FsUniform,
    rectangle_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    uniform_bind_group: wgpu::BindGroup,
}

impl super::Primitive for RoundedRectangle {
    type State = RoundedRectangleState;

    fn create_state(
        self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        u_transform: &wgpu::Buffer,
        u_transform_type: wgpu::BindingType,
    ) -> Self::State {
        let shader_module =
            crate::shaders::create_rectangle(device);

        let max =
            f32::min(self.size.width, self.size.height) * 0.54;

        let tl = self.top_left_radius.clamp(0., max);
        let tr = self.top_right_radius.clamp(0., max);
        let bl = self.bottom_left_radius.clamp(0., max);
        let br = self.bottom_right_radius.clamp(0., max);

        let fs_uniform = FsUniform {
            color: self.color,
            center_tl: crate::Vec2 { x: tl, y: tl },
            center_tr: crate::Vec2 {
                x: self.size.width - tr,
                y: tr,
            },
            center_bl: crate::Vec2 {
                x: bl,
                y: self.size.height - bl,
            },
            center_br: crate::Vec2 {
                x: self.size.width - br,
                y: self.size.height - br,
            },
            radius_tl: tl,
            radius_tr: tr,
            radius_bl: bl,
            radius_br: br,
        };

        let fs_uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("fs"),
                contents: bytemuck::cast_slice(&[fs_uniform]),
                usage: wgpu::BufferUsages::UNIFORM
                    | wgpu::BufferUsages::COPY_DST,
            },
        );

        let uniform_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
            label: Some("uniform.rectangle"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: u_transform_type,
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(std::mem::size_of::<FsUniform>() as u64),
                    },
                    count: None,
                },
            ],
        });

        let uniform_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("Uniform Bind Group"),
                layout: &uniform_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: u_transform
                            .as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: fs_uniform_buffer
                            .as_entire_binding(),
                    },
                ],
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
                    array_stride: std::mem::size_of::<
                        [crate::Vec2; 2],
                    >()
                        as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
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
                            format,
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
                [
                    crate::Vec2::new(
                        self.position.x,
                        self.position.y,
                    ),
                    crate::Vec2::new(0., 0.),
                ],
                [
                    crate::Vec2::new(
                        self.position.x + self.size.width,
                        self.position.y,
                    ),
                    crate::Vec2::new(self.size.width, 0.),
                ],
                [
                    crate::Vec2::new(
                        self.position.x + self.size.width,
                        self.position.y + self.size.height,
                    ),
                    crate::Vec2::new(
                        self.size.width,
                        self.size.height,
                    ),
                ],
                [
                    crate::Vec2::new(
                        self.position.x,
                        self.position.y + self.size.height,
                    ),
                    crate::Vec2::new(0., self.size.height),
                ],
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

        RoundedRectangleState {
            fs_uniform,
            rectangle_pipeline,
            vertex_buffer,
            index_buffer,
            index_count,
            uniform_bind_group,
        }
    }
}

impl super::PrimitiveState for RoundedRectangleState {
    fn draw(&mut self, render_pass: &mut wgpu::RenderPass<'_>) {
        render_pass.set_pipeline(&self.rectangle_pipeline);
        render_pass
            .set_vertex_buffer(0, self.vertex_buffer.slice(..));

        render_pass.set_index_buffer(
            self.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );

        render_pass.set_bind_group(
            0,
            &self.uniform_bind_group,
            &[],
        );

        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct FsUniform {
    color: crate::Color,
    center_tl: crate::Vec2,
    center_tr: crate::Vec2,
    center_bl: crate::Vec2,
    center_br: crate::Vec2,
    radius_tl: f32,
    radius_tr: f32,
    radius_bl: f32,
    radius_br: f32,
}
