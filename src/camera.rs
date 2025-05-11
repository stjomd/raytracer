use crate::objects::{Hittable, Objects};
use crate::types::{Color, Interval, Point, Ray, ToVec3, Vec3};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Camera {
	// image
	aspect_ratio: f64,
	img_size: (u64, u64),
	// camera
	focal_length: f64,
	camera_center: Point,
	viewport_size: (f64, f64),
	// viewport edge vectors
	vp_u: Vec3,
	vp_v: Vec3,
	// delta vectors between pixels
	px_d_u: Vec3,
	px_d_v: Vec3,
	// upper left point (viewport & pixel)
	vp_00: Point,
	px_00: Point,
}

impl Camera {
  pub fn new(aspect_ratio: f64, width: u64) -> Self {
    // Image
    let (width, height) = Self::image_dimensions(aspect_ratio, width);
    // Camera
    let focal_length = 1.0;
    let camera_center = Point::origin();
    let (vp_width, vp_height) = Self::viewport_dimensions(width, height);
    // Viewport edge vectors
    let vp_u = Vec3::new(vp_width, 0, 0);
    let vp_v = Vec3::new(0, -vp_height, 0);
    // Delta vectors between pixels
    let px_d_u = vp_u / (width as f64);
    let px_d_v = vp_v / (height as f64);
    // Upper left viewport point & pixel
    let (vp_00, px_00) = Self::upper_left_points(camera_center, focal_length, vp_u, vp_v, px_d_u, px_d_v);
    Self {
      aspect_ratio,
      img_size: (width, height),
      focal_length,
      camera_center,
      viewport_size: (vp_width, vp_height),
      vp_u,
      vp_v,
      px_d_u,
      px_d_v,
      vp_00,
      px_00,
    }
  }

  fn image_dimensions(aspect_ratio: f64, width: u64) -> (u64, u64) {
    let height = f64::max(1.0, (width as f64) / aspect_ratio);
    (width, height as u64)
  }
  
  fn viewport_dimensions(image_width: u64, image_height: u64) -> (f64, f64) {
    let height = 2.0;
    let width = height * (image_width as f64) / (image_height as f64);
    (width, height)
  } 
  
  fn upper_left_points(camera_center: Point, focal_length: f64, vp_u: Vec3, vp_v: Vec3, px_d_u: Vec3, px_d_v: Vec3) -> (Point, Point) {
    let vp_00 = camera_center.to_vec3() - Vec3::new(0, 0, focal_length) - (vp_u/2.0) - (vp_v/2.0);
    let px_00 = vp_00 + (px_d_u + px_d_v)/2.0;
    (vp_00.into(), px_00.into())
  }
}

impl Camera {
	pub fn render(&self, objects: &Objects) {
		let (width, height) = self.img_size;
		print!("P3\n{} {}\n255\n", width, height);
		for j in 0..height {
			eprint!("\rLines remaining: {}", height - j);
			for i in 0..width {
				let px_center = self.px_00.to_vec3() + self.px_d_u*(i as f64) + self.px_d_v*(j as f64);
				let ray_dir = px_center - self.camera_center;
				let ray = Ray::new(self.camera_center, ray_dir);
				let color = self.ray_color(ray, objects);
				println!("{}", color);
			}
		}
		eprint!("\rDone.                                  \n");
	}
	fn ray_color(&self, ray: Ray, objects: &Objects) -> Color {
		if let Some(hit) = objects.hit(ray, Interval::new(0, f64::INFINITY)) {
			return (hit.normal + Vec3::diagonal(1)).scale(0.5).into()
		}
		// background
		let a = 0.5 * (ray.direction[1]/self.viewport_size.1 + 1.0);
		let white = Color::new(1.0, 1.0, 1.0).to_vec3().scale(1.0 - a);
		let blue = Color::new(0.5, 0.7, 1.0).to_vec3().scale(a);
		(white + blue).into()
	}
}
