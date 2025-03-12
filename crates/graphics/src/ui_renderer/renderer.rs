use std::collections::BTreeMap;

use ash::{khr, vk};

use crate::spirv::{fs::FragmentShaderId, vs::VertexShaderId};

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
    vs: Option<super::shaders::VertexShaders>,
    fs: Option<super::shaders::FragmentShaders>,
    render_pipelines:
        Option<BTreeMap<RenderPipelineId, vk::Pipeline>>,
}

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
        use super::shaders::*;

        let vs = Some(unsafe { VertexShaders::new(&device)? });
        let fs =
            Some(unsafe { FragmentShaders::new(&device)? });

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
            vs,
            fs,
            render_pipelines: None,
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
                self.device
                    .destroy_image_view(image_view, None);
            }

            self.device
                .destroy_command_pool(self.command_pool, None);

            if let Some(render_pipelines) =
                self.render_pipelines.take()
            {
                for pipeline in render_pipelines.into_values() {
                    self.device
                        .destroy_pipeline(pipeline, None);
                }
            }

            self.swapchain_loader
                .destroy_swapchain(self.swapchain_khr, None);

            if let Some(vs) = self.vs.take() {
                vs.destroy(&self.device);
            }

            if let Some(fs) = self.fs.take() {
                fs.destroy(&self.device);
            }

            self.device.destroy_device(None);

            self.surface_loader
                .destroy_surface(self.surface_khr, None);
        }
    }
}
