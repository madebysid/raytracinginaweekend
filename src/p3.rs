use std::fmt::{Display, Formatter};

use crate::vec3::Vec3;

pub struct P3 {
    width: usize,
    height: usize,
}

impl P3 {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl Display for P3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;

        for i in 0..self.height {
            for j in 0..self.width {
                write!(f, "{}", Vec3::new(j, i, 128))?;
                write!(f, " ")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
