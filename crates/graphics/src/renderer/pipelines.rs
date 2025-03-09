use super::shaders::{FragmentStates, VertexStates};

pub(crate) struct Pipelines {
    pub(crate) rounded_rectangle_color_fill:
        wgpu::RenderPipeline,
}

impl Pipelines {
    pub(crate) fn new(
        device: &wgpu::Device,
        binding_group_layouts: &super::BindingGroupLayouts,
        vertex_states: &VertexStates,
        fragment_states: &FragmentStates,
    ) -> Self {
        let pipeline_layout = device
        .create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("pipeline_layout.rounded_rectangle_color_fill"),
                bind_group_layouts: &[
                    &binding_group_layouts.vs_emit_quad_view,
                    &binding_group_layouts.fs_rounded_rectangle_color_fill,
                ],
                push_constant_ranges: &[],
            },
        );

        Self {
            rounded_rectangle_color_fill: device
                .create_render_pipeline(
                &wgpu::RenderPipelineDescriptor {
                    label: Some(
                        "pipeline.rounded_rectangle_color_fill",
                    ),
                    layout: Some(&pipeline_layout),
                    vertex: vertex_states
                        .vs_emit_quad_view
                        .clone(),
                    fragment: Some(
                        fragment_states
                            .fs_rounded_rectangle_color_fill.clone(),
                    ),
                    primitive: wgpu::PrimitiveState::default(),
                    depth_stencil: None,
                    multisample:
                        wgpu::MultisampleState::default(),
                    multiview: None,
                    cache: None,
                },
            ),
        }
    }
}
