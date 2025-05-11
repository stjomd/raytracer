#![allow(unused)]

use crate::objects::{Hit, Hittable};
use crate::types::Interval;

/// A collection of objects in the scene.
pub struct Scene {
	list: Vec<Box<dyn Hittable>>
}

impl Scene {
	/// Creates a new empty collection of objects.
	pub fn new() -> Self {
		Self { list: Vec::new() }
	}
	/// Adds a hittable object to this collection.
	pub fn add(&mut self, obj: Box<dyn Hittable>) {
		self.list.push(obj);
	}
	/// Removes all objects from this collection.
	pub fn clear(&mut self) {
		self.list.clear();
	}
}

impl Hittable for Scene {
	fn hit(&self, ray: crate::types::Ray, t_range: crate::types::Interval) -> Option<Hit> {
		let mut t_max = t_range.end;
		let mut closest_hit: Option<Hit> = None;
		for obj in &self.list {
			let hit = obj.hit(ray, Interval::new(t_range.start, t_max));
			if let Some(hit) = hit {
				t_max = hit.t;
				closest_hit = Some(hit);
			}
		}
		closest_hit
	}
}

/// Creates a collection with the specified objects.
/// Each of the objects is boxed before being added into the collection.
macro_rules! scene {
		() => { crate::scene::Scene::new() };
		( $($obj:expr),* $(,)? ) => {
			{
				let mut tmp = crate::scene::Scene::new();
				$(
					tmp.add(Box::new($obj));
				)*
				tmp
			}
		};
}
pub(crate) use scene;

#[cfg(test)]
mod tests {
	use crate::objects::{Hittable, Sphere};
	use crate::types::{Interval, Point, Ray, Vec3};

	#[test]
	fn if_many_objects_then_should_hit_nearest() {
		// These two spheres are positioned after each other on the x-axis:
		let sphere1 = Sphere::new(Point::new(1.5, 0, 0), 0.5);
		let sphere2 = Sphere::new(Point::new(3.5, 0, 0), 0.5);
		// This collection contains the two spheres:
		let objects = scene!(sphere1, sphere2);
		// This ray starts at origin and shoots horizontally along the x-axis into the spheres:
		let ray = Ray::new(Point::origin(), Vec3::new(1, 0, 0));

		// We should see the intersection with the first sphere, as it's closer to the ray's origin:
		let hit = objects.hit(ray, Interval::new(0, 10));
		assert!(hit.is_some(), "ray should hit the first sphere, but didn't hit anything");
		let hit = hit.unwrap();
		assert_eq!(hit.point, Point::new(1, 0, 0), "ray should hit the first sphere, but hit another point {}", hit.point);
	}
}
