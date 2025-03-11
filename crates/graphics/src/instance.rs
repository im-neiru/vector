use ash::vk;
use winit::raw_window_handle::{
    HasDisplayHandle, RawDisplayHandle,
};

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
const EXTENSIONS_WIN32: &[*const i8] = &[
    ash::khr::surface::NAME.as_ptr(),
    ash::khr::win32_surface::NAME.as_ptr(),
];

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
const EXTENSIONS_XCB: &[*const i8] = &[
    ash::khr::surface::NAME.as_ptr(),
    ash::khr::xcb_surface::NAME.as_ptr(),
];

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
const EXTENSIONS_XLIB: &[*const i8] = &[
    ash::khr::surface::NAME.as_ptr(),
    ash::khr::xlib_surface::NAME.as_ptr(),
];

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
const EXTENSIONS_WAYLAND: &[*const i8] = &[
    ash::khr::surface::NAME.as_ptr(),
    ash::khr::wayland_surface::NAME.as_ptr(),
];

impl Instance {
    pub fn new<H>(display_handle: &H) -> logging::Result<Self>
    where
        H: HasDisplayHandle,
    {
        let entry = ash::Entry::linked();

        let display_handle = display_handle
            .display_handle()
            .map_err(|err| {
                logging::ErrorKind::DisplayHandle(err)
                    .into_error()
            })?
            .as_raw();

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
                            match display_handle {
                                #[cfg(any(
                                    target_os = "linux",
                                    target_os = "dragonfly",
                                    target_os = "freebsd",
                                    target_os = "netbsd",
                                    target_os = "openbsd"
                                ))]
                                RawDisplayHandle::Xlib(
                                    _
                                ) => EXTENSIONS_XLIB,
                                #[cfg(any(
                                    target_os = "linux",
                                    target_os = "dragonfly",
                                    target_os = "freebsd",
                                    target_os = "netbsd",
                                    target_os = "openbsd"
                                ))]
                                RawDisplayHandle::Xcb(
                                    _
                                ) => EXTENSIONS_XCB,
                                #[cfg(any(
                                    target_os = "linux",
                                    target_os = "dragonfly",
                                    target_os = "freebsd",
                                    target_os = "netbsd",
                                    target_os = "openbsd"
                                ))]
                                RawDisplayHandle::Wayland(
                                    _
                                ) => EXTENSIONS_WAYLAND,
                                #[cfg(target_family = "windows")]
                                RawDisplayHandle::Windows(
                                    _
                                ) => EXTENSIONS_WIN32,
                                _ => return logging::ErrorKind::UnsupportedWindow.into_result()
                            },
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
