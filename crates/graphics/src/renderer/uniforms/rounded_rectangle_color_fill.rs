#[repr(C)]
#[derive(
    Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable,
)]
pub(in crate::renderer) struct RoundedRectangleColorFill {
    pub(in crate::renderer) color: crate::Color,
    pub(in crate::renderer) size: crate::Size,
    pub(in crate::renderer) border_radius: crate::BorderRadius,
    pub(in crate::renderer) padding: crate::Vec2,
}

impl super::UniformTrait for RoundedRectangleColorFill {
    #[inline]
    fn binding_type() -> wgpu::BindingType {
        wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: std::num::NonZeroU64::new(
                std::mem::size_of::<Self>() as u64,
            ),
        }
    }
}
