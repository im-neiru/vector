use rspirv::{
    binary::{Consumer, ParseAction},
    dr::Operand,
    dr::{Instruction, ModuleHeader},
    spirv,
};
use std::collections::HashMap;

use crate::data_type::Type;

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

pub struct ConsumerImpl {
    entry_points: Vec<EntryPoint>,
    uniforms: Vec<UniformVariable>,
    push_constants: Vec<PushConstantVariable>,
    variable_decorations: HashMap<u32, VariableDecoration>,
}

impl ConsumerImpl {
    pub fn new() -> Self {
        Self {
            entry_points: Vec::new(),
            uniforms: Vec::new(),
            push_constants: Vec::new(),
            variable_decorations: HashMap::new(),
        }
    }

    pub fn finalize_layout(self) -> Layout {
        Layout {
            entry_points: self.entry_points.into_boxed_slice(),
            uniforms: self.uniforms.into_boxed_slice(),
            push_constants: self
                .push_constants
                .into_boxed_slice(),
        }
    }
}

impl Consumer for ConsumerImpl {
    fn initialize(&mut self) -> ParseAction {
        ParseAction::Continue
    }

    fn finalize(&mut self) -> ParseAction {
        ParseAction::Continue
    }

    fn consume_header(
        &mut self,
        _header: ModuleHeader,
    ) -> ParseAction {
        ParseAction::Continue
    }

    fn consume_instruction(
        &mut self,
        inst: Instruction,
    ) -> ParseAction {
        use spirv::Op;
        match inst.class.opcode {
            Op::EntryPoint => {
                if inst.operands.len() >= 3 {
                    if let Operand::ExecutionModel(exec_model) =
                        &inst.operands[0]
                    {
                        let stage_params = match exec_model {
                            spirv::ExecutionModel::Vertex => StageParams::Vertex {
                                inputs: Box::new([]),
                                outputs: Box::new([]),
                            },
                            spirv::ExecutionModel::Fragment => StageParams::Fragment {
                                inputs: Box::new([]),
                                outputs: Box::new([]),
                            },
                            spirv::ExecutionModel::GLCompute => StageParams::Compute {
                                local_size: LocalSize { x: 1, y: 1, z: 1 },
                            },
                            _ => StageParams::Vertex {
                                inputs: Box::new([]),
                                outputs: Box::new([]),
                            },
                        };

                        if let Operand::LiteralString(name) =
                            &inst.operands[2]
                        {
                            self.entry_points.push(
                                EntryPoint {
                                    name: Some(
                                        name.clone()
                                            .into_boxed_str(),
                                    ),
                                    stage_params,
                                },
                            );
                        }
                    }
                }
            }
            Op::Decorate => {
                if inst.operands.len() >= 2 {
                    if let Operand::IdRef(target_id) =
                        inst.operands[0]
                    {
                        let dec_entry = self
                            .variable_decorations
                            .entry(target_id)
                            .or_default();
                        if let Operand::Decoration(dec) =
                            &inst.operands[1]
                        {
                            match dec {
                                spirv::Decoration::Binding => {
                                    if let Some(Operand::LiteralBit32(binding)) = inst.operands.get(2) {
                                        dec_entry.binding = Some(*binding);
                                    }
                                }
                                spirv::Decoration::DescriptorSet => {
                                    if let Some(Operand::LiteralBit32(set)) = inst.operands.get(2) {
                                        dec_entry.set = Some(*set);
                                    }
                                }
                                spirv::Decoration::Location => {
                                    if let Some(Operand::LiteralBit32(location)) = inst.operands.get(2) {
                                        dec_entry.location = Some(*location);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Op::Name => {
                if inst.operands.len() >= 2 {
                    if let Operand::IdRef(target_id) =
                        inst.operands[0]
                    {
                        if let Operand::LiteralString(name) =
                            &inst.operands[1]
                        {
                            let dec_entry = self
                                .variable_decorations
                                .entry(target_id)
                                .or_default();
                            dec_entry.name = Some(name.clone());
                        }
                    }
                }
            }
            Op::Variable => {
                if inst.operands.len() >= 3 {
                    if let (
                        Operand::IdRef(result_id),
                        Operand::StorageClass(storage_class),
                    ) =
                        (&inst.operands[1], &inst.operands[2])
                    {
                        match storage_class {
                            spirv::StorageClass::Uniform => {
                                let dec = self.variable_decorations.get(result_id);
                                let binding = dec.and_then(|d| d.binding).unwrap_or(0);
                                let set = dec.and_then(|d| d.set).unwrap_or(0);
                                let name = dec.and_then(|d| d.name.clone())
                                    .map(|s| s.into_boxed_str());
                                self.uniforms.push(UniformVariable {
                                    set,
                                    binding,
                                    name,
                                    r#type: Type::Void,
                                });
                            }
                            spirv::StorageClass::PushConstant => {
                                let dec = self.variable_decorations.get(result_id);
                                let name = dec.and_then(|d| d.name.clone())
                                    .map(|s| s.into_boxed_str());
                                self.push_constants.push(PushConstantVariable {
                                    r#type: 0,
                                    name,
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
            Op::ExecutionMode => {
                if inst.operands.len() >= 2 {
                    if let Operand::IdRef(_entry_point_id) =
                        inst.operands[0]
                    {
                        if let Operand::ExecutionMode(
                            exec_mode,
                        ) = &inst.operands[1]
                        {
                            if *exec_mode == spirv::ExecutionMode::LocalSize && inst.operands.len() >= 5 {
                                if let (
                                    Operand::LiteralBit32(x),
                                    Operand::LiteralBit32(y),
                                    Operand::LiteralBit32(z)
                                ) = (
                                    &inst.operands[2],
                                    &inst.operands[3],
                                    &inst.operands[4]
                                ) {

                                    for ep in self.entry_points.iter_mut() {
                                        if let StageParams::Compute { ref mut local_size } = ep.stage_params {
                                            local_size.x = *x;
                                            local_size.y = *y;
                                            local_size.z = *z;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            _ => (),
        }
        ParseAction::Continue
    }
}

fn parse_spirv(binary: &[u32]) -> Layout {
    let mut consumer = ConsumerImpl::new();
    rspirv::binary::parse_words(binary, &mut consumer)
        .expect("Failed to parse SPIR-V");
    consumer.finalize_layout()
}

#[test]
fn test_parse() {
    let spirv_binary = {
        let bytes = include_bytes!(
            "../../graphics/src/spirv/vs_quad_emit_uv.spv"
        );

        bytes
            .chunks_exact(4)
            .map(|chunk| {
                u32::from_le_bytes(chunk.try_into().unwrap())
            })
            .collect::<Box<[u32]>>()
    };

    let layout = parse_spirv(&spirv_binary);
    println!("Parsed {:#?} entry points", layout);
}
