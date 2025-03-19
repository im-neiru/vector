#[derive(Clone, Debug)]
pub(crate) enum Type {
    Void,
    Bool,
    Int32,
    UInt32,
    Float32,
    Vec2,
    Vec3,
    Vec4,
    Mat3,
    Mat4,
    Image2D {
        depth: bool,
        sampled: bool,
        format: ImageFormat,
    },
    Sampler,
    SampledImage {
        image_type_index: u32,
    },
    Array {
        element_type_index: u32,
        length: Option<u32>,
    },
    Struct {
        name: Option<String>,
        members: Box<[StructMember]>,
    },
    Pointer {
        storage_class: StorageClass,
        pointed_type_index: u32,
    },
}

#[derive(Clone, Debug)]
pub(crate) struct StructMember {
    pub name: Option<String>,
    pub r#type: Type,
    pub offset: Option<u32>,
    pub row_major: bool,
    pub stride: u32,
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub(crate) enum StorageClass {
    Unknown,
    Uniform,
    UniformConstant,
    PushConstant,
    Input,
    Output,
    Workgroup,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Debug)]
pub(crate) enum ImageFormat {
    Unknown = 0,
    Rgba32f = 1,
    Rgba16f = 2,
    R32f = 3,
    Rgba8 = 4,
    Rgba8Snorm = 5,
    Rg32f = 6,
    Rg16f = 7,
    R11fG11fB10f = 8,
    R16f = 9,
    Rgba16 = 10,
    Rgb10A2 = 11,
    Rg16 = 12,
    Rg8 = 13,
    R16 = 14,
    R8 = 15,
    Rgba16Snorm = 16,
    Rg16Snorm = 17,
    Rg8Snorm = 18,
    R16Snorm = 19,
    R8Snorm = 20,
    Rgba32i = 21,
    Rgba16i = 22,
    Rgba8i = 23,
    R32i = 24,
    Rg32i = 25,
    Rg16i = 26,
    Rg8i = 27,
    R16i = 28,
    R8i = 29,
    Rgba32ui = 30,
    Rgba16ui = 31,
    Rgba8ui = 32,
    R32ui = 33,
    Rgb10a2ui = 34,
    Rg32ui = 35,
    Rg16ui = 36,
    Rg8ui = 37,
    R16ui = 38,
    R8ui = 39,
    R64ui = 40,
    R64i = 41,
}
