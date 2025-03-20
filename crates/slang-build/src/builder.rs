use crate::slang::Slang;

pub struct Builder {
    slang: Slang,
}

impl Builder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            slang: unsafe { Slang::new() },
        }
    }
}
