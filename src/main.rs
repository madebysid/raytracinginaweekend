mod p3;
mod ray;
mod sphere;
mod surface;
mod vec3;
mod world;

use p3::P3;
use sphere::Sphere;
use std::fs;

use crate::{ray::Ray, vec3::Vec3, world::World};

fn main() {
    let width = 480;
    let height = 270;
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

    // Camera is assumed to be at origin (even in struct implementations). The screen is assumed to be at z = -1

    // Translation moves all points on the screen to the coordinate system we want (screen space)
    let translation = Vec3::new(-1.0 * aspect_ratio, -1.0, -1.0);

    // Create the world
    let entities = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5), // Sphere at origin
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0), // Large sphere (floor)
    ];
    let world = World::new(entities);

    for x in 0..width {
        for y in 0..height {
            // Texture coordinates. 0 < u,v < 1
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;

            let ray = Ray::new(origin, translation + u * x_space + v * y_space);
            image.set_color_at(x, y, world.texel_color(ray))
        }
    }

    fs::write("target/out.ppm", format!("{}", image)).unwrap();
}
