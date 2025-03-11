use std::pin::Pin;

pub struct UiRenderer {
    pub(crate) context: Pin<Box<dyn super::Context>>,
}
