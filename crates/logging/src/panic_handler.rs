use std::sync::{LazyLock, Mutex};

struct PanicHandlerState {
    owner: Option<winit::raw_window_handle_05::RawWindowHandle>,
}

unsafe impl Sync for PanicHandlerState {}
unsafe impl Send for PanicHandlerState {}

static STATE: LazyLock<Mutex<PanicHandlerState>> =
    LazyLock::new(|| {
        Mutex::new(PanicHandlerState { owner: None })
    });

pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        if let Some(proper_error) =
            info.payload().downcast_ref::<crate::Error>()
        {
            let message = format!(
                "at {}: {}",
                proper_error.location, proper_error.kind,
            );

            let mut dialog =
                native_dialog::MessageDialog::new()
                    .set_title("Vector error")
                    .set_text(message.as_str());

            if let Some(owner) = STATE.lock().unwrap().owner {
                dialog =
                    unsafe { dialog.set_owner_handle(owner) };
            }

            if dialog.show_alert().is_err() {
                eprintln!("{message}",)
            }
        } else {
            eprintln!(
                "{:?}",
                info.payload().downcast_ref::<String>()
            );
        }
    }));
}

pub fn set_dialog_box_owner(
    handle: Option<
        winit::raw_window_handle_05::RawWindowHandle,
    >,
) {
    STATE.lock().unwrap().owner = handle;
}
