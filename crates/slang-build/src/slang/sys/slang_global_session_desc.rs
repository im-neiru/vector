#[repr(C)]
pub(crate) struct SlangGlobalSessionDesc {
    structure_size: u32,
    api_version: u32,
    language_version: u32,
    enable_glsl: bool,
    reserved: [u32; 16],
}

impl SlangGlobalSessionDesc {
    const SLANG_API_VERSION: u32 = 0;
    const SLANG_LANGUAGE_VERSION_2025: u32 = 2025;
}

impl Default for SlangGlobalSessionDesc {
    fn default() -> Self {
        Self {
            structure_size: size_of::<Self>() as u32,
            api_version: Self::SLANG_API_VERSION,
            language_version: Self::SLANG_LANGUAGE_VERSION_2025,
            enable_glsl: false,
            reserved: [0; 16],
        }
    }
}
