use std::fs::File;
use std::io::Write;

use crate::vec3f::Vec3f;

pub struct Ppm {
    width: i32,
    height: i32,
    data: Vec<(u8, u8, u8)>,
}

fn color_to_tuple(color: Vec3f) -> (u8, u8, u8) {
    (
        (255.0 * color.x.sqrt()) as u8,
        (255.0 * color.y.sqrt()) as u8,
        (255.0 * color.z.sqrt()) as u8,
    )
}

#[allow(dead_code)]
impl Ppm {
    pub fn new(width: i32, height: i32) -> Self {
        let data = vec![(0, 0, 0); (width * height) as usize];
        Self {
            width,
            height,
            data,
        }
    }

    pub fn set(&mut self, x: i32, y: i32, color: Vec3f) {
        self.data[(x + y * self.width) as usize] = color_to_tuple(color);
    }

    pub fn get(&self, x: i32, y: i32) -> Vec3f {
        let (r, g, b) = self.data[(x + y * self.width) as usize];
        Vec3f {
            x: (r as f32 / 255.0).powf(2.0),
            y: (g as f32 / 255.0).powf(2.0),
            z: (b as f32 / 255.0).powf(2.0),
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
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
