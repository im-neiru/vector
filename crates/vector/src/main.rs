use std::panic::panic_any;

fn main() {
    logging::set_panic_hook();

    match ui::App::new() {
        Ok(app) => {
            if let Err(err) = app.run() {
                panic_any(err)
            }
        }
        Err(err) => panic_any(err),
    }
}
