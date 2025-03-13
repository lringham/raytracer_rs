use std::fs::File;
use std::io::Write;

use crate::vec3f::Vec3f;

pub struct Ppm {
    width: usize,
    height: usize,
    data: Vec<(u8, u8, u8)>,
}

pub fn color_to_tuple(color: &Vec3f) -> (u8, u8, u8) {
    (
        (255.0 * color.x.min(1.0).sqrt()) as u8,
        (255.0 * color.y.min(1.0).sqrt()) as u8,
        (255.0 * color.z.min(1.0).sqrt()) as u8,
    )
}

#[allow(dead_code)]
impl Ppm {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![(0, 0, 0); width * height];
        Self {
            width,
            height,
            data,
        }
    }

    pub fn from(width: usize, height: usize, colors: &Vec<Vec3f>) -> Self {
        let mut data = vec![(0, 0, 0); width * height];
        for (i, color) in colors.iter().rev().enumerate() {
            data[i] = color_to_tuple(&color);
        };
        Self {
            width,
            height,
            data,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Vec3f) {
        self.data[x + y * self.width] = color_to_tuple(&color);
    }

    pub fn get(&self, x: usize, y: usize) -> Vec3f {
        let (r, g, b) = self.data[x + y * self.width];
        Vec3f {
            x: (r as f32 / 255.0).powf(2.0),
            y: (g as f32 / 255.0).powf(2.0),
            z: (b as f32 / 255.0).powf(2.0),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        let mut f = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filename)?;
        writeln!(&mut f, "P3")?;
        writeln!(&mut f, "{} {}", self.width, self.height)?;
        writeln!(&mut f, "255")?;
        for (r, g, b) in self.data.iter() {
            writeln!(&mut f, "{} {} {}", r, g, b)?;
        }
        f.flush()?;
        Ok(())
    }
}
