mod types;

use types::{Color, Config, Ray};

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

      let a = 0.5 * (ray.direction[1]/(height as f64) + 1.0);
      let white = Color::new(1.0, 1.0, 1.0).scale(1.0 - a);
      let blue = Color::new(0.5, 0.7, 1.0).scale(a);
      let color: Color = (white + blue).into();
      print!("{}\n", color);
    }
  }
  eprint!("\rDone.                                  \n");
}
