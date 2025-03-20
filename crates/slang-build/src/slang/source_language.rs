#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SlangSourceLanguage {
    Unknown = 0,
    Slang = 1,
    Hlsl = 2,
    Glsl = 3,
    C = 4,
    Cpp = 5,
    Cuda = 6,
    Spirv = 7,
    Metal = 8,
    Wgsl = 9,
    CountOf = 10,
}
