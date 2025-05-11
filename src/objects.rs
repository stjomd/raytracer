#![allow(unused)]

mod hit;
mod sphere;

pub use hit::{Hittable, Hit};
pub use sphere::Sphere;

/// A collection of objects in the scene.
pub struct Objects {
	list: Vec<Box<dyn Hittable>>
}

impl Objects {
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

impl Hittable for Objects {
	fn hit(&self, ray: crate::types::Ray, t_range: std::ops::RangeInclusive<f64>) -> Option<Hit> {
		let mut t_max = *t_range.end();
		let mut closest_hit: Option<Hit> = None;
		for obj in &self.list {
			let hit = obj.hit(ray, *t_range.start() ..= t_max);
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
macro_rules! objects {
		() => { objects::Objects::new() };
		( $($obj:expr),* $(,)? ) => {
			{
				let mut tmp = crate::objects::Objects::new();
				$(
					tmp.add(Box::new($obj));
				)*
				tmp
			}
		};
}
pub(crate) use objects;

#[cfg(test)]
mod tests {
	use crate::types::{Point, Ray, Vec3};
	use super::{Hittable, Objects, Sphere};

	#[test]
	fn if_many_objects_then_should_hit_nearest() {
		// These two spheres are positioned after each other on the x-axis:
		let sphere1 = Sphere::new(Point::new(1.5, 0, 0), 0.5);
		let sphere2 = Sphere::new(Point::new(3.5, 0, 0), 0.5);
		// This collection contains the two spheres:
		let objects = objects!(sphere1, sphere2);
		// This ray starts at origin and shoots horizontally along the x-axis into the spheres:
		let ray = Ray::new(Point::origin(), Vec3::new(1, 0, 0));

		// We should see the intersection with the first sphere, as it's closer to the ray's origin:
		let hit = objects.hit(ray, -10.0 ..= 10.0);
		assert!(hit.is_some(), "ray should hit the first sphere, but didn't hit anything");
		let hit = hit.unwrap();
		assert_eq!(hit.point, Point::new(1, 0, 0), "ray should hit the first sphere, but hit another point {}", hit.point);
	}
}
