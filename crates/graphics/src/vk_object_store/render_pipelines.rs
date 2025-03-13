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
        render_pass: &vk::RenderPass,
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

        let multisample_state =
            vk::PipelineMultisampleStateCreateInfo {
                rasterization_samples:
                    vk::SampleCountFlags::TYPE_1,
                ..Default::default()
            };

        let color_blend_attachment_states =
            [vk::PipelineColorBlendAttachmentState {
                blend_enable: 0,
                src_color_blend_factor:
                    vk::BlendFactor::SRC_COLOR,
                dst_color_blend_factor:
                    vk::BlendFactor::ONE_MINUS_DST_COLOR,
                color_blend_op: vk::BlendOp::ADD,
                src_alpha_blend_factor: vk::BlendFactor::ZERO,
                dst_alpha_blend_factor: vk::BlendFactor::ZERO,
                alpha_blend_op: vk::BlendOp::ADD,
                color_write_mask: vk::ColorComponentFlags::RGBA,
            }];

        let color_blend_state =
            vk::PipelineColorBlendStateCreateInfo::default()
                .logic_op(vk::LogicOp::CLEAR)
                .attachments(&color_blend_attachment_states);

        let dynamic_state = [
            vk::DynamicState::VIEWPORT,
            vk::DynamicState::SCISSOR,
        ];

        let dynamic_state =
            vk::PipelineDynamicStateCreateInfo::default()
                .dynamic_states(&dynamic_state);

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

            infos[index] = infos[index]
                .stages(unsafe {
                    stages_ptr.as_ref().unwrap_unchecked()
                })
                .multisample_state(&multisample_state)
                .color_blend_state(&color_blend_state)
                .dynamic_state(&dynamic_state)
                .render_pass(*render_pass);
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
