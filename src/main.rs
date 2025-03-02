mod app;
mod error;
mod window_state;

fn main() {
    throw!(app::App::default().run());
}
