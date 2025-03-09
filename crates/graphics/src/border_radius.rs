#[repr(C)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    bytemuck::Zeroable,
    bytemuck::Pod,
)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

impl BorderRadius {
    #[inline]
    pub const fn all(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_left: radius,
            bottom_right: radius,
        }
    }

    #[inline]
    pub const fn top(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_left: 0.,
            bottom_right: 0.,
        }
    }

    #[inline]
    pub const fn bottom(radius: f32) -> Self {
        Self {
            top_left: 0.,
            top_right: 0.,
            bottom_left: radius,
            bottom_right: radius,
        }
    }

    #[inline]
    pub const fn left(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: 0.,
            bottom_left: radius,
            bottom_right: 0.,
        }
    }

    #[inline]
    pub const fn right(radius: f32) -> Self {
        Self {
            top_left: 0.,
            top_right: radius,
            bottom_left: 0.,
            bottom_right: radius,
        }
    }

    #[inline]
    pub const fn clamp(self, min: f32, max: f32) -> Self {
        Self {
            top_left: self.top_left.clamp(min, max),
            top_right: self.top_left.clamp(min, max),
            bottom_left: self.top_left.clamp(min, max),
            bottom_right: self.top_left.clamp(min, max),
        }
    }
}
