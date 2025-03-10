fn main() {
    logging::set_panic_hook();
    ui::App::new().unwrap().run().unwrap();
}
