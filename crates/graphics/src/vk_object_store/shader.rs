use ash::vk::{self, ShaderModule, TaggedStructure};

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

const ENTRY_POINT_NAME: *const i8 = c"main".as_ptr();

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

    fn internal_use_shader(
        &mut self,
        device: &ash::Device,
        source: &ShaderSource<K>,
    ) -> logging::Result<&vk::ShaderModule> {
        let storage = self.0.get_or_insert(Vec::new());

        let index = match storage
            .binary_search_by(|entry| entry.key.cmp(&source.id))
        {
            Ok(index) => index,
            Err(dest) => {
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

                storage.insert(
                    dest,
                    super::Entry {
                        key: source.id,
                        object: module,
                    },
                );

                dest
            }
        };

        Ok(unsafe {
            &storage
                .get(index)
                .as_ref()
                .unwrap_unchecked()
                .object
        })
    }
}

impl VertexShaderStore {
    #[inline]
    pub(crate) fn use_shader(
        &mut self,
        device: &ash::Device,
        source: &ShaderSource<crate::spirv::vs::VertexShaderId>,
    ) -> logging::Result<vk::PipelineShaderStageCreateInfo>
    {
        let module =
            self.internal_use_shader(device, source)?;

        Ok(vk::PipelineShaderStageCreateInfo {
            s_type: vk::PipelineShaderStageCreateInfo::STRUCTURE_TYPE,
            p_next: core::ptr::null(),
            stage: vk::ShaderStageFlags::VERTEX,
            module: *module,
            p_name: ENTRY_POINT_NAME,
            ..Default::default()
        })
    }
}

impl FragmentShaderStore {
    #[inline]
    pub(crate) fn use_shader(
        &mut self,
        device: &ash::Device,
        source: &ShaderSource<
            crate::spirv::fs::FragmentShaderId,
        >,
    ) -> logging::Result<vk::PipelineShaderStageCreateInfo>
    {
        let module =
            self.internal_use_shader(device, source)?;

        Ok(vk::PipelineShaderStageCreateInfo {
            s_type: vk::PipelineShaderStageCreateInfo::STRUCTURE_TYPE,
            p_next: core::ptr::null(),
            stage: vk::ShaderStageFlags::FRAGMENT,
            module: *module,
            p_name: ENTRY_POINT_NAME,
            ..Default::default()
        })
    }
}
