use serde::Deserialize;

use crate::core::types::{Interval, Point, Ray, Vec3};

use super::{Material, Sphere};

/// A type that wraps hittable objects.
/// This is done for performance improvements (static dispatch).
// -Also we can avoid messing with Box<dyn Hittable> :)
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Object {
	/// A sphere.
	Sphere(Sphere),
}

/// Represents an object hittable/intersectable by a ray.
pub trait Hittable {
	/// Calculates the intersection point of the specified ray with this object.
	/// Additionally, validates if the parameter `t` lies within the specified (plausible) range.
	/// If `t` lies outside the range, returns [`None`]; otherwise a [`Hit`] object.
	fn hit(&self, ray: Ray, t_range: Interval) -> Option<Hit>;
}

/// A trait to wrap objects into an [`Object`] enum.
pub trait ToObject {
	/// Wraps this object into an [`Object`] enum instance.
	fn wrap(self) -> Object;
}

// Dispatch methods
impl Hittable for Object {
	fn hit(&self, ray: Ray, t_range: Interval) -> Option<Hit> {
		match self {
			Self::Sphere(sphere) => sphere.hit(ray, t_range),
		}
	}
}

/// Represents an intersection between a ray and an object in the scene.
#[derive(Debug, Clone, Copy)]
pub struct Hit {
	/// The time parameter along the the ray vector axis.
	pub t: f64,
	/// The intersection point.
	pub point: Point,
	/// The normal vector at the intersection point.
	pub normal: Vec3,
	/// Determines if the ray hits from outside the object (`true`) or inside (`false`).
	pub is_front_face: bool,
	/// The material of the surface that was hit.
	pub material: Material,
}

impl Hit {
	/// Calculates the orientation between the ray and the outward normal.
	///
	/// The parameter `outward_normal` **must** be a unit, normal vector.
	///
	/// Returns a tuple containing:
	/// - the normal vector ([`Vec3`]) that is oriented in the opposite direction as `ray`,
	///   if it hits the object from outside; and in the same direction otherwise;
	/// - a boolean value indicating the orientation: `true` if the ray hits from outside,
	///   `false` otherwise.
	pub fn determine_front_face(ray: Ray, outward_normal: Vec3) -> (Vec3, bool) {
		let is_front_face = ray.direction.dot(outward_normal) < 0.0;
		let normal = if is_front_face {
			outward_normal
		} else {
			-outward_normal
		};
		(normal, is_front_face)
	}
}

#[cfg(test)]
mod tests {
	use super::Hit;
	use crate::core::types::{Point, Ray, Vec3};

	#[test]
	fn if_ray_hits_from_outside_then_front_face() {
		// this ray shoots out from origin horizontally (x-axis) into the positive direction:
		let ray = Ray::new(Point::origin(), Vec3::new(1, 0, 0));
		// this normal points outwards the object
		let outward_normal = Vec3::new(-1, 0, 0);

		let (normal, is_front_face) = Hit::determine_front_face(ray, outward_normal);
		assert_eq!(normal, outward_normal, "normal should not be negated");
		assert!(
			is_front_face,
			"is_front_face should be true, was `{is_front_face}`"
		);
	}

	#[test]
	fn if_ray_hits_from_inside_then_not_front_face() {
		// this ray shoots out from origin horizontally (x-axis) into the positive direction:
		let ray = Ray::new(Point::origin(), Vec3::new(1, 0, 0));
		// this normal points outwards the object
		let outward_normal = Vec3::new(1, 0, 0);

		let (normal, is_front_face) = Hit::determine_front_face(ray, outward_normal);
		assert_eq!(normal, -outward_normal, "normal should be negated");
		assert!(
			!is_front_face,
			"is_front_face should be false, was `{is_front_face}`"
		);
	}
}
