use ash::{khr, vk};
use winit::raw_window_handle::{
    HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle,
};

#[cfg(target_family = "windows")]
use ash::khr::win32_surface;

pub struct Instance {
    entry: ash::Entry,
    instance: ash::Instance,
    surface_loader: khr::surface::Instance,
    #[cfg(target_family = "windows")]
    win32_instance: win32_surface::Instance,
}

const APP_VERSION: u32 = vk::make_api_version(0, 0, 1, 0);

const APP_NAME: &std::ffi::CStr = c"Vector";
const ENGINE_NAME: &std::ffi::CStr = c"Vector Engine";

const LAYER_NAMES: &[*const i8] =
    &[c"VK_LAYER_KHRONOS_validation".as_ptr()];

#[cfg(target_family = "windows")]
const EXTENSIONS_WIN32: &[*const i8] = &[
    khr::surface::NAME.as_ptr(),
    win32_surface::NAME.as_ptr(),
];

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
const EXTENSIONS_XCB: &[*const i8] = &[
    khr::surface::NAME.as_ptr(),
    khr::xcb_surface::NAME.as_ptr(),
];

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
const EXTENSIONS_XLIB: &[*const i8] = &[
    khr::surface::NAME.as_ptr(),
    khr::xlib_surface::NAME.as_ptr(),
];

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
const EXTENSIONS_WAYLAND: &[*const i8] = &[
    khr::surface::NAME.as_ptr(),
    khr::wayland_surface::NAME.as_ptr(),
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
                                .engine_version(APP_VERSION)
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

        let surface_loader =
            khr::surface::Instance::new(&entry, &instance);

        match display_handle {
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ))]
            RawDisplayHandle::Xlib(_) => EXTENSIONS_XLIB,
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ))]
            RawDisplayHandle::Xcb(_) => EXTENSIONS_XCB,
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ))]
            RawDisplayHandle::Wayland(_) => EXTENSIONS_WAYLAND,
            #[cfg(target_family = "windows")]
            RawDisplayHandle::Windows(_) => Ok({
                let win32_instance =
                    win32_surface::Instance::new(
                        &entry, &instance,
                    );

                Self {
                    entry,
                    instance,
                    win32_instance,
                    surface_loader,
                }
            }),
            _ => logging::ErrorKind::UnsupportedWindow
                .into_result(),
        }
    }

    #[inline]
    pub fn create_surface_with_window<H>(
        &self,
        handle: &H,
    ) -> logging::Result<crate::Surface>
    where
        H: HasDisplayHandle + HasWindowHandle,
    {
        let surface_khr = self.create_surface_khr(handle)?;

        Ok(crate::Surface { surface_khr })
    }
}

impl Instance {
    #[inline]
    fn create_surface_khr<H>(
        &self,
        handle: &H,
    ) -> logging::Result<vk::SurfaceKHR>
    where
        H: HasDisplayHandle + HasWindowHandle,
    {
        let display_handle = handle
            .display_handle()
            .map_err(|err| {
                logging::ErrorKind::DisplayHandle(err)
                    .into_error()
            })?
            .as_raw();

        let window_handle = handle
            .window_handle()
            .map_err(|err| {
                logging::ErrorKind::WindowHandle(err)
                    .into_error()
            })?
            .as_raw();

        #[allow(clippy::let_unit_value)]
        let surface_khr = match (display_handle, window_handle)
        {
            #[cfg(target_family = "windows")]
            (
                RawDisplayHandle::Windows(_),
                RawWindowHandle::Win32(handle),
            ) => unsafe {
                self.win32_instance.create_win32_surface(&
                    ash::vk::Win32SurfaceCreateInfoKHR::default()
                    .hinstance(handle.hinstance.ok_or(logging::ErrorKind::HInstanceIsNull.into_error())?.into())
                    .hwnd(handle.hwnd.into()),
                    None,
                ).map_err(|err|
                    logging::ErrorKind::VulkanError {
                        function_name: "create_win32_surface",
                        vk_code: err.as_raw()
                    }
                    .into_error()
                )?
            },
            _ => {
                return logging::ErrorKind::UnsupportedWindow
                    .into_result();
            }
        };

        let (physical_device, queue_family_index) =
            self.select_physical_device(surface_khr)?;

        // TODO: create logical device and queue

        Ok(surface_khr)
    }

    #[inline]
    fn select_physical_device(
        &self,
        surface_khr: vk::SurfaceKHR,
    ) -> logging::Result<(vk::PhysicalDevice, usize)> {
        let physical_devices = unsafe {
            self.instance.enumerate_physical_devices().map_err(
                |err| {
                    logging::ErrorKind::VulkanError {
                    function_name: "enumerate_physical_devices",
                    vk_code: err.as_raw(),
                }.into_error()
                },
            )?
        };

        physical_devices.iter().find_map(|device| {
            unsafe { self.instance
                .get_physical_device_queue_family_properties(
                    *device,
                )
                .iter()
                .enumerate() .find_map(|(index, info)| {
                    let supports_graphic_and_surface =
                        info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                            && self.surface_loader
                                .get_physical_device_surface_support(
                                    *device,
                                    index as u32,
                                    surface_khr,
                                )
                                .unwrap();
                    if supports_graphic_and_surface {
                        Some((*device, index))
                    } else {
                        None
                    }
                }) }
        })
        .ok_or(logging::ErrorKind::NoCompatibleDevice.into_error())
    }
}
