use std::ops::{Add, Div, Mul, Sub};

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

pub struct Uv {
    pub u: f32,
    pub v: f32,
}

impl Uv {
    #[inline(always)]
    pub const fn new(u: f32, v: f32) -> Self {
        Self { u, v }
    }
}

impl Add for Uv {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Uv::new(self.u + other.u, self.v + other.v)
    }
}

impl Sub for Uv {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Uv::new(self.u - other.u, self.v - other.v)
    }
}

impl Mul<f32> for Uv {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Uv::new(self.u * scalar, self.v * scalar)
    }
}

impl Div<f32> for Uv {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Uv::new(self.u / scalar, self.v / scalar)
    }
}

impl Uv {
    pub fn dot(self, other: Self) -> f32 {
        self.u * other.u + self.v * other.v
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        Uv::new(
            self.u + (other.u - self.u) * t,
            self.v + (other.v - self.v) * t,
        )
    }

    pub fn max_f32(self, other: f32) -> Self {
        Self {
            u: self.u.max(other),
            v: self.v.max(other),
        }
    }

    #[inline]
    pub fn clamp_f32(self, min: f32, max: f32) -> Self {
        Uv::new(self.u.clamp(min, max), self.v.clamp(min, max))
    }

    #[inline]
    pub fn is_finite(self) -> bool {
        self.u.is_finite() && self.v.is_finite()
    }

    #[inline]
    pub fn is_nan(self) -> bool {
        self.u.is_nan() || self.v.is_nan()
    }

    #[inline]
    pub fn recip(self) -> Self {
        Self {
            u: self.u.recip(),
            v: self.v.recip(),
        }
    }

    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        ((self.u - other.u).powi(2)
            + (self.v - other.v).powi(2))
        .sqrt()
    }
}

impl std::fmt::Display for Uv {
    #[inline]
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_tuple("Uv")
            .field(&self.u)
            .field(&self.v)
            .finish()
    }
}

impl From<super::Vec2> for Uv {
    #[inline(always)]
    fn from(super::Vec2 { x: u, y: v }: super::Vec2) -> Self {
        Self { u, v }
    }
}

impl From<(f32, f32)> for Uv {
    #[inline(always)]
    fn from((u, v): (f32, f32)) -> Self {
        Self { u, v }
    }
}
