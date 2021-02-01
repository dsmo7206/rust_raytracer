use std::{
    fs::File,
    io::{prelude::*, BufWriter},
};

pub type Colour = glam::Vec3A;

pub struct Image {
    width: usize,
    height: usize,
    pub rows: Vec<Vec<Colour>>,
}

impl Image {
    pub fn from_rows(width: usize, height: usize, rows: Vec<Vec<Colour>>) -> Image {
        Image {
            width,
            height,
            rows,
        }
    }

    pub fn into_ppm(self, filename: &str) -> std::io::Result<()> {
        let mut file = BufWriter::new(File::create(filename)?);

        write!(file, "P3\n{} {}\n255\n", self.width, self.height)?;

        for row in self.rows.into_iter().rev() {
            for pixel in row {
                writeln!(
                    file,
                    "{} {} {}",
                    (pixel.x * 255.0) as usize,
                    (pixel.y * 255.0) as usize,
                    (pixel.z * 255.0) as usize
                )?;
            }
        }

        Ok(())
    }
}
