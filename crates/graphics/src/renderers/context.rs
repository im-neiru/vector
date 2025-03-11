use ash::{khr, vk};

pub(crate) struct Context {
    pub(crate) surface_loader: khr::surface::Instance,
    pub(crate) swapchain_loader: khr::swapchain::Device,
    pub(crate) surface_khr: vk::SurfaceKHR,
    pub(crate) swapchain_khr: vk::SwapchainKHR,
    pub(crate) device: ash::Device,
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();

            self.swapchain_loader
                .destroy_swapchain(self.swapchain_khr, None);

            self.device.destroy_device(None);

            self.surface_loader
                .destroy_surface(self.surface_khr, None);
        }
    }
}
