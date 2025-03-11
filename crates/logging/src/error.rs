use std::panic::{Location, panic_any};

#[derive(Debug)]
pub struct Error<'a> {
    pub(crate) location: &'a Location<'a>,
    pub(crate) kind: ErrorKind,
}

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("Window create error: {0}")]
    WindowCreation(winit::error::OsError),
    #[error("Surface create error: {0}")]
    CreateSurface(wgpu::CreateSurfaceError),
    #[error("No compatible wgpu adapter found")]
    NoWgpuAdapter,
    #[error("Request device error: {0}")]
    RequestDeviceError(wgpu::RequestDeviceError),
    #[error("Size must not exceed `Size::MAX`")]
    SizeExceedMaxSize,
    #[error("Size cannot be negative")]
    NegativeSize,
    #[error("Display handle error: {0}")]
    DisplayHandle(winit::raw_window_handle::HandleError),
    #[error("Window handle error: {0}")]
    WindowHandle(winit::raw_window_handle::HandleError),
    #[error("Window platform is not supported")]
    UnsupportedWindow,
    #[cfg(target_family = "windows")]
    #[error("HINSTANCE is null")]
    HInstanceIsNull,
    #[error(
        "Vulkan function named {function_name} returned error code {vk_code}"
    )]
    VulkanError {
        function_name: &'static str,
        vk_code: i32,
    },
    #[error("No compatible Vulkan physical device")]
    NoCompatibleDevice,
}

pub type Result<T> =
    std::result::Result<T, self::Error<'static>>;

impl ErrorKind {
    #[inline(always)]
    #[track_caller]
    pub const fn into_error(self) -> self::Error<'static> {
        Error {
            location: Location::caller(),
            kind: self,
        }
    }

    #[inline(always)]
    #[track_caller]
    pub const fn into_result<T>(self) -> self::Result<T> {
        Err(Error {
            location: Location::caller(),
            kind: self,
        })
    }
}

pub trait UnwrapReport<T> {
    fn unwrap_report(self) -> T;
}

impl<T> UnwrapReport<T> for self::Result<T> {
    #[inline(always)]
    fn unwrap_report(self) -> T {
        match self {
            Ok(value) => value,
            Err(error) => panic_any(error),
        }
    }
}
