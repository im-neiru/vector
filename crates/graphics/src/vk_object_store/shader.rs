use ash::vk::ShaderModule;

use crate::{
    allocation_callbacks::ALLOCATION_CALLBACKS,
    spirv::ShaderSource,
};

pub(crate) type VertexShaderStore = super::VkObjectStore<
    crate::spirv::vs::VertexShaderId,
    ShaderModule,
>;

pub(crate) type FragmentShaderStore = super::VkObjectStore<
    crate::spirv::fs::FragmentShaderId,
    ShaderModule,
>;

impl<K> super::VkObjectStore<K, ShaderModule>
where
    K: Copy
        + Clone
        + PartialEq
        + PartialOrd
        + Ord
        + Eq
        + std::hash::Hash,
{
    #[inline]
    pub(crate) fn from_shader_sources<const N: usize>(
        device: &ash::Device,
        sources: [ShaderSource<K>; N],
    ) -> logging::Result<Self> {
        if N == 0 {
            return Ok(Self(None));
        }

        let mut storage = Vec::with_capacity(N);

        for source in sources {
            let module = unsafe {
                device
                    .create_shader_module(
                        &source.create_info(),
                        ALLOCATION_CALLBACKS,
                    )
                    .map_err(|err| {
                        logging::ErrorKind::VulkanError {
                            function_name: "create_shader_module",
                            vk_code: err.as_raw(),
                        }
                        .into_error()
                    })?
            };

            storage.push(super::Entry {
                key: source.id,
                object: module,
            })
        }

        storage.sort_by(|a, b| a.key.cmp(&b.key));

        Ok(Self(Some(storage)))
    }
}
