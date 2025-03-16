use rspirv::{binary::ParseAction, dr::Instruction};

use crate::data_type::Type;

pub(crate) struct Layout {
    entry_points: Box<[EntryPoint]>,
    uniforms: Box<[UniformVariable]>,
    push_constants: Box<[PushConstantVariable]>,
}

pub(crate) struct UniformVariable {
    pub(crate) set: u32,
    pub(crate) binding: u32,
    pub(crate) name: Option<Box<str>>,
    pub(crate) r#type: Type,
}

pub(crate) struct LocationVariable {
    pub(crate) location: u32,
    pub(crate) name: Option<Box<str>>,
    pub(crate) r#type: Type,
}

pub(crate) struct EntryPoint {
    name: Option<Box<str>>,
    stage_params: StageParams,
}
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

pub(crate) struct LocalSize {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) z: u32,
}

pub(crate) struct PushConstantVariable {
    pub(crate) r#type: u32,
    pub(crate) name: Option<Box<str>>,
}

struct ConsumerImpl {}

impl rspirv::binary::Consumer for ConsumerImpl {
    fn initialize(&mut self) -> ParseAction {
        ParseAction::Continue
    }

    fn finalize(&mut self) -> ParseAction {
        ParseAction::Continue
    }

    fn consume_header(
        &mut self,
        module: rspirv::dr::ModuleHeader,
    ) -> ParseAction {
        ParseAction::Continue
    }

    fn consume_instruction(
        &mut self,
        inst: Instruction,
    ) -> ParseAction {
        match inst.class.opcode {
            _ => todo!(),
        }

        ParseAction::Continue
    }
}
