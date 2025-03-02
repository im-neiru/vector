use native_dialog::MessageDialog;
use raw_window_handle::{HasRawWindowHandle, HasWindowHandle};
use winit::error::EventLoopError;

pub(crate) enum Error {
    EventLoopError(EventLoopError),
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
                            "An operating system error occurred during window initialization: {:?}.",
                            os_error
                        )
                    }
                    EventLoopError::RecreationAttempt => {
                        "Attempted to re-create an already running event loop. Please ensure that the event loop is only initialized once.".to_string()
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
        }
    }
}
