#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct TransformUniform {
    pub scale: crate::Vec2,
    pub translate: crate::Vec2,
}

impl TransformUniform {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        let w = width as f32;
        let h = height as f32;

        Self {
            scale: crate::Vec2 {
                x: w.recip(),
                y: h.recip(),
            },
            translate: crate::Vec2 { x: -w, y: h },
        }
    }
}
