use ash::{khr, vk};

pub(crate) struct SurfacedContext {
    pub(crate) surface_loader: khr::surface::Instance,
    pub(crate) swapchain_loader: khr::swapchain::Device,
    pub(crate) surface_khr: vk::SurfaceKHR,
    pub(crate) swapchain_khr: vk::SwapchainKHR,
    pub(crate) device: ash::Device,
    pub(crate) present_queue: vk::Queue,
    pub(crate) command_pool: vk::CommandPool,
    pub(crate) setup_command_buffer: vk::CommandBuffer,
    pub(crate) draw_command_buffer: vk::CommandBuffer,
    pub(crate) present_image_views: Box<[vk::ImageView]>,
}

impl super::Context for SurfacedContext {}

impl Drop for SurfacedContext {
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

            self.swapchain_loader
                .destroy_swapchain(self.swapchain_khr, None);

            self.device.destroy_device(None);

            self.surface_loader
                .destroy_surface(self.surface_khr, None);
        }
    }
}
