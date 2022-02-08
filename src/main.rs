use camera::Camera;
use color::Color;
use hit::Hittable;
use point3::Point3;
use rnd::pseudo_rnd;
use sphere::Sphere;
use std::error::Error;
use std::io;
use std::io::Write;

mod camera;
mod color;
mod hit;
mod point3;
mod ray;
mod rnd;
mod sphere;
mod vec3;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let stderr = io::stderr();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u16;

    // World
    let sphere_small = Sphere::new(Point3::new(0., 0., -1.), 0.5);
    let sphere_big = Sphere::new(Point3::new(0., -100.5, -1.), 100.0);
    let world: Vec<Box<dyn Hittable>> = vec![Box::new(sphere_small), Box::new(sphere_big)];

    // Camera
    let camera = Camera::new();

    writeln!(&stdout, "P3")?;
    writeln!(&stdout, "{} {}", image_width, image_height)?;
    writeln!(&stdout, "255")?;

    // Antialiasing
    let samples = 100;

    for j in (0..image_height).rev() {
        write!(&stderr, "\rScanlines remaining: {:0>3}", j)?;
        for i in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0.);
            for _ in 0..samples {
                let u = (i as f64 + pseudo_rnd()) / (image_width - 1) as f64;
                let v = (j as f64 + pseudo_rnd()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += world.ray_color(&ray);
                // dbg!(&pixel_color);
            }
            pixel_color.write_sampled(&stdout, samples)?;
        }
    }

    writeln!(&stderr, "\nDone.")?;

    Ok(())
}
