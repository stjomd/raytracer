use crate::core::objects::Hittable;
use crate::scene::Scene;

use super::vector::ToVec3;
use super::{Color, Interval, Point, Vec3};

/// A representation of a ray.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
	/// The starting point of a ray.
	pub origin: Point,
	/// The direction where the ray is pointing.
	pub direction: Vec3,
	/// A measure of how much luminance this ray keeps.
	pub attenuation: Color,
}

// Constructors
impl Ray {
	/// Creates a ray with full attenuation (factor of 1).
	pub fn new(origin: Point, direction: Vec3) -> Self {
		Ray { origin, direction, attenuation: Color::new(1, 1, 1) }
	}
	/// Creates a ray with a specified color/attenuation.
	pub fn newc(origin: Point, direction: Vec3, color: Color) -> Self {
		Ray { origin, direction, attenuation: color }
	}
}

// Operations
impl Ray {
	/// Calculates the point where a ray is located at a specific parameter `t`.
	/// This parameter can be (albeit misguidedly) understood as 'time'.
	pub fn at(&self, t: f64) -> Point {
		let point = self.origin.to_vec3() + self.direction.scale(t);
		point.into()
	}
	/// Calculates the color of a ray in the specified scene.
	pub fn color(self, scene: &Scene, bounces: u32) -> Color {
		if bounces == 0 {
			return Color::black();
		}
		if let Some(hit) = scene.hit(self, Interval::from(0.001)) {
			if let Some(scattered_ray) = hit.material.scatter(self, hit) {
				let attenuation = scattered_ray.attenuation;
				let color = scattered_ray.color(scene, bounces - 1);
				return (attenuation.to_vec3() * color.to_vec3()).into()
			} else {
				// ray was absorbed
				return Color::black();
			}
		}
		// background
		let a = 0.5 * (self.direction.unit().y() + 1.0);
		let white = Color::new(1.0, 1.0, 1.0).to_vec3().scale(1.0 - a);
		let blue = Color::new(0.5, 0.7, 1.0).to_vec3().scale(a);
		(white + blue).into()
	}
}
