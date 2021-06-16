use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    /// Ray parameter at which this hit occurs
    pub t: f32,
    /// Point at which this hit occurs
    pub point: Vec3,
    /// Normal at point at which this hit occurs
    pub normal: Vec3,
}

impl Hit {
    pub fn new(t: f32, hit_point: Vec3, normal: Vec3) -> Self {
        Self {
            t,
            point: hit_point,
            normal,
        }
    }
}

pub trait HitSurface {
    /// Returns the closest hit (to the origin)
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
