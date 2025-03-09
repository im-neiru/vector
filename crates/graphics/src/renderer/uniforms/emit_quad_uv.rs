#[repr(C)]
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    bytemuck::Zeroable,
    bytemuck::Pod,
)]
pub(crate) struct EmitQuadUv {
    pub(crate) transform: [[f32; 4]; 3],
    pub(crate) position: crate::Vec2,
    pub(crate) z: f32,
    pub(crate) struct_pad: f32,
}

impl super::UniformTrait for EmitQuadUv {
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
