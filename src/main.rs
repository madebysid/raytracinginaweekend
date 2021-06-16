mod p3;
mod ray;
mod vec3;

use p3::P3;
use std::fs;
use vec3::dot;

use crate::{ray::Ray, vec3::Vec3};

fn hits_sphere(center: Vec3, radius: f32, ray: Ray) -> bool {
    // For ray p(t) = A + t*B
    // For sphere centered at C and with radius R, a point p on sphere satisfies:
    // dot(p-C, p-C) = R*R
    // So, common points of sphere and ray are: dot(p(t) - C, p(t) - C) = R * R
    // Reduce it and get: t*t*dot(B, B) + 2*t*dot(A-C, B) + dot(A-C, A-C) = R * R
    // If we solve this for `t` (the only unknown), this is a quadratic equation
    // If this has 1 or 2 roots, the ray hits the sphere; aka quadratic discriminant b*b - 4*a*c > 0

    // In quadratic equation a*a +2*a*b + c = 0, replace a, b, c with all calculation above
    let a = dot(ray.direction(), ray.direction());
    let b = 2.0 * dot(ray.origin() - center, ray.direction());
    let c = dot(ray.origin() - center, ray.origin() - center) - radius * radius;

    b * b - 4.0 * a * c > 0.0
}

fn texel_color(ray: Ray) -> Vec3 {
    if hits_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
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

fn main() {
    let width = 400;
    let height = 200;
    let mut image = P3::new(width, height);
    let aspect_ratio = image.aspect_ratio;

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

    // Camera is assumed to be at origin. The screen is assumed to be at z = -1

    // Translation moves all points on the screen to the coordinate system we want (screen space)
    let translation = Vec3::new(-1.0 * aspect_ratio, -1.0, -1.0);

    for x in 0..width {
        for y in 0..height {
            // Texture coordinates. 0 < u,v < 1
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;

            let ray = Ray::new(origin, translation + u * x_space + v * y_space);
            image.set_color_at(x, y, texel_color(ray))
        }
    }

    fs::write("target/out.ppm", format!("{}", image)).unwrap();
}
