use crate::objects::Hittable;
use crate::scene::Scene;
use crate::types::{Color, Interval, Point, Ray, ToVec3, Vec3};

/// The viewport height.
/// The corresponding width should be calculated from the image's dimensionsto have the same aspect ratio.
const VIEWPORT_HEIGHT: f64 = 2.0;

/// A type that represents a camera and stores all related configuration.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Camera {
	// image
	aspect_ratio: f64,
	img_size: (u64, u64),
	// camera
	focal_length: f64,
	center: Point,
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
	// anti-aliasing
	samples_per_px: u32,
}

// Constructors
impl Camera {
	/// Creates a new camera capturing an image of specified dimensions.
  pub fn new(width: u64, height: u64) -> Self {
    // Image
		let aspect_ratio = (width as f64) / (height as f64);
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
      center: camera_center,
      viewport_size: (vp_width, vp_height),
      vp_u,
      vp_v,
      px_d_u,
      px_d_v,
      vp_00,
      px_00,
			samples_per_px: 1,
    }
  }
	/// Calculates the dimensions of the viewport from specified image dimensions.
	/// The aspect ratio remains unchanged.
  fn viewport_dimensions(image_width: u64, image_height: u64) -> (f64, f64) {
    let height = VIEWPORT_HEIGHT;
    let width = height * (image_width as f64) / (image_height as f64);
    (width, height)
  }
	/// Calculates the upper left viewport and pixel points.
  fn upper_left_points(camera_center: Point, focal_length: f64, vp_u: Vec3, vp_v: Vec3, px_d_u: Vec3, px_d_v: Vec3)
	-> (Point, Point) {
    let vp_00 = camera_center.to_vec3() - Vec3::new(0, 0, focal_length) - (vp_u/2.0) - (vp_v/2.0);
    let px_00 = vp_00 + (px_d_u + px_d_v)/2.0;
    (vp_00.into(), px_00.into())
  }
}

// Optional features
impl Camera {
	/// Enables supersampling for this camera.
	/// The amount of samples per pixel is specified by the parameter, which should be at least 1.
	/// If it is less than 1, one sample per pixel is assumed (and thus no anti-aliasing).
	pub fn anti_aliasing(&mut self, samples: u32) {
		self.samples_per_px = u32::max(1, samples);
	}
}

// Rendering
impl Camera {
	/// Renders a scene and produces a .ppm image to stdout.
	pub fn render(&self, scene: &Scene) {
		let (width, height) = self.img_size;
		print!("P3\n{} {}\n255\n", width, height);
		for j in 0..height {
			eprint!("\rLines remaining: {}", height - j);
			for i in 0..width {
				// Sample pixel multiple times and accumulate the color value
				let mut rgb = Vec3::zero();
				for _ in 0..self.samples_per_px {
					let ray = self.sampling_ray(i, j);
					rgb += self.ray_color(ray, scene).to_vec3();
				}
				// The resulting color is the average over all samples
				let color: Color = rgb.scale(1.0 / (self.samples_per_px as f64)).into();
				println!("{}", color);
			}
		}
		eprint!("\rDone.                                  \n");
	}
	/// Calculates the color of a ray in the specified scene.
	fn ray_color(&self, ray: Ray, scene: &Scene) -> Color {
		if let Some(hit) = scene.hit(ray, Interval::new(0, f64::INFINITY)) {
			return (hit.normal + Vec3::diagonal(1)).scale(0.5).into()
		}
		// background
		let a = 0.5 * (ray.direction[1]/self.viewport_size.1 + 1.0);
		let white = Color::new(1.0, 1.0, 1.0).to_vec3().scale(1.0 - a);
		let blue = Color::new(0.5, 0.7, 1.0).to_vec3().scale(a);
		(white + blue).into()
	}
	/// Creates a sampling ray for the pixel with index `(px_i, px_j)`.
	fn sampling_ray(&self, px_i: u64, px_j: u64) -> Ray {
		let offset = self.sampling_offset();
		let px_sample = self.px_00.to_vec3()
				+ (self.px_d_u * ((px_i as f64) + offset.0))
				+ (self.px_d_v * ((px_j as f64) + offset.1));
		let origin = self.center;
		let direction = px_sample - origin;
		Ray { origin, direction }
	}
	/// Calculates a random offset in the `x` and `y` coordinates for supersampling.
	/// Both offsets lie in [-0.5; 0.5).
	/// If anti-aliasing is disabled for this camera, returns a zero vector.
	fn sampling_offset(&self) -> Vec3 {
		if self.samples_per_px > 1 {
			Vec3(
				rand::random_range(-0.5 .. 0.5),
				rand::random_range(-0.5 .. 0.5),
				0.0
			)
		} else {
			Vec3::zero()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Camera;

	#[test]
	fn if_pixel_above_center_then_ray_dir_only_z_axis() {
		// This camera produces a 5x5 image:
		let camera = Camera::new(5, 5);
		// This pixel is in the middle of the image and thus right above the camera center:
		let (px_i, px_j) = (2, 2);

		// The ray's direction should only be moving towards the viewport and no other direction:
		let ray = camera.sampling_ray(px_i, px_j);
		assert_eq!(ray.direction.0, 0.0, "the ray's direction should be only in the z-axis, but x was {}", ray.direction.0);
		assert_eq!(ray.direction.1, 0.0, "the ray's direction should be only in the z-axis, but y was {}", ray.direction.0);
	}

	#[test]
	fn if_pixel_above_center_and_antialiasing_then_some_ray_dir_also_xy_axis() {
		// A pixel should be sampled this many times:
		let samples = 10;
		// This camera produces a 5x5 image, and has enabled anti-aliasing:
		let mut camera = Camera::new(5, 5);
		camera.anti_aliasing(samples);
		// This pixel is in the middle of the image and thus right above the camera center:
		let (px_i, px_j) = (2, 2);

		// Since supersampling is enabled, all rays intersect the viewport within the 0.5-window of the pixel center.
		// Thus, we can expect at least one ray's direction to also have a non-zero x- and y-component:
		let mut has_deviating_rays = false;
		for _ in 0..samples {
			let ray = camera.sampling_ray(px_i, px_j);
			// At least x or y of the ray's direction vector should not equal the corresponding camera center's coordinate:
			let eq_x = f64_approx_eq(ray.direction.0, camera.center.x());
			let eq_y = f64_approx_eq(ray.direction.1, camera.center.y());
			if !eq_x || !eq_y {
				has_deviating_rays = true;
				break;
			}
		}
		assert!(has_deviating_rays, "at least one ray should deviate due to anti-aliasing, but all rays hit pixel center")
	}

	/// Epsilon for f64 equality comparisons.
	/// Two f64 values are assumed to be equal if their difference is smaller than this value.
	const F64_EQ_EPSILON: f64 = 1e-10;
	/// Checks whether two `f64` values are approximately equal within [`F64_EQ_EPSILON`].
	fn f64_approx_eq(a: f64, b: f64) -> bool {
		f64::abs(a - b) < F64_EQ_EPSILON
	}
}
