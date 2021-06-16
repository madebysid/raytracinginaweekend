mod camera;
mod p3;
mod ray;
mod sphere;
mod surface;
mod vec3;
mod world;

use p3::P3;
use rand::random;
use sphere::Sphere;
use std::fs;

use crate::{camera::Camera, vec3::Vec3, world::World};

fn main() {
    let width = 480;
    let height = 270;
    let aa_samples = 8;
    let mut image = P3::new(width, height);

    // Hello World!
    // for row in 0..height {
    //     for col in 0..width {
    //         let u = row as f32 / width as f32;
    //         let v = col as f32 / height as f32;
    //         image.set_color_at(row, col, Vec3::new(u, v, 0.5))
    //     }
    // }

    // Camera is assumed to be at origin (even in struct implementations). The screen is assumed to be at z = -1
    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), width, height);

    // Create the world
    let entities = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5), // Sphere at origin
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0), // Large sphere (floor)
    ];
    let world = World::new(entities);

    for x in 0..width {
        for y in 0..height {
            let mut total_color = Vec3::new(0.0, 0.0, 00.);
            // Samples per pixel
            for _ in 0..aa_samples {
                // Texture coordinates. 0 < u,v < 1
                let u = (x as f32 + random::<f32>()) / width as f32;
                let v = (y as f32 + random::<f32>()) / height as f32;

                let ray = camera.cast_ray(u, v);
                let color = world.texel_color(ray);
                total_color = total_color + color;
            }

            image.set_color_at(x, y, total_color / aa_samples as f32)
        }
    }

    fs::write("target/out.ppm", format!("{}", image)).unwrap();
}
