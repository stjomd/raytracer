mod types;

use types::{Color, Point, Ray, Vec3};

fn main() {
  // Image
  let aspect_ratio = 16.0 / 9.0;
  let (width, height) = image_dimensions(aspect_ratio, 400);
  // Camera
  let focal_length = 1.0;
  let camera_center = Point::origin();
  let (vp_width, vp_height) = viewport_dimensions(width, height);
  // Viewport edge vectors
  let vp_u = Vec3::new(vp_width as f64, 0, 0);
  let vp_v = Vec3::new(0, -(vp_height as f64), 0);
  // Delta vectors between pixels
  let px_d_u = vp_u / (vp_width as f64);
  let px_d_v = vp_v / (vp_height as f64);
  // Upper left viewport point & pixel
  let (vp_00, px_00) = upper_left_points(camera_center, focal_length, vp_u, vp_v, px_d_u, px_d_v);

  // dbg!(width, height, vp_width, vp_height, vp_u, vp_v, px_d_u, px_d_v, vp_00, px_00);

  print!("P3\n{} {}\n255\n", width, height);
  for j in 0..height {
    eprint!("\rLines remaining: {}", height - j);
    for i in 0..width {
      let px_center = *px_00 + px_d_u*(i as f64) + px_d_v*(j as f64);
      let ray_dir = px_center - camera_center;
      let ray = Ray::new(camera_center, ray_dir);

      let a = 0.5 * (ray.direction[1]/(height as f64) + 1.0);
      let white = Color::new(1.0, 1.0, 1.0).scale(1.0 - a);
      let blue = Color::new(0.5, 0.7, 1.0).scale(a);
      let color: Color = (white + blue).into();
      print!("{}\n", color);
    }
  }
  eprint!("\rDone.                                  \n");
}

fn image_dimensions(aspect_ratio: f64, width: u64) -> (u64, u64) {
  let height = f64::max(1.0, (width as f64) / aspect_ratio);
  (width, height as u64)
}

fn viewport_dimensions(image_width: u64, image_height: u64) -> (u64, u64) {
  let height = 2.0;
  let width = height * (image_width as f64) / (image_height as f64);
  (width as u64, height as u64)
} 

fn upper_left_points(camera_center: Point, focal_length: f64, vp_u: Vec3, vp_v: Vec3, px_d_u: Vec3, px_d_v: Vec3) -> (Point, Point) {
  let vp_00 = *camera_center - Vec3::new(0, 0, focal_length) - (vp_u/2.0) + (vp_v/2.0);
  let px_00 = vp_00 + (px_d_u + px_d_v)/2.0;
  (vp_00.into(), px_00.into())
}
