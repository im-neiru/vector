use ash::vk::ShaderModule;

pub(super) struct VertexShaders {
    pub(super) quad_emit_uv: ShaderModule,
}

pub(crate) struct FragmentShaders {
    pub(super) rounded_rectangle_color_fill: ShaderModule,
}

impl VertexShaders {
    pub(super) unsafe fn new(
        device: &ash::Device,
    ) -> logging::Result<Self> {
        use crate::spirv::vs;

        unsafe {
            Ok(Self {
                quad_emit_uv: create_shader_module(
                    device,
                    &vs::QUAD_EMIT_UV,
                )?,
            })
        }
    }

    pub(super) unsafe fn destroy(self, device: &ash::Device) {
        unsafe {
            device
                .destroy_shader_module(self.quad_emit_uv, None)
        };
    }
}

impl FragmentShaders {
    pub(super) unsafe fn new(
        device: &ash::Device,
    ) -> logging::Result<Self> {
        use crate::spirv::fs;

        unsafe {
            Ok(Self {
                rounded_rectangle_color_fill:
                    create_shader_module(
                        device,
                        &fs::ROUNDED_RECTANGLE_COLOR_FILL,
                    )?,
            })
        }
    }

    pub(super) unsafe fn destroy(self, device: &ash::Device) {
        unsafe {
            device.destroy_shader_module(
                self.rounded_rectangle_color_fill,
                None,
            )
        };
    }
}

#[inline(always)]
unsafe fn create_shader_module(
    device: &ash::Device,
    source: &crate::spirv::ShaderSource,
) -> logging::Result<ShaderModule> {
    unsafe {
        device
            .create_shader_module(&source.create_info(), None)
            .map_err(|err| {
                logging::ErrorKind::VulkanError {
                    function_name: "create_shader_module",
                    vk_code: err.as_raw(),
                }
                .into_error()
            })
    }
}
