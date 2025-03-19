use rspirv::{
    binary::{Consumer, ParseAction},
    dr::Operand,
    dr::{Instruction, ModuleHeader},
    spirv,
};

use utils::U32BufferMap;

pub struct ConsumerImpl {
    entry_points: Vec<super::EntryPoint>,
    type_declarations: U32BufferMap<TypeDeclaration, 8>,
}

enum TypeDeclaration {
    Void,
    Bool,
    Int32,
    UInt32,
    Float32,
    Vec2 { scalar_type: u32 },
    Vec3 { scalar_type: u32 },
    Vec4 { scalar_type: u32 },
    Mat2 { column_type: u32 },
    Mat3 { column_type: u32 },
    Mat4 { column_type: u32 },
    Struct { members_type: Vec<u32> },
}

impl ConsumerImpl {
    pub fn new() -> Self {
        Self {
            type_declarations: U32BufferMap::new(),
            entry_points: Vec::with_capacity(1),
        }
    }

    pub fn finalize_layout(self) {
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
        use super::parse_error::ParseError;
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
            // Op::Decorate => {
            //     if inst.operands.len() >= 2 {
            //         if let Operand::IdRef(target_id) =
            //             inst.operands[0]
            //         {
            //             let dec_entry = self
            //                 .variable_decorations
            //                 .entry(target_id)
            //                 .or_default();
            //             if let Operand::Decoration(dec) =
            //                 &inst.operands[1]
            //             {
            //                 match dec {
            //                     spirv::Decoration::Binding => {
            //                         if let Some(Operand::LiteralBit32(binding)) = inst.operands.get(2) {
            //                             dec_entry.binding = Some(*binding);
            //                         }
            //                     }
            //                     spirv::Decoration::DescriptorSet => {
            //                         if let Some(Operand::LiteralBit32(set)) = inst.operands.get(2) {
            //                             dec_entry.set = Some(*set);
            //                         }
            //                     }
            //                     spirv::Decoration::Location => {
            //                         if let Some(Operand::LiteralBit32(location)) = inst.operands.get(2) {
            //                             dec_entry.location = Some(*location);
            //                         }
            //                     }
            //                     _ => {}
            //                 }
            //             }
            //         }
            //     }
            // }
            // Op::Name => {
            //     if inst.operands.len() >= 2 {
            //         if let Operand::IdRef(target_id) =
            //             inst.operands[0]
            //         {
            //             // println!("{:?}", inst.operands);
            //             if let Operand::LiteralString(name) =
            //                 &inst.operands[1]
            //             {
            //                 let dec_entry = self
            //                     .variable_decorations
            //                     .entry(target_id)
            //                     .or_default();
            //                 dec_entry.name = Some(name.clone());
            //             }
            //         }
            //     }
            // }
            // Op::Variable => {
            //     if inst.operands.len() >= 3 {
            //         if let (
            //             Operand::IdRef(result_id),
            //             Operand::StorageClass(storage_class),
            //         ) =
            //             (&inst.operands[1], &inst.operands[2])
            //         {
            //             match storage_class {
            //                 spirv::StorageClass::Uniform => {
            //                     let dec = self.variable_decorations.get(result_id);
            //                     let binding = dec.and_then(|d| d.binding).unwrap_or(0);
            //                     let set = dec.and_then(|d| d.set).unwrap_or(0);
            //                     let name = dec.and_then(|d| d.name.clone())
            //                         .map(|s| s.into_boxed_str());
            //                     self.uniforms.push(UniformVariable {
            //                         set,
            //                         binding,
            //                         name,
            //                         r#type: Type::Void,
            //                     });
            //                 }
            //                 spirv::StorageClass::PushConstant => {
            //                     let dec = self.variable_decorations.get(result_id);
            //                     let name = dec.and_then(|d| d.name.clone())
            //                         .map(|s| s.into_boxed_str());
            //                     self.push_constants.push(PushConstantVariable {
            //                         r#type: 0,
            //                         name,
            //                     });
            //                 }
            //                 _ => {}
            //             }
            //         }
            //     }
            // }
            Op::TypePointer => {
                println!(
                    "{:?} = {:?} {:?}",
                    inst.result_id,
                    inst.class.opname,
                    inst.operands
                );
            }
            Op::TypeFloat => {
                if let Some(result_id) = inst.result_id {
                    match inst.operands.first() {
                        Some(Operand::LiteralBit32(32)) => {
                            self.type_declarations.insert(
                                result_id,
                                TypeDeclaration::Float32,
                            );
                        }
                        Some(Operand::LiteralBit32(width)) => {
                            return ParseAction::Error(
                                Box::new(ParseError::InvalidFloatWidth(
                                    *width,
                                )),
                            );
                        }
                        _ => {
                            return ParseAction::Error(
                                Box::new(
                                    ParseError::ExpectOperand(
                                        inst.class.opcode,
                                    ),
                                ),
                            );
                        }
                    }
                } else {
                    return ParseAction::Error(Box::new(
                        ParseError::NoResultId(
                            inst.class.opcode,
                        ),
                    ));
                }
            }
            Op::TypeVector => {
                if let Some(result_id) = inst.result_id {
                    match (
                        inst.operands.first(),
                        inst.operands.get(1),
                    ) {
                        (
                            Some(&Operand::IdRef(scalar_type)),
                            Some(&Operand::LiteralBit32(2)),
                        ) => {
                            self.type_declarations.insert(
                                result_id,
                                TypeDeclaration::Vec2 {
                                    scalar_type,
                                },
                            );
                        }
                        (
                            Some(&Operand::IdRef(scalar_type)),
                            Some(&Operand::LiteralBit32(3)),
                        ) => {
                            self.type_declarations.insert(
                                result_id,
                                TypeDeclaration::Vec3 {
                                    scalar_type,
                                },
                            );
                        }
                        (
                            Some(&Operand::IdRef(scalar_type)),
                            Some(&Operand::LiteralBit32(4)),
                        ) => {
                            self.type_declarations.insert(
                                result_id,
                                TypeDeclaration::Vec4 {
                                    scalar_type,
                                },
                            );
                        }
                        (
                            Some(&Operand::IdRef(_)),
                            Some(&Operand::LiteralBit32(
                                column_count,
                            )),
                        ) => {
                            return ParseAction::Error(
                                Box::new(
                                    ParseError::InvalidVectorSize(
                                        column_count
                                    ),
                                ),
                            );
                        }
                        _ => {
                            return ParseAction::Error(
                                Box::new(
                                    ParseError::ExpectOperand(
                                        inst.class.opcode,
                                    ),
                                ),
                            );
                        }
                    }
                } else {
                    return ParseAction::Error(Box::new(
                        ParseError::NoResultId(
                            inst.class.opcode,
                        ),
                    ));
                }
            }
            Op::TypeMatrix => {
                if let Some(result_id) = inst.result_id {
                    match (
                        inst.operands.first(),
                        inst.operands.get(1),
                    ) {
                        (
                            Some(&Operand::IdRef(column_type)),
                            Some(&Operand::LiteralBit32(2)),
                        ) => {
                            self.type_declarations.insert(
                                result_id,
                                TypeDeclaration::Mat2 {
                                    column_type,
                                },
                            );
                        }
                        (
                            Some(&Operand::IdRef(column_type)),
                            Some(&Operand::LiteralBit32(3)),
                        ) => {
                            self.type_declarations.insert(
                                result_id,
                                TypeDeclaration::Mat3 {
                                    column_type,
                                },
                            );
                        }
                        (
                            Some(&Operand::IdRef(column_type)),
                            Some(&Operand::LiteralBit32(4)),
                        ) => {
                            self.type_declarations.insert(
                                result_id,
                                TypeDeclaration::Mat4 {
                                    column_type,
                                },
                            );
                        }
                        (
                            Some(&Operand::IdRef(_)),
                            Some(&Operand::LiteralBit32(
                                column_count,
                            )),
                        ) => {
                            return ParseAction::Error(
                                Box::new(
                                    ParseError::InvalidVectorSize(
                                        column_count
                                    ),
                                ),
                            );
                        }
                        _ => {
                            return ParseAction::Error(
                                Box::new(
                                    ParseError::ExpectOperand(
                                        inst.class.opcode,
                                    ),
                                ),
                            );
                        }
                    }
                } else {
                    return ParseAction::Error(Box::new(
                        ParseError::NoResultId(
                            inst.class.opcode,
                        ),
                    ));
                }
            }
            Op::TypeStruct => {
                if let Some(result_id) = inst.result_id {
                    self.type_declarations.insert(
                        result_id,
                        TypeDeclaration::Struct {
                            members_type: inst
                                .operands
                                .iter()
                                .filter_map(|operand| {
                                    operand.id_ref_any()
                                })
                                .collect(),
                        },
                    );
                } else {
                    return ParseAction::Error(Box::new(
                        ParseError::NoResultId(
                            inst.class.opcode,
                        ),
                    ));
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
                                        if let super::StageParams::Compute { ref mut local_size } = ep.stage_params {
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

#[test]
fn test_parse() {
    fn parse_spirv(binary: &[u32]) {
        let mut consumer = ConsumerImpl::new();
        rspirv::binary::parse_words(binary, &mut consumer)
            .expect("Failed to parse SPIR-V");
        consumer.finalize_layout();
    }

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

    // let layout = parse_spirv(&spirv_binary);
    // println!("Parsed {:#?} entry points", layout);

    parse_spirv(&spirv_binary);
}
