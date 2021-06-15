mod p3;
mod vec3;

use p3::P3;
use std::fs;

fn main() {
    let image = P3::new(255, 255);

    fs::write("target/out.ppm", format!("{}", image)).unwrap();
}
