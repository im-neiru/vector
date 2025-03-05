#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct TransformUniform {
    pub scale: crate::Vec2,
    pub translate: crate::Vec2,
}

impl TransformUniform {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        use std::ops::Mul;

        let span = {
            let w = width as f32;
            let h = height as f32;

            crate::Vec2::new(w, h).mul(0.5)
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
