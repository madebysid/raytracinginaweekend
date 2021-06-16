use std::{
    fmt::{Display, Formatter},
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    pub fn unit() -> Self {
        Self(1.0, 1.0, 1.0)
    }
}

impl Vec3 {
    pub fn r(&self) -> f32 {
        self.0
    }
    pub fn g(&self) -> f32 {
        self.1
    }
    pub fn b(&self) -> f32 {
        self.2
    }

    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }

    /// Magnitude
    pub fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    /// Returns another vector in the same direction, but with 0 < magnitude < 1
    pub fn as_unit(&self) -> Vec3 {
        let length = self.length();
        Vec3::new(self.0 / length, self.1 / length, self.2 / length)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)?;

        Ok(())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

/// f32 * Vec3
impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
/// Vec3 * f32
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

/// usize * Vec3
impl Mul<usize> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Vec3(
            self.0 * rhs as f32,
            self.1 * rhs as f32,
            self.2 * rhs as f32,
        )
    }
}
/// Vec3 * usize
impl Mul<Vec3> for usize {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}
