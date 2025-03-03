use native_dialog::MessageDialog;
use rwh_05::HasRawWindowHandle;
use wgpu::CreateSurfaceError;
use winit::error::{EventLoopError, OsError};

pub enum Error {
    EventLoop(EventLoopError),
    WindowCreation(OsError),
    SurfaceCreation(CreateSurfaceError),
    NoWgpuAdapter,
}

struct ErrorMessage {
    title: &'static str,
    text: Box<str>,
}

impl Error {
    pub fn show_with_owner<W>(self, owner: &W)
    where
        W: HasRawWindowHandle,
    {
        let message = self.into_error_message();

        unsafe {
            MessageDialog::new()
                .set_title(message.title)
                .set_text(message.text.as_ref())
                .set_owner_handle(owner.raw_window_handle())
                .show_alert()
                .ok()
        };

        panic!("Fatal Error: {}", message.text);
    }

    pub fn show_no_owner(self) {
        let message = self.into_error_message();

        MessageDialog::new()
            .set_title(message.title)
            .set_text(message.text.as_ref())
            .show_alert()
            .ok();

        panic!("Fatal Error: {}", message.text);
    }

    fn into_error_message(self) -> ErrorMessage {
        match self {
            Self::EventLoop(error) => {
                let title = "Vector: Window Initialization Error";
                let text = match error {
                    EventLoopError::NotSupported(details) => {
                        format!(
                            "The current backend does not support this operation. Details: {}.",
                            details
                        )
                    }
                    EventLoopError::Os(os_error) => {
                        format!(
                            "An operating system error occurred during window initialization: {}.",
                            os_error
                        )
                    }
                    EventLoopError::RecreationAttempt => {
                        "Attempted to re-create an already running event loop.".to_string()
                    }
                    EventLoopError::ExitFailure(code) => {
                        format!("The application encountered an exit error with code: {}.", code)
                    }
                };

                ErrorMessage {
                    title,
                    text: text.into_boxed_str(),
                }
            }
            Self::WindowCreation(os_error) => ErrorMessage {
                title: "Vector: Window Creation Failed",
                text: format!(
                    "An error occurred while attempting to create a new window. The operating system reported: {}.",
                    os_error
                )
                .into_boxed_str(),
            },
            Self::SurfaceCreation(create_surface_error) => ErrorMessage {
                title: "Vector: WGPU Surface Creation Failed",
                text: format!(
                    "{}",
                    create_surface_error
                )
                .into_boxed_str(),
            },
            Self::NoWgpuAdapter => ErrorMessage { title:  "Vector: Graphics initialization failed",
            text: "Compataible WGPU Adapter not found".into() }
        }
    }
}

impl From<EventLoopError> for Error {
    #[inline(always)]
    fn from(value: EventLoopError) -> Self {
        Self::EventLoop(value)
    }
}

impl From<CreateSurfaceError> for Error {
    #[inline(always)]
    fn from(value: CreateSurfaceError) -> Self {
        Self::SurfaceCreation(value)
    }
}

#[macro_export]
macro_rules! throw {
    ($expr:expr) => {{
        match $expr {
            Ok(val) => val,
            Err(err) => {
                $crate::error::Error::from(err).show_no_owner();
                return;
            }
        }
    }};
    ($expr:expr, $owner:expr) => {{
        match $expr {
            Ok(val) => val,
            Err(err) => {
                $crate::error::Error::from(err)
                    .show_with_owner($owner);
                return;
            }
        }
    }};
}
