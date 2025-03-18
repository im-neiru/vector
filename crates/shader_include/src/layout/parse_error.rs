#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Expect result id for {0:?}")]
    NoResultId(rspirv::spirv::Op),
    #[error("Expect operands for {0:?}")]
    ExpectOperand(rspirv::spirv::Op),
    #[error("Invalid float width. {0} bit/s is not supported")]
    InvalidFloatWidth(u32),
    #[error(
        "Invalid vector scalar count. {0} element/s is not supported"
    )]
    InvalidVectorSize(u32),
    #[error(
        "Invalid matrix column count. {0} column/s is not supported"
    )]
    InvalidMatrixSize(u32),
}
