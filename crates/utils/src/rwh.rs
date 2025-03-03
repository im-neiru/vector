pub fn convert_display_handle_06(
    handle: rwh_05::RawDisplayHandle,
) -> rwh_06::RawDisplayHandle {
    use std::ptr::NonNull;

    match handle {
        rwh_05::RawDisplayHandle::UiKit(_) => {
            rwh_06::RawDisplayHandle::UiKit(
                rwh_06::UiKitDisplayHandle::new(),
            )
        }
        rwh_05::RawDisplayHandle::AppKit(_) => {
            rwh_06::RawDisplayHandle::AppKit(
                rwh_06::AppKitDisplayHandle::new(),
            )
        }
        rwh_05::RawDisplayHandle::Orbital(_) => {
            rwh_06::RawDisplayHandle::Orbital(
                rwh_06::OrbitalDisplayHandle::new(),
            )
        }
        rwh_05::RawDisplayHandle::Xlib(xlib) => {
            rwh_06::RawDisplayHandle::Xlib(
                rwh_06::XlibDisplayHandle::new(
                    NonNull::new(xlib.display),
                    xlib.screen,
                ),
            )
        }
        rwh_05::RawDisplayHandle::Xcb(xcb) => {
            rwh_06::RawDisplayHandle::Xcb(
                rwh_06::XcbDisplayHandle::new(
                    NonNull::new(xcb.connection),
                    xcb.screen,
                ),
            )
        }
        rwh_05::RawDisplayHandle::Wayland(wayland) => {
            rwh_06::RawDisplayHandle::Wayland(
                rwh_06::WaylandDisplayHandle::new(
                    NonNull::new(wayland.display).expect(
                        "Wayland display handle is null",
                    ),
                ),
            )
        }
        rwh_05::RawDisplayHandle::Drm(drm) => {
            rwh_06::RawDisplayHandle::Drm(
                rwh_06::DrmDisplayHandle::new(drm.fd),
            )
        }
        rwh_05::RawDisplayHandle::Gbm(gbm) => {
            rwh_06::RawDisplayHandle::Gbm(
                rwh_06::GbmDisplayHandle::new(
                    NonNull::new(gbm.gbm_device)
                        .expect("GBM display handle is null"),
                ),
            )
        }
        rwh_05::RawDisplayHandle::Windows(_) => {
            rwh_06::RawDisplayHandle::Windows(
                rwh_06::WindowsDisplayHandle::new(),
            )
        }
        rwh_05::RawDisplayHandle::Web(_) => {
            rwh_06::RawDisplayHandle::Web(
                rwh_06::WebDisplayHandle::new(),
            )
        }
        rwh_05::RawDisplayHandle::Android(_) => {
            rwh_06::RawDisplayHandle::Android(
                rwh_06::AndroidDisplayHandle::new(),
            )
        }
        rwh_05::RawDisplayHandle::Haiku(_) => {
            rwh_06::RawDisplayHandle::Haiku(
                rwh_06::HaikuDisplayHandle::new(),
            )
        }
        _ => panic!("Unsupported display handle"),
    }
}

pub fn convert_window_handle_06(
    handle: rwh_05::RawWindowHandle,
) -> rwh_06::RawWindowHandle {
    use std::num::{NonZeroIsize, NonZeroU32};
    use std::ptr::NonNull;

    match handle {
        rwh_05::RawWindowHandle::UiKit(uikit) => {
            rwh_06::RawWindowHandle::UiKit(
                rwh_06::UiKitWindowHandle::new(
                    NonNull::new(uikit.ui_view)
                        .expect("UIKit UIView is null"),
                ),
            )
        }
        rwh_05::RawWindowHandle::AppKit(appkit) => {
            rwh_06::RawWindowHandle::AppKit(
                rwh_06::AppKitWindowHandle::new(
                    NonNull::new(appkit.ns_window)
                        .expect("AppKit NSWindow is null"),
                ),
            )
        }
        rwh_05::RawWindowHandle::Orbital(orbital) => {
            rwh_06::RawWindowHandle::Orbital(
                rwh_06::OrbitalWindowHandle::new(
                    NonNull::new(orbital.window)
                        .expect("Orbital window is null"),
                ),
            )
        }
        rwh_05::RawWindowHandle::Xlib(xlib) => {
            rwh_06::RawWindowHandle::Xlib(
                rwh_06::XlibWindowHandle::new(xlib.window),
            )
        }
        rwh_05::RawWindowHandle::Xcb(xcb) => {
            rwh_06::RawWindowHandle::Xcb(
                rwh_06::XcbWindowHandle::new(
                    NonZeroU32::new(xcb.window)
                        .expect("Xcb window is null"),
                ),
            )
        }
        rwh_05::RawWindowHandle::Wayland(wayland) => {
            rwh_06::RawWindowHandle::Wayland(
                rwh_06::WaylandWindowHandle::new(
                    NonNull::new(wayland.surface)
                        .expect("Wayland surface is null"),
                ),
            )
        }
        rwh_05::RawWindowHandle::Win32(win) => {
            rwh_06::RawWindowHandle::Win32({
                let mut handle = rwh_06::Win32WindowHandle::new(
                    NonZeroIsize::new(win.hwnd as isize)
                        .expect("Win32 window handle is null"),
                );

                handle.hinstance =
                    NonZeroIsize::new(win.hinstance as isize);

                handle
            })
        }
        rwh_05::RawWindowHandle::Web(web) => {
            rwh_06::RawWindowHandle::Web(
                rwh_06::WebWindowHandle::new(web.id),
            )
        }
        rwh_05::RawWindowHandle::AndroidNdk(android) => {
            rwh_06::RawWindowHandle::AndroidNdk(
                rwh_06::AndroidNdkWindowHandle::new(
                    NonNull::new(android.a_native_window)
                        .expect(
                            "Android ANativeWindow is null",
                        ),
                ),
            )
        }
        rwh_05::RawWindowHandle::Haiku(haiku) => {
            rwh_06::RawWindowHandle::Haiku(
                rwh_06::HaikuWindowHandle::new(
                    NonNull::new(haiku.b_window)
                        .expect("Haiku BWindow is null"),
                ),
            )
        }
        _ => panic!("Unsupported window handle"),
    }
}
