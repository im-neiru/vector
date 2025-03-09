pub(crate) struct ColorTargetStates {
    pub(super) standard_blend:
        [Option<wgpu::ColorTargetState>; 1],
}

impl ColorTargetStates {
    pub(crate) fn new(format: wgpu::TextureFormat) -> Self {
        Self {
            standard_blend: [Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor:
                            wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor:
                            wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }
    }
}
