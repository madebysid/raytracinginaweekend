use std::{
    fmt::{Display, Formatter},
    fs, u32,
};

struct P3 {
    width: usize,
    height: usize,
    pix: Vec<u32>,
}

impl P3 {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pix: Vec::with_capacity(width * height),
        }
    }
}

impl Display for P3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;

        for i in 0..self.height {
            for j in 0..self.width {
                write!(f, "{} {} 128", i, j)?;
                write!(f, " ")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let image = P3::new(255, 255);

    fs::write("target/out.ppm", format!("{}", image)).unwrap();
}
