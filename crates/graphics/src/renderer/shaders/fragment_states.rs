pub(crate) struct FragmentStates<'a> {
    pub(crate) fs_rounded_rectangle_color_fill:
        wgpu::FragmentState<'a>,
}

impl<'a> FragmentStates<'a> {
    #[inline]
    pub(crate) fn new(
        modules: &'a super::ShaderModules,
        targets: &'a super::ColorTargetStates,
    ) -> Self {
        Self {
            fs_rounded_rectangle_color_fill: wgpu::FragmentState {
                module: &modules.fs_rounded_rectangle_color_fill,
                entry_point: Some("fs_main"),
                compilation_options:
                    wgpu::PipelineCompilationOptions::default(),
                targets: &targets.standard_blend,
            },
        }
    }
}
