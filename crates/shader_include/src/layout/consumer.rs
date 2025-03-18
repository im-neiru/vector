use rspirv::{
    binary::{Consumer, ParseAction},
    dr::Operand,
    dr::{Instruction, ModuleHeader},
    spirv,
};

use utils::U32BufferMap;

pub struct ConsumerImpl {
    entry_points: Vec<super::EntryPoint>,
    field_declarations: U32BufferMap<u32, 8>,
}

impl ConsumerImpl {
    pub fn new() -> Self {
        Self {
            field_declarations: U32BufferMap::new(),
            entry_points: Vec::with_capacity(1),
        }
    }

    pub fn finalize_layout(self) -> super::Layout {
        todo!()
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
                            spirv::ExecutionModel::Vertex => super::StageParams::Vertex {
                                inputs: Box::new([]),
                                outputs: Box::new([]),
                            },
                            spirv::ExecutionModel::Fragment => super::StageParams::Fragment {
                                inputs: Box::new([]),
                                outputs: Box::new([]),
                            },
                            spirv::ExecutionModel::GLCompute => super::StageParams::Compute {
                                local_size: super::LocalSize { x: 1, y: 1, z: 1 },
                            },
                            _ => super::StageParams::Vertex {
                                inputs: Box::new([]),
                                outputs: Box::new([]),
                            },
                        };

                        if let Operand::LiteralString(name) =
                            &inst.operands[2]
                        {
                            self.entry_points.push(
                                super::EntryPoint {
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
                        // println!("{:?}", inst.operands);
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
            Op::TypePointer => {
                println!(
                    "{:?} = OpTypePointer {:?}",
                    inst.result_id, inst.operands
                );
            }
            Op::TypeStruct => {
                println!(
                    "{:?} = OpTypeStruct {:?}",
                    inst.result_id, inst.operands
                );
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
            "../../../graphics/src/spirv/vs_quad_emit_uv.spv"
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
