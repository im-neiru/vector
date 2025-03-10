use wgpu::util::DeviceExt;

use crate::renderer::{
    BindingGroupLayouts, Pipelines, uniforms,
};

#[derive(Clone, Copy)]
pub struct RoundedRectangle {
    pub color: crate::Color,
    pub position: crate::Vec2,
    pub size: crate::Size,
    pub radius: crate::BorderRadius,
    pub z: f32,
    pub transform: crate::Mat3,
}

pub struct RoundedRectangleState {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) index_count: u32,
    pub(crate) vs_emit_quad_uv: wgpu::BindGroup,
    pub(crate) fs_rounded_rectangle_color_fill: wgpu::BindGroup,
    pub(crate) emit_quad_uv: wgpu::Buffer,
}

const PADDING: crate::Vec2 = crate::Vec2::splat(12.);

impl super::Primitive for RoundedRectangle {
    type State = RoundedRectangleState;
    type Mutator = RoundedRectangleMutator;

    #[inline]
    fn get_pipeline(
        pipelines: &Pipelines,
    ) -> std::sync::Arc<wgpu::RenderPipeline> {
        pipelines.rounded_rectangle_color_fill.clone()
    }

    fn create_state(
        self,
        device: &wgpu::Device,
        projection_buffer: &wgpu::Buffer,
        binding_group_layouts: &mut BindingGroupLayouts,
    ) -> logging::Result<Self::State> {
        let emit_quad_uv = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("buffer.emit_quad_uv"),
                contents: bytemuck::cast_slice(&[
                    uniforms::EmitQuadUv {
                        transform: self.transform.into(),
                        position: self.position,
                        z: self.z,
                        struct_pad: 0.,
                    },
                ]),
                usage: wgpu::BufferUsages::UNIFORM
                    | wgpu::BufferUsages::COPY_DST,
            },
        );

        let rounded_rectangle_color_fill = device
            .create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(
                        "buffer.rounded_rectangle_color_fill",
                    ),
                    contents: bytemuck::cast_slice(&[{
                        let max = f32::min(
                            self.size.width,
                            self.size.height,
                        ) * 0.5;

                        uniforms::RoundedRectangleColorFill {
                            color: self.color,
                            size: self.size.validate()?,
                            border_radius: self
                                .radius
                                .clamp(0., max),
                            padding: PADDING,
                        }
                    }]),
                    usage: wgpu::BufferUsages::UNIFORM
                        | wgpu::BufferUsages::COPY_DST,
                },
            );

        let vs_emit_quad_uv = binding_group_layouts
            .bind_vs_emit_quad_view(
                device,
                projection_buffer.as_entire_binding(),
                emit_quad_uv.as_entire_binding(),
            );

        let fs_rounded_rectangle_color_fill =
            binding_group_layouts
                .bind_fs_rounded_rectangle_color_fill(
                    device,
                    rounded_rectangle_color_fill
                        .as_entire_binding(),
                );

        let vertex_buffer = {
            let m_u = self.size.width + PADDING.x;
            let m_v = self.size.height + PADDING.y;
            let half_u = m_u * 0.5;
            let half_v = m_v * 0.5;

            let vertex_data = [
                crate::Vec2::new(-half_u, -half_v),
                crate::Vec2::new(half_u, -half_v),
                crate::Vec2::new(half_u, half_v),
                crate::Vec2::new(-half_u, half_v),
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

        Ok(Self::State {
            vertex_buffer,
            index_buffer,
            index_count,
            vs_emit_quad_uv,
            fs_rounded_rectangle_color_fill,
            emit_quad_uv,
        })
    }
}

impl super::PrimitiveState for RoundedRectangleState {
    fn draw(
        &mut self,
        render_pass: &mut wgpu::RenderPass<'_>,
        binding_group_layouts: &BindingGroupLayouts,
    ) {
        render_pass
            .set_vertex_buffer(0, self.vertex_buffer.slice(..));

        render_pass.set_index_buffer(
            self.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );

        binding_group_layouts.set_vs_emit_quad_view(
            render_pass,
            0,
            &self.vs_emit_quad_uv,
        );

        binding_group_layouts
            .set_fs_rounded_rectangle_color_fill(
                render_pass,
                1,
                &self.fs_rounded_rectangle_color_fill,
            );

        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}

pub struct RoundedRectangleMutator {}
