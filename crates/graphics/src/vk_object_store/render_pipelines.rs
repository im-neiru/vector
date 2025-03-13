use std::mem::MaybeUninit;

use ash::vk;

use crate::{
    allocation_callbacks::ALLOCATION_CALLBACKS,
    spirv::{
        ShaderSource, fs::FragmentShaderId, vs::VertexShaderId,
    },
};

impl
    super::VkObjectStore<
        (VertexShaderId, FragmentShaderId),
        vk::Pipeline,
    >
{
    pub(crate) fn preload_pipelines<const N: usize>(
        device: &ash::Device,
        vertex_shaders: &mut super::VertexShaderStore,
        fragment_shaders: &mut super::FragmentShaderStore,
        sources: [(
            ShaderSource<VertexShaderId>,
            ShaderSource<FragmentShaderId>,
        ); N],
    ) -> logging::Result<Self> {
        #[derive(Copy, Clone)]
        struct Dependencies<'a> {
            stages: MaybeUninit<
                [vk::PipelineShaderStageCreateInfo<'a>; 2],
            >,
            // will add more later
        }

        let mut deps = [Dependencies {
            stages: MaybeUninit::uninit(),
        }; N];

        let mut infos =
            [vk::GraphicsPipelineCreateInfo::default(); N];

        for (index, (vs_source, fs_source)) in
            sources.iter().enumerate()
        {
            let vs_info =
                vertex_shaders.use_shader(device, vs_source)?;
            let fs_info = fragment_shaders
                .use_shader(device, fs_source)?;

            deps[index].stages =
                MaybeUninit::new([vs_info, fs_info]);

            let stages_ptr = deps[index].stages.as_ptr();

            infos[index] = infos[index].stages(unsafe {
                stages_ptr.as_ref().unwrap_unchecked()
                // will add more later
            });
        }

        let pipelines = unsafe {
            device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &infos,
                ALLOCATION_CALLBACKS,
            )
        }
        .map_err(|(pipelines, result)| {
            for pipeline in pipelines.into_iter() {
                unsafe {
                    device.destroy_pipeline(
                        pipeline,
                        ALLOCATION_CALLBACKS,
                    )
                };
            }

            logging::ErrorKind::VulkanError {
                function_name: "create_graphics_pipelines",
                vk_code: result.as_raw(),
            }
            .into_error()
        })?;

        let mut entries = Vec::with_capacity(N);

        for (index, pipeline) in
            pipelines.into_iter().enumerate()
        {
            let (vertex_source, fragment_source) =
                &sources[index];

            entries[index] = super::Entry {
                key: (vertex_source.id, fragment_source.id),
                object: pipeline,
            };
        }

        Ok(Self(Some(entries)))
    }
}
