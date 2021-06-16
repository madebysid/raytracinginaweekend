use std::f32::INFINITY;

use crate::{ray::Ray, sphere::Sphere, surface::HitSurface, vec3::Vec3};

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
        // For a pixel that a ray from the camera hits, its color is the color of the entity the ray hits NEXT (after reflection). For different materials, the reflection strategy is different.
        // This can continue recursively; the color of the entity the reflected ray hits is the color of the entity the reflected reflected ray hits.
        // This continues until the ray does not hit anything.

        // For matte materials, we assume the ray is reflected in a random direction. The contribution of the reflected ray is also significantly reduced.
        // We'll treat all entities in this world as matte materials.

        if let Some(hit) = self.hit(ray, 0.001, INFINITY) {
            // For randomness, we'll say that the ray gets reflected to a random point (S) of a unit sphere that is tangeant to the hitpoint (P).
            // Our entity's surface normal (N) points to this unit sphere's center. Also our normals are unit length, so they point exactly at this center
            // The center of such a sphere then is P + N. (look at last diagram on page 22 of book)
            // The random point in this unit sphere (from ITS center) is S. So the target pixel is P + N + S.
            // The ray to this target pixel starts at P, and has the direction (P + N + S) - P

            let target_pixel = hit.point + hit.normal + Sphere::random_point_in_unit_sphere();
            return 0.5 * self.texel_color(Ray::new(hit.point, target_pixel - hit.point));
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
