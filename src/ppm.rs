use std::fs::File;
use std::io::Write;

pub struct Ppm {
    width: i32,
    height: i32,
    data: Vec<(u8, u8, u8)>,
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

    pub fn set(&mut self, x: i32, y: i32, color: (u8, u8, u8)) {
        self.data[(x + y * self.width) as usize] = color;
    }

    pub fn get(&self, x: i32, y: i32) -> (u8, u8, u8) {
        self.data[(x + y * self.width) as usize]
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}

pub fn save(ppm: &Ppm, filename: String) -> std::io::Result<()> {
    let mut f = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(filename)?;
    writeln!(&mut f, "P3")?;
    writeln!(&mut f, "{} {}", ppm.width, ppm.height)?;
    writeln!(&mut f, "255")?;
    for (r, g, b) in ppm.data.iter() {
        writeln!(&mut f, "{} {} {}", r, g, b)?;
    }
    f.flush()?;
    Ok(())
}
