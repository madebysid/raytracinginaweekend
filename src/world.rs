use std::f32::INFINITY;

use crate::{ray::Ray, surface::HitSurface, vec3::Vec3};

pub struct World<T>
where
    T: HitSurface,
{
    entities: Vec<T>,
}

impl<T> World<T>
where
    T: HitSurface,
{
    pub fn new(entities: Vec<T>) -> Self {
        Self { entities }
    }
}

impl<T> HitSurface for World<T>
where
    T: HitSurface,
{
    fn hit(&self, ray: crate::ray::Ray, t_min: f32, t_max: f32) -> Option<crate::surface::Hit> {
        // Try to hit all entities, but return the closest hit (to the camera) only

        let mut closest_hit = None;
        let mut closest_t_so_far = t_max;
        for entity in &self.entities {
            if let Some(hit) = entity.hit(ray, t_min, closest_t_so_far) {
                closest_t_so_far = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}

impl<T> World<T>
where
    T: HitSurface,
{
    pub fn texel_color(&self, ray: Ray) -> Vec3 {
        if let Some(hit) = self.hit(ray, 0.0, INFINITY) {
            // Color normal depending on direction. First move the normal to be between 0 & 1, then use those values as r,g,b
            let normal = Vec3::new(
                0.5 * (hit.normal.x() + 1.0),
                0.5 * (hit.normal.y() + 1.0),
                0.5 * (hit.normal.z() + 1.0),
            );
            return normal;
        }

        // If there are no hits, then render the sky (background), which will be a linear gradient from blue to white
        // Lerp between blue and white to create a vertical gradient
        // lerp_value = (1-t) * start_value + t * end_value (0 < t < 1)
        // Our `t` will be derived from the Y coordinate of the ray's direction

        let t = ray.direction();
        let t = t.as_unit(); // Ensures all components are between -1 & 1
        let t = 0.5 * (t.y() + 1.0); // Move to be between 0 & 1

        // 0 < r,g,b < 1
        let white = Vec3::new(1.0, 1.0, 1.0);
        let blue = Vec3::new(0.5, 0.7, 1.0);

        (1.0 - t) * white + t * blue
    }
}
