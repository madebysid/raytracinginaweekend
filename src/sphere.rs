use rand::random;

use crate::{
    surface::{Hit, HitSurface},
    vec3::{dot, Vec3},
};

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Returns a random point within a unit sphere centered at the origin
    pub fn random_point_in_unit_sphere() -> Vec3 {
        let mut point;
        loop {
            point = Vec3::new(random::<f32>(), random::<f32>(), random::<f32>()); // 0 < x,y,z < 1
            point = 2.0 * point - Vec3::new(1.0, 1.0, 1.0); // -1 < x,y,z < 1

            // Inside unit sphere -> distance from unit sphere's center < 1
            if point.squared_length() < 1.0 {
                break;
            }
        }

        point
    }
}

impl HitSurface for Sphere {
    fn hit(&self, ray: crate::ray::Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        // For ray p(t) = A + t*B
        // For sphere centered at C and with radius R, a point p on sphere satisfies:
        // dot(p-C, p-C) = R*R
        // So, common points of sphere and ray are: dot(p(t) - C, p(t) - C) = R * R
        // Reduce it and get: t*t*dot(B, B) + 2*t*dot(A-C, B) + dot(A-C, A-C) = R * R
        // If we solve this for `t` (the only unknown), this is a quadratic equation
        // If this has 1 or 2 roots, the ray hits the sphere; aka quadratic discriminant b*b - 4*a*c > 0

        // In quadratic equation a*a + 2*a*b + c = 0, replace a, b, c with all calculation above
        let a = dot(ray.direction(), ray.direction());
        let b = 2.0 * dot(ray.origin() - self.center, ray.direction());
        let c =
            dot(ray.origin() - self.center, ray.origin() - self.center) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let hit_t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t_min < hit_t && hit_t < t_max {
                let hit_point = ray.point_at(hit_t);
                // Normal at a point P on the sphere is assumed to be a unit vector pointing towards (P - C) where C is the center of the sphere
                let hit_normal = (hit_point - self.center).as_unit(); // -1 < normal < 1

                return Some(Hit::new(hit_t, hit_point, hit_normal));
            }

            let hit_t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t_min < hit_t && hit_t < t_max {
                let hit_point = ray.point_at(hit_t);
                // Normal at a point P on the sphere is assumed to be a unit vector pointing towards (P - C) where C is the center of the sphere
                let hit_normal = (hit_point - self.center).as_unit(); // -1 < normal < 1

                return Some(Hit::new(hit_t, hit_point, hit_normal));
            }

            None
        }
    }
}
