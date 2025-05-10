mod types;
mod hit;
mod sphere;

use hit::Hittable;
use sphere::Sphere;
use types::{Color, Config, Point, Ray, ToVec3, Vec3};

fn main() {
  let config= Config::new(16.0 / 9.0, 400);
  let (width, height) = config.img_size;

  let sphere = Sphere::new(Point::new(0, 0, -1), 0.5);

  print!("P3\n{} {}\n255\n", width, height);
  for j in 0..height {
    eprint!("\rLines remaining: {}", height - j);
    for i in 0..width {
      let px_center = config.px_00.to_vec3() + config.px_d_u*(i as f64) + config.px_d_v*(j as f64);
      let ray_dir = px_center - config.camera_center;
      let ray = Ray::new(config.camera_center, ray_dir);

      let mut color: Color;

      let a = 0.5 * (ray.direction[1]/(config.viewport_size.1 as f64) + 1.0);
      let white = Color::new(1.0, 1.0, 1.0).to_vec3().scale(1.0 - a);
      let blue = Color::new(0.5, 0.7, 1.0).to_vec3().scale(a);
      color = (white + blue).into();

      if let Some(hit) = sphere.hit(ray, -50..=50) {
        let rgb = (hit.normal + Vec3::diagonal(1)) / Vec3::diagonal(2);
        color = Color::new(rgb.0, rgb.1, rgb.2);
      }

      print!("{}\n", color);
    }
  }
  eprint!("\rDone.                                  \n");
}
