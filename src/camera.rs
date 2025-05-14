use crate::objects::Hittable;
use crate::scene::Scene;
use crate::types::{Color, Image, Interval, Point, Ray, ToVec3, Vec3};

/// The viewport height.
/// The corresponding width should be calculated from the image's dimensionsto have the same aspect ratio.
const VIEWPORT_HEIGHT: f64 = 2.0;

/// Caret return followed by ANSI erase line command sequence.
static CLEAR: &str = "\r\u{1b}[2K";

/// A type that represents a camera and stores all related configuration.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Camera {
	// image
	aspect_ratio: f64,
	img_size: (usize, usize),
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
	// reflections
	bounces: u32,
}

// Constructors
impl Camera {
	/// Creates a new camera capturing an image of specified dimensions.
  pub fn new(width: usize, height: usize) -> Self {
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
			bounces: 1,
    }
  }
	/// Calculates the dimensions of the viewport from specified image dimensions.
	/// The aspect ratio remains unchanged.
  fn viewport_dimensions(image_width: usize, image_height: usize) -> (f64, f64) {
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
	/// Controls supersampling for this camera.
	/// The amount of samples per pixel is specified by the parameter, which should be at least 1.
	/// If it is less than 1, one sample per pixel is assumed (and thus no anti-aliasing).
	pub fn anti_aliasing(&mut self, samples: u32) {
		self.samples_per_px = u32::max(1, samples);
	}
	/// Specifies how many times a ray can bounce until the color is determined.
	/// An amount of 0 means rays do not bounce and only return the color of the surface they land on.
	pub fn bounces(&mut self, bounces: u32) {
		self.bounces = bounces;
	}
}

// Rendering
impl Camera {
	/// Renders a scene and produces an image.
	pub fn render(&self, scene: &Scene) -> Image {
		let (width, height) = self.img_size;
		let mut image = Image::init(height, width);
		for j in 0..height {
			eprint!("{CLEAR}Lines remaining: {}", height - j);
			for i in 0..width {
				let color = self.sample_pixel(i, j, scene);
				image[(j, i)] = color;
			}
		}
		eprintln!("{CLEAR}Done.");
		image
	}
	/// Samples a pixel and returns the average color.
	fn sample_pixel(&self, px_i: usize, px_j: usize, scene: &Scene) -> Color {
		let mut rgb = Vec3::zero();
		for _ in 0..self.samples_per_px {
			let ray = self.sampling_ray(px_i, px_j);
			rgb += self.ray_color(ray, scene, self.bounces).to_vec3();
		}
		rgb.scale(1.0 / (self.samples_per_px as f64)).into()
	}
	/// Calculates the color of a ray in the specified scene.
	fn ray_color(&self, ray: Ray, scene: &Scene, bounces: u32) -> Color {
		if bounces == 0 {
			return Color::black();
		}
		if let Some(hit) = scene.hit(ray, Interval::from(0.001)) {
			if let Some(scattered_ray) = hit.material.scatter(ray, hit) {
				let attenuation = scattered_ray.attenuation;
				let color = self.ray_color(scattered_ray, scene, bounces - 1);
				return (attenuation.to_vec3() * color.to_vec3()).into()
			} else {
				// ray was absorbed
				return Color::black();
			}
		}
		// background
		let a = 0.5 * (ray.direction.y()/self.viewport_size.1 + 1.0);
		let white = Color::new(1.0, 1.0, 1.0).to_vec3().scale(1.0 - a);
		let blue = Color::new(0.5, 0.7, 1.0).to_vec3().scale(a);
		(white + blue).into()
	}
	/// Creates a sampling ray for the pixel with index `(px_i, px_j)`.
	fn sampling_ray(&self, px_i: usize, px_j: usize) -> Ray {
		let offset = self.sampling_offset();
		let px_sample = self.px_00.to_vec3()
				+ (self.px_d_u * ((px_i as f64) + offset.x()))
				+ (self.px_d_v * ((px_j as f64) + offset.y()));
		let origin = self.center;
		let direction = px_sample - origin;
		Ray::new(origin, direction)
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

	/// Epsilon for f64 equality comparisons.
	/// Two f64 values are assumed to be equal if their difference is smaller than this value.
	const F64_EQ_EPSILON: f64 = 1e-10;
	/// Checks whether two `f64` values are approximately equal within [`F64_EQ_EPSILON`].
	fn f64_approx_eq(a: f64, b: f64) -> bool {
		f64::abs(a - b) < F64_EQ_EPSILON
	}

	#[test]
	fn if_pixel_above_center_then_ray_dir_only_z_axis() {
		// This camera produces a 5x5 image:
		let camera = Camera::new(5, 5);
		// This pixel is in the middle of the image and thus right above the camera center:
		let (px_i, px_j) = (2, 2);

		// The ray's direction should only be moving towards the viewport and no other direction:
		let ray = camera.sampling_ray(px_i, px_j);
		assert_eq!(ray.direction.x(), 0.0, "the ray's direction should be only in the z-axis, but x was {}", ray.direction.x());
		assert_eq!(ray.direction.y(), 0.0, "the ray's direction should be only in the z-axis, but y was {}", ray.direction.y());
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
			let eq_x = f64_approx_eq(ray.direction.x(), camera.center.x());
			let eq_y = f64_approx_eq(ray.direction.y(), camera.center.y());
			if !eq_x || !eq_y {
				has_deviating_rays = true;
				break;
			}
		}
		assert!(has_deviating_rays, "at least one ray should deviate due to anti-aliasing, but all rays hit pixel center")
	}
}
