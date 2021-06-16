use std::{
    fmt::{Display, Formatter},
    mem::size_of,
};

use crate::vec3::Vec3;

pub struct P3 {
    width: usize,
    height: usize,
    texels: Vec<Vec3>, // row-major texels
}

impl P3 {
    pub fn new(width: usize, height: usize) -> Self {
        let mut texels = Vec::with_capacity(width * height * size_of::<Vec3>());
        texels.resize(texels.capacity(), Vec3::unit());

        Self {
            width,
            height,
            texels,
        }
    }
}

impl P3 {
    pub fn color_at(&self, x: usize, y: usize) -> Vec3 {
        self.texels[x * self.width + y]
    }

    pub fn set_color_at(&mut self, x: usize, y: usize, color: Vec3) {
        self.texels[x * self.width + y] = color;
    }
}

impl Display for P3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;

        // y goes .rev() because we consider 0,0 to be on the bottom left
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let color = self.color_at(x, y);
                write!(
                    f,
                    "{} {} {} ",
                    (color.r() * 255.0).ceil(),
                    (color.g() * 255.0).ceil(),
                    (color.b() * 255.0).ceil()
                )?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
