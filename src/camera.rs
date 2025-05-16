use std::f64::consts::PI;

use crate::objects::Hittable;
use crate::scene::Scene;
use crate::types::{Color, Image, Interval, Point, Ray, ToVec3, Vec3};

/// Caret return followed by ANSI erase line command sequence.
static CLEAR: &str = "\r\u{1b}[2K";

// MARK: - CameraSetup

/// A type that stores mandatory information for a camera.
#[derive(Debug, Clone, Copy)]
pub struct CameraSetup {
	/// The width of the image the camera produces, in pixels.
	pub width: usize,
	/// The height of the image the camera produces, in pixels.
	pub height: usize,
	/// The vertical field of view, in degrees.
	pub v_fov: f64,
	/// The position of the camera.
	pub lookfrom: Point,
	/// The point the camera is looking at.
	pub lookat: Point,
	/// The vector pointing from the camera upwards.
	pub view_up: Vec3,
	/// ???
	pub defocus_angle: f64,
	/// Distance from camera center to perfect focus plane.
	pub focus_distance: f64,
}
impl Default for CameraSetup {
	fn default() -> Self {
		Self {
			width: 400,
			height: 225,
			v_fov: 45.0,
			lookfrom: Point(0.0, 0.0, 0.0),
			lookat: Point(0.0, 0.0, -1.0),
			view_up: Vec3(0.0, 1.0, 0.0),
			defocus_angle: 0.0,
			focus_distance: 10.0
		}
	}
}
impl From<CameraSetup> for Camera {
	fn from(value: CameraSetup) -> Self {
		Camera::new(value)
	}
}

// MARK: - Camera

/// A type that represents a camera and stores all related configuration.
/// 
/// This type can only be constructed from a [`CameraSetup`] instance.
/// ```
/// let setup = CameraSetup { width: 3840, height: 2160, ..Default::default() };
/// let camera = Camera::from(setup);
/// ```
/// The camera setup stores mandatory parameters upon which many calculations depend.
/// Optional parameters can be set on the camera directly:
/// ```
/// let camera = camera.bounces(10);
/// ```
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
	v_fov: f64,
	// orientation,
	lookfrom: Point,
	lookat: Point,
	view_up: Vec3,
	// orthonormal basis
	u: Vec3,
	v: Vec3,
	w: Vec3,
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
	// depth of field
	defocus_angle: f64,
	defocus_disk_u: Vec3,
	defocus_disk_v: Vec3,
}

// Constructors
impl Camera {
	/// Creates a new camera capturing an image of specified dimensions.
	fn new(setup: CameraSetup) -> Self {
		// Image
		let aspect_ratio = (setup.width as f64) / (setup.height as f64);
		// Camera
		let direction = setup.lookfrom.to_vec3() - setup.lookat.to_vec3();
		let focal_length = direction.norm();
		let camera_center = setup.lookfrom;
		let (vp_width, vp_height) = Self::viewport_dimensions(&setup);
		// Orthronormal basis
		let w = direction.unit();
		let u = setup.view_up.cross(w).unit();
		let v = w.cross(u);
		// Viewport edge vectors
		let vp_u = u.scale(vp_width);
		let vp_v = -v.scale(vp_height);
		// Delta vectors between pixels
		let px_d_u = vp_u / (setup.width as f64);
		let px_d_v = vp_v / (setup.height as f64);
		// Upper left viewport point & pixel
		let (vp_00, px_00) = Self::upper_left_points(camera_center, setup.focus_distance, w, vp_u, vp_v, px_d_u, px_d_v);
		// Defocus disk
		let defocus_radius = setup.focus_distance * f64::tan(setup.defocus_angle / 2.0 * PI / 180.0);
		let defocus_disk_u = u.scale(defocus_radius);
		let defocus_disk_v = v.scale(defocus_radius);
		Self {
			aspect_ratio,
			img_size: (setup.width, setup.height),
			focal_length,
			center: camera_center,
			viewport_size: (vp_width, vp_height),
			v_fov: setup.v_fov,
			lookfrom: setup.lookfrom,
			lookat: setup.lookat,
			view_up: setup.view_up,
			u,
			v,
			w,
			vp_u,
			vp_v,
			px_d_u,
			px_d_v,
			vp_00,
			px_00,
			samples_per_px: 1,
			bounces: 1,
			defocus_angle: setup.defocus_angle,
			defocus_disk_u,
			defocus_disk_v
		}
	}
	/// Calculates the dimensions of the viewport from specified image dimensions.
	/// The aspect ratio remains unchanged.
	fn viewport_dimensions(setup: &CameraSetup) -> (f64, f64) {
		let h = f64::tan(setup.v_fov / 2.0 * PI / 180.0);
		let height = 2.0 * h * setup.focus_distance;
		let width = height * (setup.width as f64) / (setup.height as f64);
		(width, height)
	}
	/// Calculates the upper left viewport and pixel points.
	fn upper_left_points(camera_center: Point, focus_dist: f64, w: Vec3, vp_u: Vec3, vp_v: Vec3, px_d_u: Vec3, px_d_v: Vec3)
	-> (Point, Point) {
		let vp_00 = camera_center.to_vec3() - w.scale(focus_dist) - (vp_u/2.0) - (vp_v/2.0);
		let px_00 = vp_00 + (px_d_u + px_d_v)/2.0;
		(vp_00.into(), px_00.into())
	}
}

// Optional features
impl Camera {
	/// Controls supersampling for this camera.
	/// The amount of samples per pixel is specified by the parameter, which should be at least 1.
	/// If it is less than 1, one sample per pixel is assumed (and thus no anti-aliasing).
	pub fn anti_aliasing(self, samples: u32) -> Self {
		Camera { samples_per_px: u32::max(1, samples), ..self }
	}
	/// Specifies how many times a ray can bounce until the color is determined.
	/// An amount of 0 means rays do not bounce and only return the color of the surface they land on.
	pub fn bounces(self, bounces: u32) -> Self {
		Camera { bounces, ..self }
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
		let px_offset = self.sampling_offset();
		let px_sample = self.px_00.to_vec3()
				+ (self.px_d_u * ((px_i as f64) + px_offset.x()))
				+ (self.px_d_v * ((px_j as f64) + px_offset.y()));

		let origin_offset = self.sampling_disk_offset();
		let origin = self.center.to_vec3()
				+ self.defocus_disk_u.scale(origin_offset.x())
				+ self.defocus_disk_v.scale(origin_offset.y());
		let origin = origin.into();

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
	fn sampling_disk_offset(&self) -> Vec3 {
		if self.defocus_angle > 0.0 {
			Vec3::random_in_unit_disk()
		} else {
			Vec3::zero()
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::camera::CameraSetup;

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
		let setup = CameraSetup { width: 5, height: 5, ..Default::default() };
		let camera = Camera::from(setup);
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
		let setup = CameraSetup { width: 5, height: 5, ..Default::default() };
		let camera = Camera::from(setup).anti_aliasing(samples);
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
