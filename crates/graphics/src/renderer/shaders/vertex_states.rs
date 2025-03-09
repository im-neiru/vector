pub(crate) struct VertexStates<'a> {
    pub(crate) vs_emit_quad_view: wgpu::VertexState<'a>,
}

impl<'a> VertexStates<'a> {
    #[inline]
    pub(crate) fn new(
        modules: &'a super::ShaderModules,
    ) -> Self {
        Self {
            vs_emit_quad_view: wgpu::VertexState {
                module: &modules.vs_emit_quad_uv,
                entry_point: Some("vs_main"),
                compilation_options:
                    wgpu::PipelineCompilationOptions::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<crate::Vec2>(
                    )
                        as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                }],
            },
        }
    }
}
