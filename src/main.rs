mod types;

use types::{Color, Config, Point, Ray, Vec3};

fn main() {
  let config= Config::new(16.0 / 9.0, 400);
  let (width, height) = config.img_size;

  print!("P3\n{} {}\n255\n", width, height);
  for j in 0..height {
    eprint!("\rLines remaining: {}", height - j);
    for i in 0..width {
      let px_center = *config.px_00 + config.px_d_u*(i as f64) + config.px_d_v*(j as f64);
      let ray_dir = px_center - config.camera_center;
      let ray = Ray::new(config.camera_center, ray_dir);

      let mut color: Color;

      let a = 0.5 * (ray.direction[1]/(config.viewport_size.1 as f64) + 1.0);
      let white = Color::new(1.0, 1.0, 1.0).scale(1.0 - a);
      let blue = Color::new(0.5, 0.7, 1.0).scale(a);
      color = (white + blue).into();

      let hit = hit_sphere(Point::new(0, 0, -1), 0.5, ray);
      if let Some(t) = hit {
        let normal = (*ray.at(t) - Vec3::new(0, 0, -1)).unit();
        let rgb = (normal + Vec3::diagonal(1)) / Vec3::diagonal(2);
        color = Color::new(rgb.0, rgb.1, rgb.2);
      }

      print!("{}\n", color);
    }
  }
  eprint!("\rDone.                                  \n");
}

fn hit_sphere(sphere_center: Point, radius: f64, ray: Ray) -> Option<f64> {
  let cq = *sphere_center - ray.origin;
  let a = ray.direction.dot(ray.direction);
  let b = ray.direction.scale(-2).dot(cq);
  let c = cq.dot(cq) - radius*radius;

  let discr = b*b - 4.0*a*c;
  if discr < 0.0 {
    None
  } else {
    let t = (-b - discr.sqrt()) / (2.0 * a);
    Some(t) 
  }
}
