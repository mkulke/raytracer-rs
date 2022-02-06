use color::Color;
use point3::Point3;
use ray::Ray;
use std::error::Error;
use std::io;
use std::io::Write;
use vec3::Vec3;

mod color;
mod point3;
mod ray;
mod vec3;

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    let color_a = Color::new(1.0, 1.0, 1.0) * (1.0 - t);
    // dbg!(&color_a);
    let color_b = Color::new(0.5, 0.7, 1.0) * t;
    // dbg!(&color_b);
    color_a + color_b
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let stderr = io::stderr();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u16;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal: Vec3 = (viewport_width, 0., 0.).into();
    let vertical: Vec3 = (0., viewport_height, 0.).into();
    let horizontal_split = &horizontal / 2.;
    let vertical_split = &vertical / 2.;
    let focal_vec3: Vec3 = (0., 0., focal_length).into();
    let lower_left_corner = &origin.0 - &horizontal_split - &vertical_split - &focal_vec3;

    writeln!(&stdout, "P3")?;
    writeln!(&stdout, "{} {}", image_width, image_height)?;
    writeln!(&stdout, "255")?;

    for j in (0..image_height).rev() {
        write!(&stderr, "\rScanlines remaining: {}", j)?;
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let destination = &lower_left_corner + &(&horizontal * u) + &vertical * v - &origin.0;
            let ray = Ray::new(&origin, destination);
            let pixel_color = ray_color(&ray);
            // dbg!(&pixel_color);
            pixel_color.write(&stdout)?;
        }
    }

    writeln!(&stderr, "\nDone.")?;

    Ok(())
}
