mod consumer;
mod data_type;

pub use data_type::*;

#[derive(Clone, Debug)]
pub(crate) struct Layout {
    pub entry_points: Box<[EntryPoint]>,
    pub uniforms: Box<[UniformVariable]>,
    pub push_constants: Box<[PushConstantVariable]>,
}

#[derive(Clone, Debug)]
pub(crate) struct UniformVariable {
    pub set: u32,
    pub binding: u32,
    pub name: Option<Box<str>>,
    pub r#type: Type,
}

#[derive(Clone, Debug)]
pub(crate) struct LocationVariable {
    pub location: u32,
    pub name: Option<Box<str>>,
    pub r#type: Type,
}

#[derive(Clone, Debug)]
pub(crate) struct EntryPoint {
    pub name: Option<Box<str>>,
    pub stage_params: StageParams,
}

#[derive(Clone, Debug)]
pub(crate) enum StageParams {
    Vertex {
        inputs: Box<[LocationVariable]>,
        outputs: Box<[LocationVariable]>,
    },
    Fragment {
        inputs: Box<[LocationVariable]>,
        outputs: Box<[LocationVariable]>,
    },
    Compute {
        local_size: LocalSize,
    },
}

#[derive(Clone, Debug)]
pub(crate) struct LocalSize {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Clone, Debug)]
pub(crate) struct PushConstantVariable {
    pub r#type: u32,
    pub name: Option<Box<str>>,
}

#[derive(Default)]
struct VariableDecoration {
    binding: Option<u32>,
    set: Option<u32>,
    location: Option<u32>,
    name: Option<String>,
}
