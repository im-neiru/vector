use crate::renderer::uniforms;

pub(crate) struct BindingGroupLayouts {
    pub(crate) vs_emit_quad_view: wgpu::BindGroupLayout,
    pub(crate) fs_rounded_rectangle_color_fill:
        wgpu::BindGroupLayout,
}

impl BindingGroupLayouts {
    #[inline]
    pub(crate) fn new(device: &wgpu::Device) -> Self {
        use super::uniforms::UniformTrait;

        let vs_emit_quad_view = device
            .create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("vs_emit_quad_view"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: uniforms::Projection::binding_type(
                        ),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: uniforms::EmitQuadUv::binding_type(
                        ),
                        count: None,
                    },
                ],
            },
        );

        let fs_rounded_rectangle_color_fill = device
            .create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some("fs_rounded_rectangle_color_fill"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: uniforms::RoundedRectangleColorFill::binding_type(
                        ),
                        count: None,
                    }],
                },
            );

        Self {
            vs_emit_quad_view,
            fs_rounded_rectangle_color_fill,
        }
    }

    #[inline(always)]
    pub(crate) fn bind_vs_emit_quad_view(
        &self,
        device: &wgpu::Device,
        projection: wgpu::BindingResource,
        emit_quad_uv: wgpu::BindingResource,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("group.vs_emit_quad_view"),
            layout: &self.vs_emit_quad_view,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: projection,
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: emit_quad_uv,
                },
            ],
        })
    }

    #[inline(always)]
    pub(crate) fn bind_fs_rounded_rectangle_color_fill(
        &self,
        device: &wgpu::Device,
        rounded_rectangle_color_fill: wgpu::BindingResource,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("group.vs_emit_quad_view"),
            layout: &self.fs_rounded_rectangle_color_fill,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: rounded_rectangle_color_fill,
            }],
        })
    }

    #[inline(always)]
    pub(crate) fn set_vs_emit_quad_view(
        &self,
        render_pass: &mut wgpu::RenderPass<'_>,
        index: u32,
        bind_group: &wgpu::BindGroup,
    ) {
        render_pass.set_bind_group(index, bind_group, &[]);
    }

    #[inline(always)]
    pub(crate) fn set_fs_rounded_rectangle_color_fill(
        &self,
        render_pass: &mut wgpu::RenderPass<'_>,
        index: u32,
        bind_group: &wgpu::BindGroup,
    ) {
        render_pass.set_bind_group(index, bind_group, &[]);
    }
}
