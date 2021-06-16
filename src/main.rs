mod p3;
mod ray;
mod vec3;

use p3::P3;
use std::fs;

use crate::{ray::Ray, vec3::Vec3};

fn texel_color(ray: Ray) -> Vec3 {
    // Lerp between blue and white to create a vertical gradient
    // lerp_value = (1-t) * start_value + t * end_value (0 < t < 1)
    // Our `t` will be derived from the Y coordinate of the ray's direction

    let t = ray.direction();
    let t = t.as_unit(); // Ensures all components are between -1 & 1
    let t = 0.5 * (t.y() + 1.0); // Move to be between 0 & 1

    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);

    (1.0 - t) * white + t * blue
}

fn main() {
    let width = 400;
    let height = 200;
    let mut image = P3::new(width, height);

    // Hello World!
    // for row in 0..height {
    //     for col in 0..width {
    //         let u = row as f32 / width as f32;
    //         let v = col as f32 / height as f32;
    //         image.set_color_at(row, col, Vec3::new(u, v, 0.5))
    //     }
    // }

    let origin = Vec3::new(0.0, 0.0, 0.0);
    // We want the origin to be at the center of the screen. In the image, it is on the bottom-left.
    // So move all points by this amount
    let translation = Vec3::new(-2.0, -1.0, -1.0);

    // let x_unit = Vec3::new(1.0, 0.0, 0.0);
    // let y_unit = Vec3::new(0.0, 1.0, 0.0);
    let x_unit = Vec3::new(4.0, 0.0, 0.0);
    let y_unit = Vec3::new(0.0, 2.0, 0.0);

    // Camera is assumed to be at origin
    // Assume the "screen" is at Z = -1

    for x in 0..width {
        for y in 0..height {
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;
            let ray = Ray::new(origin, translation + u * x_unit + v * y_unit);
            image.set_color_at(x, y, texel_color(ray))
        }
    }

    fs::write("target/out.ppm", format!("{}", image)).unwrap();
}
