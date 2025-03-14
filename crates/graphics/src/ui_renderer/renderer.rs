use ash::{khr, vk};

use crate::{
    allocation_callbacks::ALLOCATION_CALLBACKS,
    spirv::{fs::FragmentShaderId, vs::VertexShaderId},
    vk_object_store::{
        FragmentShaderStore, VertexShaderStore, VkObjectStore,
    },
};

pub struct UiRenderer {
    surface_loader: khr::surface::Instance,
    swapchain_loader: khr::swapchain::Device,
    surface_khr: vk::SurfaceKHR,
    swapchain_khr: vk::SwapchainKHR,
    device: ash::Device,
    present_queue: vk::Queue,
    command_pool: vk::CommandPool,
    setup_command_buffer: vk::CommandBuffer,
    draw_command_buffer: vk::CommandBuffer,
    present_image_views: Box<[vk::ImageView]>,
    vertex_shaders: VertexShaderStore,
    fragment_shaders: FragmentShaderStore,
    render_pipelines:
        VkObjectStore<RenderPipelineId, vk::Pipeline>,
}

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
struct RenderPipelineId {
    vs: VertexShaderId,
    fs: FragmentShaderId,
}

impl UiRenderer {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn create(
        surface_loader: khr::surface::Instance,
        swapchain_loader: khr::swapchain::Device,
        surface_khr: vk::SurfaceKHR,
        swapchain_khr: vk::SwapchainKHR,
        device: ash::Device,
        present_queue: vk::Queue,
        command_pool: vk::CommandPool,
        setup_command_buffer: vk::CommandBuffer,
        draw_command_buffer: vk::CommandBuffer,
        present_image_views: Box<[vk::ImageView]>,
    ) -> logging::Result<Self> {
        use crate::spirv::*;

        let vertex_shaders =
            VertexShaderStore::from_shader_sources(
                &device,
                [vs::QUAD_EMIT_UV],
            )?;

        let fragment_shaders =
            FragmentShaderStore::from_shader_sources(
                &device,
                [fs::ROUNDED_RECTANGLE_COLOR_FILL],
            )?;

        Ok(Self {
            surface_loader,
            swapchain_loader,
            surface_khr,
            swapchain_khr,
            device,
            present_queue,
            command_pool,
            setup_command_buffer,
            draw_command_buffer,
            present_image_views,
            vertex_shaders,
            fragment_shaders,
            render_pipelines: VkObjectStore::default(),
        })
    }
}

impl Drop for UiRenderer {
    fn drop(&mut self) {
        use logging::UnwrapReport;

        unsafe {
            self.device
                .device_wait_idle()
                .map_err(|err| {
                    logging::ErrorKind::VulkanError {
                        function_name: "device_wait_idle",
                        vk_code: err.as_raw(),
                    }
                    .into_error()
                })
                .unwrap_report();

            for &image_view in self.present_image_views.iter() {
                self.device.destroy_image_view(
                    image_view,
                    ALLOCATION_CALLBACKS,
                );
            }

            self.device.destroy_command_pool(
                self.command_pool,
                ALLOCATION_CALLBACKS,
            );

            self.render_pipelines.destroy(|pipeline| {
                self.device.destroy_pipeline(
                    pipeline,
                    ALLOCATION_CALLBACKS,
                )
            });

            self.swapchain_loader.destroy_swapchain(
                self.swapchain_khr,
                ALLOCATION_CALLBACKS,
            );

            self.vertex_shaders.destroy(|shader_module| {
                self.device.destroy_shader_module(
                    shader_module,
                    ALLOCATION_CALLBACKS,
                )
            });

            self.fragment_shaders.destroy(|shader_module| {
                self.device.destroy_shader_module(
                    shader_module,
                    ALLOCATION_CALLBACKS,
                )
            });

            self.device.destroy_device(ALLOCATION_CALLBACKS);

            self.surface_loader.destroy_surface(
                self.surface_khr,
                ALLOCATION_CALLBACKS,
            );
        }
    }
}
