use crate::vec3::Vec3;

/// A ray is a function of 2 vectors (A, B) where A = origin of ray, B = Vector in direction of ray
/// p(t) = A + B * t
#[derive(Debug, Clone, Copy)]
pub struct Ray(Vec3, Vec3);

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self(a, b)
    }
}

impl Ray {
    pub fn origin(&self) -> Vec3 {
        self.0
    }

    pub fn direction(&self) -> Vec3 {
        self.1
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin() + t * self.direction()
    }
}
