#[repr(C)]
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    bytemuck::Zeroable,
    bytemuck::Pod,
)]
pub struct Mat3([[f32; 3]; 3]);

impl Mat3 {
    pub const ZERO: Self = Self([
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
    ]);

    pub const IDENTITY: Self = Self([
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ]);

    #[inline]
    pub fn scale_f32(factor: f32) -> Self {
        Self([
            [factor, 0.0, 0.0],
            [0.0, factor, 0.0],
            [0.0, 0.0, 1.0],
        ])
    }

    #[inline]
    pub fn scale(vector: crate::Vec2) -> Self {
        Self([
            [vector.x, 0.0, 0.0],
            [0.0, vector.y, 0.0],
            [0.0, 0.0, 1.0],
        ])
    }

    #[inline]
    pub fn scale_x(factor: f32) -> Self {
        Self([
            [factor, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ])
    }

    #[inline]
    pub fn scale_y(factor: f32) -> Self {
        Self([
            [1.0, 0.0, 0.0],
            [0.0, factor, 0.0],
            [0.0, 0.0, 1.0],
        ])
    }

    #[inline]
    pub fn rotation_x(radians: f32) -> Self {
        let sin = radians.sin();
        let cos = radians.cos();

        Self([
            [1.0, 0.0, 0.0],
            [0.0, cos, -sin],
            [0.0, sin, cos],
        ])
    }

    #[inline]
    pub fn rotation_y(radians: f32) -> Self {
        let sin = radians.sin();
        let cos = radians.cos();

        Self([
            [cos, 0.0, sin],
            [0.0, 1.0, 0.0],
            [-sin, 0.0, cos],
        ])
    }

    #[inline]
    pub fn rotation_z(radians: f32) -> Self {
        let sin = radians.sin();
        let cos = radians.cos();

        Self([
            [cos, -sin, 0.0],
            [sin, cos, 0.0],
            [0.0, 0.0, 1.0],
        ])
    }

    #[inline]
    pub fn skew(vector: crate::Vec2) -> Self {
        let tan_x = vector.x.tan();
        let tan_y = vector.y.tan();
        Self([
            [1.0, tan_x, 0.0],
            [tan_y, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ])
    }

    #[inline]
    pub fn skew_xy(x_radians: f32, y_radians: f32) -> Self {
        let tan_x = x_radians.tan();
        let tan_y = y_radians.tan();
        Self([
            [1.0, tan_x, 0.0],
            [tan_y, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ])
    }

    #[inline]
    pub fn skew_x(radians: f32) -> Self {
        let tan_x = radians.tan();
        Self([
            [1.0, tan_x, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ])
    }

    #[inline]
    pub fn skew_y(radians: f32) -> Self {
        let tan_y = radians.tan();
        Self([
            [1.0, 0.0, 0.0],
            [tan_y, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ])
    }

    #[inline]
    pub fn translation(translation: crate::Vec2) -> Self {
        Self([
            [1.0, 0.0, translation.x],
            [0.0, 1.0, translation.y],
            [0.0, 0.0, 1.0],
        ])
    }

    #[inline]
    pub fn multiply(self, other: Self) -> Self {
        let mut result = Self::ZERO;
        for i in 0..3 {
            for j in 0..3 {
                result.0[i][j] = self.0[i][0] * other.0[0][j]
                    + self.0[i][1] * other.0[1][j]
                    + self.0[i][2] * other.0[2][j];
            }
        }
        result
    }

    #[inline]
    pub fn determinant(self) -> f32 {
        let m = self.0;
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }

    #[inline]
    pub fn transpose(self) -> Self {
        let m = self.0;
        Self([
            [m[0][0], m[1][0], m[2][0]],
            [m[0][1], m[1][1], m[2][1]],
            [m[0][2], m[1][2], m[2][2]],
        ])
    }

    #[inline]
    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < 1e-6 {
            return None;
        }
        let m = self.0;
        let inv_det = 1.0 / det;
        let inv = [
            [
                (m[1][1] * m[2][2] - m[1][2] * m[2][1])
                    * inv_det,
                (m[0][2] * m[2][1] - m[0][1] * m[2][2])
                    * inv_det,
                (m[0][1] * m[1][2] - m[0][2] * m[1][1])
                    * inv_det,
            ],
            [
                (m[1][2] * m[2][0] - m[1][0] * m[2][2])
                    * inv_det,
                (m[0][0] * m[2][2] - m[0][2] * m[2][0])
                    * inv_det,
                (m[0][2] * m[1][0] - m[0][0] * m[1][2])
                    * inv_det,
            ],
            [
                (m[1][0] * m[2][1] - m[1][1] * m[2][0])
                    * inv_det,
                (m[0][1] * m[2][0] - m[0][0] * m[2][1])
                    * inv_det,
                (m[0][0] * m[1][1] - m[0][1] * m[1][0])
                    * inv_det,
            ],
        ];
        Some(Self(inv))
    }

    #[inline]
    pub fn transform(self, point: crate::Vec2) -> crate::Vec2 {
        let x = self.0[0][0] * point.x
            + self.0[0][1] * point.y
            + self.0[0][2];
        let y = self.0[1][0] * point.x
            + self.0[1][1] * point.y
            + self.0[1][2];
        crate::Vec2 { x, y }
    }
}

impl Default for Mat3 {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl std::ops::Mul for Mat3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl From<Mat3> for [[f32; 4]; 3] {
    #[inline]
    fn from(val: Mat3) -> Self {
        let Mat3(m) = val;

        [
            [m[0][0], m[0][1], m[0][2], 0.0],
            [m[1][0], m[1][1], m[1][2], 0.0],
            [m[2][0], m[2][1], m[2][2], 1.0],
        ]
    }
}
