use ash::vk;

pub struct Instance {
    entry: ash::Entry,
    instance: ash::Instance,
}

const APP_VERSION: u32 = vk::make_api_version(0, 0, 1, 0);

const APP_NAME: &std::ffi::CStr = c"Vector";
const ENGINE_NAME: &std::ffi::CStr = c"Vector Engine";

const LAYER_NAMES: &[*const i8] =
    &[c"VK_LAYER_KHRONOS_validation".as_ptr()];

#[cfg(target_family = "windows")]
const EXTENSION_NAMES: &[*const i8] = &[
    ash::khr::surface::NAME.as_ptr(),
    ash::khr::win32_surface::NAME.as_ptr(),
];

#[cfg(target_os = "linux")]
const EXTENSION_NAMES: &[*const i8] = &[
    ash::khr::surface::NAME.as_ptr(),
    ash::khr::xcb_surface::NAME.as_ptr(),
    ash::khr::xlib_surface::NAME.as_ptr(),
    ash::khr::wayland_surface::NAME.as_ptr(),
];

impl Instance {
    pub fn new() -> logging::Result<Self> {
        let entry = ash::Entry::linked();

        let instance = unsafe {
            entry
                .create_instance(
                    &vk::InstanceCreateInfo::default()
                        .application_info(
                            &vk::ApplicationInfo::default()
                                .api_version(
                                    vk::API_VERSION_1_3,
                                )
                                .application_version(
                                    APP_VERSION,
                                )
                                .application_name(APP_NAME)
                                .engine_name(ENGINE_NAME),
                        )
                        .enabled_layer_names(LAYER_NAMES)
                        .enabled_extension_names(
                            EXTENSION_NAMES,
                        ),
                    None,
                )
                .map_err(|err| {
                    logging::ErrorKind::VulkanError {
                        function_name: "create_instance",
                        vk_code: err.as_raw(),
                    }
                    .into_error()
                })?
        };

        Ok(Self { entry, instance })
    }
}
