use std::{fmt, ops};

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
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    const MAX_DIM: f32 = 32768.; // 2^15
    pub const MAX: Size = Self::square(Self::MAX_DIM);

    #[inline(always)]
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    #[inline(always)]
    pub const fn square(size: f32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    #[inline(always)]
    pub const fn area(&self) -> f32 {
        self.width * self.height
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.width.is_finite() && self.height.is_finite()
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        self.width.is_nan() || self.height.is_nan()
    }

    #[inline]
    pub const fn validate(self) -> logging::Result<Self> {
        if self.width > Self::MAX_DIM
            || self.height > Self::MAX_DIM
        {
            return logging::ErrorKind::SizeExceedMaxSize
                .into_result();
        }

        if self.width < 0. || self.height < 0. {
            return logging::ErrorKind::NegativeSize
                .into_result();
        }

        Ok(self)
    }
}

impl ops::Add for Size {
    type Output = Size;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl ops::Sub for Size {
    type Output = Size;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl ops::Mul<f32> for Size {
    type Output = Size;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl ops::Div<f32> for Size {
    type Output = Size;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}

impl fmt::Display for Size {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Size")
            .field(&self.width)
            .field(&self.height)
            .finish()
    }
}

impl From<(f32, f32)> for Size {
    #[inline(always)]
    fn from((width, height): (f32, f32)) -> Self {
        Size { width, height }
    }
}

impl From<super::Vec2> for Size {
    #[inline(always)]
    fn from(
        super::Vec2 {
            x: width,
            y: height,
        }: super::Vec2,
    ) -> Self {
        Size { width, height }
    }
}
