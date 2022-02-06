use std::error::Error;
use std::io;
use std::io::Write;
use vec3::Vec3;

struct Point(Vec3);
struct Color(Vec3);

mod vec3;

impl Color {
    fn write(&self, mut writer: impl Write) -> Result<(), Box<dyn Error>> {
        let ir = (255.999 * self.0.x()) as i8;
        let ig = (255.999 * self.0.y()) as i8;
        let ib = (255.999 * self.0.z()) as i8;

        writeln!(writer, "{} {} {}", ir, ig, ib)?;
        Ok(())
    }

    fn new(r: f64, g: f64, b: f64) -> Self {
        Color((r, g, b).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let image_width = 256;
    let image_height = 256;

    let stdout = io::stdout();
    let stderr = io::stderr();

    writeln!(&stdout, "P3")?;
    writeln!(&stdout, "{} {}", image_width, image_height)?;
    writeln!(&stdout, "255")?;

    for j in (0..image_height).rev() {
        write!(&stderr, "\rScanlines remaining: {}", j)?;
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;
            let pixel_color = Color::new(r, g, b);
            pixel_color.write(&stdout)?;
        }
    }

    writeln!(&stderr, "\nDone.")?;

    Ok(())
}
