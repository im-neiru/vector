use shader_include::include_spirv;

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub(crate) enum FragmentShaderId {
    QuadEmitUv,
}

pub(crate) const ROUNDED_RECTANGLE_COLOR_FILL:
    super::ShaderSource<FragmentShaderId> = include_spirv!(
    FragmentShaderId::QuadEmitUv,
    "fs_rounded_rectangle_color_fill.spv"
);
