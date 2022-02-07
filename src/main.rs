use hit::Hittable;
use point3::Point3;
use ray::Ray;
use sphere::Sphere;
use std::error::Error;
use std::io;
use std::io::Write;
use vec3::Vec3;

mod color;
mod hit;
mod point3;
mod ray;
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
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal: Vec3 = (viewport_width, 0., 0.).into();
    let vertical: Vec3 = (0., viewport_height, 0.).into();
    let horizontal_split = &horizontal / 2.;
    let vertical_split = &vertical / 2.;
    let focal_vec3: Vec3 = (0., 0., focal_length).into();
    let lower_left_corner = origin.as_vec3() - &horizontal_split - &vertical_split - &focal_vec3;

    writeln!(&stdout, "P3")?;
    writeln!(&stdout, "{} {}", image_width, image_height)?;
    writeln!(&stdout, "255")?;

    for j in (0..image_height).rev() {
        write!(&stderr, "\rScanlines remaining: {}", j)?;
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let destination =
                &lower_left_corner + &(&horizontal * u) + &vertical * v - origin.as_vec3();
            let ray = Ray::new(&origin, destination);
            let pixel_color = world.ray_color(&ray);
            // dbg!(&pixel_color);
            pixel_color.write(&stdout)?;
        }
    }

    writeln!(&stderr, "\nDone.")?;

    Ok(())
}
