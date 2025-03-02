use native_dialog::MessageDialog;
use raw_window_handle::{HasRawWindowHandle, HasWindowHandle};
use winit::error::{EventLoopError, OsError};

pub(crate) enum Error {
    EventLoopError(EventLoopError),
    WindowCreationFailed(OsError),
}

struct ErrorMessage {
    title: &'static str,
    text: Box<str>,
}

impl Error {
    pub(crate) fn show_with_owner<W>(self, owner: &W)
    where
        W: HasWindowHandle + HasRawWindowHandle,
    {
        let message = self.into_error_message();

        if MessageDialog::new()
            .set_title(message.title)
            .set_text(message.text.as_ref())
            .set_owner(owner)
            .show_alert()
            .is_err()
        {
            panic!("Fatal Error: {}", message.text);
        }
    }

    pub(crate) fn show_no_owner(self) {
        let message = self.into_error_message();

        if MessageDialog::new()
            .set_title(message.title)
            .set_text(message.text.as_ref())
            .show_alert()
            .is_err()
        {
            panic!("Fatal Error: {}", message.text);
        }
    }

    fn into_error_message(self) -> ErrorMessage {
        match self {
            Self::EventLoopError(error) => {
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
            Self::WindowCreationFailed(os_error) => ErrorMessage {
                title: "Vector: Window Creation Failed",
                text: format!(
                    "An error occurred while attempting to create a new window. The operating system reported: {}.",
                    os_error
                )
                .into_boxed_str(),
            },

        }
    }
}

impl From<EventLoopError> for Error {
    #[inline(always)]
    fn from(value: EventLoopError) -> Self {
        Self::EventLoopError(value)
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
                $crate::error::Error::from(err).show_with_owner($owner);
                return;
            }
        }
    }};
}
