use serde::Deserialize;

use super::objects::{Hit, Hittable, Object, ToObject};
use super::types::Interval;

/// A collection of objects to be rendered.
#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Scene {
	list: Vec<Object>,
}

impl Scene {
	/// Creates a new empty scene, without any objects.
	pub fn new() -> Self {
		Self { list: Vec::new() }
	}
	/// Adds an object to this scene.
	pub fn add<T: Hittable + ToObject>(&mut self, obj: T) {
		self.list.push(obj.wrap());
	}
	/// Appends a collection of objects to this scene.
	///
	/// You can chain this method multiple times to append objects of different types:
	/// ```
	/// let scene = Scene::from(objects)
	///   .append([sphere_left, sphere_right])
	///   .append([triangle_top, triangle_center, triangle_bottom]);
	/// ```
	/// Each call returns a [`Scene`] instance which contains all objects appended in and before it.
	pub fn append<I, O>(mut self, objs: I) -> Self
	where
		I: IntoIterator<Item = O>,
		O: Hittable + ToObject,
	{
		let mut wrapped_objs = objs.into_iter().map(|obj| obj.wrap()).collect::<Vec<_>>();
		self.list.append(&mut wrapped_objs);
		self
	}
	/// Removes all objects from this scene.
	pub fn clear(&mut self) {
		self.list.clear();
	}
}

// ::from constructor
impl<I, O> From<I> for Scene
where
	I: IntoIterator<Item = O>,
	O: Hittable + ToObject,
{
	fn from(value: I) -> Self {
		let objects = value.into_iter().map(|obj| obj.wrap()).collect::<Vec<_>>();
		Self { list: objects }
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
	use super::Scene;
	use crate::core::objects::{Hittable, Material, Sphere};
	use crate::core::types::{Color, Interval, Point, Ray, Vec3};
	use crate::objects::Object;

	#[test]
	fn if_many_objects_then_should_hit_nearest() {
		// These two spheres are positioned after each other on the x-axis:
		let sphere1 = Sphere::new(Point::new(1.5, 0, 0), 0.5, Material::Absorbant);
		let sphere2 = Sphere::new(Point::new(3.5, 0, 0), 0.5, Material::Absorbant);
		let objects = Scene::from([sphere1, sphere2]);
		// This ray starts at origin and shoots horizontally along the x-axis into the spheres:
		let ray = Ray::new(Point::origin(), Vec3::new(1, 0, 0));

		// We should see the intersection with the first sphere, as it's closer to the ray's origin:
		let hit = objects.hit(ray, Interval::from(0));
		assert!(
			hit.is_some(),
			"ray should hit the first sphere, but didn't hit anything"
		);
		let hit = hit.unwrap();
		assert_eq!(
			hit.point,
			Point::new(1, 0, 0),
			"ray should hit the first sphere, but hit another point {}",
			hit.point
		);
	}

	#[test]
	fn if_no_objects_then_no_hit() {
		// This scene has no objects:
		let mut scene = Scene::new();
		scene.clear();
		// This ray shoots somewhere:
		let ray = Ray::new(Point::origin(), Vec3::diagonal(1));

		// There should be no intersection
		let hit = scene.hit(ray, Interval::from(0));
		assert!(
			hit.is_none(),
			"ray shouldn't hit anything as scene is empty, but there was a hit"
		);
	}

	#[test]
	fn if_ray_shoots_into_void_then_no_hit() {
		// This sphere is located on the x-axis at x=10:
		let sphere = Sphere::new(
			Point::new(10, 0, 0),
			1,
			Material::Matte {
				color: Color::black(),
			},
		);
		// This scene only has the object:
		let mut scene = Scene::new();
		scene.add(sphere);
		// This ray does not shoot towards the spheres:
		let ray = Ray::new(Point::origin(), Vec3::new(0, 1, 0));

		// There should be no intersection
		let hit = scene.hit(ray, Interval::from(0));
		assert!(
			hit.is_none(),
			"ray shouldn't hit anything as ray doesn't shoot towards the object, but there was a hit"
		);
	}

	#[test]
	fn builder_contains_all_objects() {
		// These are all the spheres intended for the scene:
		let spheres = [
			Sphere::new(Point::origin(), 1.0, Material::Absorbant),
			Sphere::new(Point::origin(), 2.0, Material::Absorbant),
			Sphere::new(Point::origin(), 3.0, Material::Absorbant),
			Sphere::new(Point::origin(), 4.0, Material::Absorbant),
			Sphere::new(Point::origin(), 5.0, Material::Absorbant),
		];

		// Appending all of them in multiple .append calls should contain all of them in the end:
		let scene = Scene::new()
			.append([spheres[0], spheres[1]])
			.append([spheres[2], spheres[3]])
			.append([spheres[4]]);

		let mut missing_objects: Vec<Sphere> = Vec::new();
		for sphere in spheres {
			if !scene.list.contains(&Object::Sphere(sphere)) {
				missing_objects.push(sphere);
			}
		}
		assert!(
			missing_objects.is_empty(),
			"multiple `.append`s should collect all objects, but these were missing: {:?}",
			missing_objects
		)
	}
}
