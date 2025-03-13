#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub(crate) enum VertexShaderId {
    QuadEmitUv,
}

pub(crate) const QUAD_EMIT_UV: super::ShaderSource<
    VertexShaderId,
> = crate::include_spirv!(
    VertexShaderId::QuadEmitUv,
    "vs_quad_emit_uv.spv"
);
