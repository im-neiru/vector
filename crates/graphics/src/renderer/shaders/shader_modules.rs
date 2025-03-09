use std::borrow::Cow;

pub(crate) struct ShaderModules {
    pub(crate) vs_emit_quad_uv: wgpu::ShaderModule,
    pub(crate) fs_rounded_rectangle_color_fill:
        wgpu::ShaderModule,
}

impl ShaderModules {
    #[inline]
    pub(crate) fn new(device: &wgpu::Device) -> Self {
        Self {
            vs_emit_quad_uv: device
                .create_shader_module(VS_EMIT_QUAD_UV),
            fs_rounded_rectangle_color_fill: device
                .create_shader_module(
                    FS_ROUNDED_RECTANGLE_COLOR_FILL,
                ),
        }
    }
}

const FS_ROUNDED_RECTANGLE_COLOR_FILL:
    wgpu::ShaderModuleDescriptor<'static> =
    wgpu::ShaderModuleDescriptor {
        label: Some("fs_rounded_rectangle_fill"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
            include_str!(
                "fs_rounded_rectangle_color_fill.wgsl"
            ),
        )),
    };

const VS_EMIT_QUAD_UV: wgpu::ShaderModuleDescriptor<'static> =
    wgpu::ShaderModuleDescriptor {
        label: Some("vs_emit_quad_uv"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
            include_str!("vs_emit_quad_uv.wgsl"),
        )),
    };
