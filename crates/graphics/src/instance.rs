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

const EXTENSIONS_DEVICE: &[*const i8] = &[
    khr::swapchain::NAME.as_ptr(),
    khr::external_memory::NAME.as_ptr(),
    #[cfg(target_family = "unix")]
    khr::external_memory_fd::NAME.as_ptr(),
    #[cfg(target_family = "windows")]
    khr::external_memory_win32::NAME.as_ptr(),
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
        width: u32,
        height: u32,
    ) -> logging::Result<crate::Surface>
    where
        H: HasDisplayHandle + HasWindowHandle,
    {
        let surface_khr = self.create_surface_khr(handle)?;

        let (physical_device, queue_family_index) =
            self.select_physical_device(surface_khr)?;

        let features = vk::PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };

        let queue_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&[1.0]);

        let device = unsafe {
            self.instance
                .create_device(
                    physical_device,
                    &vk::DeviceCreateInfo::default()
                        .queue_create_infos(
                            std::slice::from_ref(&queue_info),
                        )
                        .enabled_features(&features)
                        .enabled_extension_names(
                            EXTENSIONS_DEVICE,
                        ),
                    None,
                )
                .map_err(|err| {
                    logging::ErrorKind::VulkanError {
                        function_name: "create_device",
                        vk_code: err.as_raw(),
                    }
                    .into_error()
                })?
        };

        let present_queue = unsafe {
            device.get_device_queue(queue_family_index, 0)
        };

        let surface_format = unsafe {
        self.surface_loader
            .get_physical_device_surface_formats(
                physical_device,
                surface_khr,
            )
            .map_err(|err| {
                logging::ErrorKind::VulkanError {
                    function_name: "get_physical_device_surface_formats",
                    vk_code: err.as_raw(),
                }
                .into_error()
            })?
        }
        .into_iter()
        .find(|format| {
            format.format == vk::Format::B8G8R8A8_UNORM
        })

        .ok_or(
            logging::ErrorKind::UnsupportedSurfaceFormat
                .into_error(),
        )?;

        let surface_capabilities = unsafe {
            self
        .surface_loader
        .get_physical_device_surface_capabilities(
            physical_device, surface_khr,
        ).map_err(|err| {
            logging::ErrorKind::VulkanError {
                function_name: "get_physical_device_surface_capabilities",
                vk_code: err.as_raw(),
            }
            .into_error()
        })?
        };

        let mut desired_image_count =
            surface_capabilities.min_image_count + 1;
        if surface_capabilities.max_image_count > 0
            && desired_image_count
                > surface_capabilities.max_image_count
        {
            desired_image_count =
                surface_capabilities.max_image_count;
        }

        let surface_resolution =
            match surface_capabilities.current_extent.width {
                u32::MAX => vk::Extent2D { width, height },
                _ => surface_capabilities.current_extent,
            };

        let pre_transform = if surface_capabilities
            .supported_transforms
            .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
        {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            surface_capabilities.current_transform
        };

        let present_modes = unsafe {
            self.surface_loader
                .get_physical_device_surface_present_modes(
                    physical_device,
                    surface_khr,
                )
                .map_err(|err| {
                    logging::ErrorKind::VulkanError {
                        function_name: "get_physical_device_surface_present_modes",
                        vk_code: err.as_raw(),
                    }
                    .into_error()
                })?
        };

        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO);

        let swapchain_loader = khr::swapchain::Device::new(
            &self.instance,
            &device,
        );

        let swapchain_create_info =
            vk::SwapchainCreateInfoKHR::default()
                .surface(surface_khr)
                .min_image_count(desired_image_count)
                .image_color_space(surface_format.color_space)
                .image_format(surface_format.format)
                .image_extent(surface_resolution)
                .image_usage(
                    vk::ImageUsageFlags::COLOR_ATTACHMENT,
                )
                .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                .pre_transform(pre_transform)
                .composite_alpha(
                    vk::CompositeAlphaFlagsKHR::OPAQUE,
                )
                .present_mode(present_mode)
                .clipped(true)
                .image_array_layers(1);

        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .map_err(|err| {
                    logging::ErrorKind::VulkanError {
                        function_name: "create_swapchain",
                        vk_code: err.as_raw(),
                    }
                    .into_error()
                })?
        };

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

        Ok(surface_khr)
    }

    #[inline]
    fn select_physical_device(
        &self,
        surface_khr: vk::SurfaceKHR,
    ) -> logging::Result<(vk::PhysicalDevice, u32)> {
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
                        info.queue_flags.contains(vk::QueueFlags::GRAPHICS) &&
                        info.queue_flags.contains(vk::QueueFlags::COMPUTE)
                            && self.surface_loader
                                .get_physical_device_surface_support(
                                    *device,
                                    index as u32,
                                    surface_khr,
                                )
                                .unwrap();
                    if supports_graphic_and_surface {
                        Some((*device, index as u32))
                    } else {
                        None
                    }
                }) }
        })
        .ok_or(logging::ErrorKind::NoCompatibleDevice.into_error())
    }
}
