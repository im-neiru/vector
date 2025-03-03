mod app;
mod error;
mod graphics_state;
mod utils;
mod window_state;

fn main() {
    throw!(app::App::new().run());
}
