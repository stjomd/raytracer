#![allow(unused)]

use super::objects::{Hit, Hittable, Object};
use super::types::Interval;

/// A collection of objects in the scene.
#[derive(Default)]
pub struct Scene {
	list: Vec<Object>
}

impl Scene {
	/// Creates a new empty collection of objects.
	pub fn new() -> Self {
		Self { list: Vec::new() }
	}
	/// Adds a hittable object to this collection.
	pub fn add(&mut self, obj: Object) {
		self.list.push(obj);
	}
	/// Removes all objects from this collection.
	pub fn clear(&mut self) {
		self.list.clear();
	}
}

// Sugar
impl<const N: usize> From<[Object; N]> for Scene {
	fn from(value: [Object; N]) -> Self {
		Self { list: value.to_vec() }
	}
}

// Handle as collection of hittables
impl Hittable for Scene {
	fn hit(&self, ray: super::types::Ray, t_range: super::types::Interval) -> Option<Hit> {
		let mut t_max = t_range.end;
		let mut closest_hit: Option<Hit> = None;
		for obj in &self.list {
			let hit = obj.hit(ray, Interval::new(t_range.start, t_max));
			if let Some(_hit) = hit {
				t_max = _hit.t;
				closest_hit = hit;
			}
		}
		closest_hit
	}
}

#[cfg(test)]
mod tests {
	use crate::core::objects::{Hittable, Material, Object, Sphere, ToObject};
	use crate::core::types::{Color, Interval, Point, Ray, Vec3};
	use super::Scene;

	#[test]
	fn if_many_objects_then_should_hit_nearest() {
		// These two spheres are positioned after each other on the x-axis:
		let sphere1 = Sphere::new(Point::new(1.5, 0, 0), 0.5, Material::Absorbant);
		let sphere2 = Sphere::new(Point::new(3.5, 0, 0), 0.5, Material::Absorbant);
		let objects = Scene::from([sphere1.obj(), sphere2.obj()]);
		// This ray starts at origin and shoots horizontally along the x-axis into the spheres:
		let ray = Ray::new(Point::origin(), Vec3::new(1, 0, 0));

		// We should see the intersection with the first sphere, as it's closer to the ray's origin:
		let hit = objects.hit(ray, Interval::new(0, 10));
		assert!(hit.is_some(), "ray should hit the first sphere, but didn't hit anything");
		let hit = hit.unwrap();
		assert_eq!(hit.point, Point::new(1, 0, 0), "ray should hit the first sphere, but hit another point {}", hit.point);
	}
}
