#[repr(C)]
#[derive(
    Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable,
)]
pub(crate) struct Projection {
    pub(crate) scale: crate::Vec2,
    pub(crate) translate: crate::Vec2,
}

impl Projection {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        use std::ops::Mul;

        let span = {
            let w = width as f32;
            let h = height as f32;

            crate::Vec2::new(w, -h).mul(0.5)
        };

        Self {
            scale: span.recip(),
            translate: crate::Vec2 {
                x: -span.x,
                y: span.y,
            },
        }
    }
}

impl super::UniformTrait for Projection {
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
