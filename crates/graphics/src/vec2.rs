use std::{fmt, ops};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, bytemuck::Zeroable,
)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2::splat(0.0);

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn splat(value: f32) -> Self {
        Self { x: value, y: value }
    }

    #[inline]
    pub fn dot(self, other: Vec2) -> f32 {
        let sq_x = self.x * other.x;
        let sq_y = self.y * other.y;

        sq_x + sq_y
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.x.hypot(self.y)
    }

    #[inline]
    pub fn length_sq(self) -> f32 {
        let sq_x = self.x * self.x;
        let sq_y = self.y * self.y;

        sq_x + sq_y
    }

    #[inline]
    pub fn atan2(self) -> f32 {
        self.y.atan2(self.x)
    }

    #[inline]
    pub fn lerp(self, other: Self, scalar: f32) -> Self {
        Self {
            x: self.x + scalar * (other.x - self.x),
            y: self.y + scalar * (other.y - self.y),
        }
    }

    #[inline]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    #[inline]
    pub fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }
}

impl ops::Add for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Vec2")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

unsafe impl bytemuck::Pod for Vec2 {}
