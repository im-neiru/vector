mod error;
mod panic_handler;

pub use error::{Error, ErrorKind, Result, UnwrapReport};
pub use panic_handler::{set_dialog_box_owner, set_panic_hook};
