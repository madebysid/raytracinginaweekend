use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Vec3,
    x_space: Vec3,
    y_space: Vec3,
    translation: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, viewport_width: usize, viewport_height: usize) -> Self {
        let aspect_ratio = viewport_width as f32 / viewport_height as f32;

        // We want the origin to be at the center of the screen. In the image, it is on the bottom-left.
        // We also want to deal with smaller numbers, so we'll use the aspect ratio `a`.
        // This is the coordinate systme we want:
        //
        // (-a,1,-1)------------(a,1,-1)
        //  |                      |
        //  |        (0,0,0)       |
        //  |                      |
        // (-a,-1,-1)-----------(a,-1,-1)

        let x_space = Vec3::new(2.0 * aspect_ratio, 0.0, 0.0);
        let y_space = Vec3::new(0.0, 2.0, 0.0);

        // Translation moves all points on the screen to the coordinate system we want (screen space)
        let translation = Vec3::new(-1.0 * aspect_ratio, -1.0, -1.0);

        Self {
            origin: position,
            x_space,
            y_space,
            translation,
        }
    }
}

impl Camera {
    pub fn cast_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.translation + u * self.x_space + v * self.y_space,
        )
    }
}
