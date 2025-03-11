use ash::vk;

pub struct Surface {
    pub(crate) surface_khr: vk::SurfaceKHR,
}
