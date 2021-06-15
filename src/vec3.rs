use std::fmt::{Display, Formatter};

pub struct Vec3(usize, usize, usize);

impl Vec3 {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self(x, y, z)
    }
}

impl Vec3 {
    pub fn r(&self) -> usize {
        self.0
    }
    pub fn g(&self) -> usize {
        self.1
    }
    pub fn b(&self) -> usize {
        self.2
    }

    pub fn x(&self) -> usize {
        self.0
    }
    pub fn y(&self) -> usize {
        self.1
    }
    pub fn z(&self) -> usize {
        self.2
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)?;

        Ok(())
    }
}
