use crate::objects::Hit;
use crate::types::{Color, Ray, Vec3};

/// A type that describes a material of a surface.
/// 
/// This is used to mimic dynamic dispatch to simplify handling of different materials
/// (so that we do not have to use `Box<dyn Material>` and deal with its consequences).
/// For this reason, matching against this type should be avoided.
/// Instead, use the available methods on the instance.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Material {
	/// A material which absorbs all light.
	Absorbant,
	/// A matte material with Lambertian reflectance.
	Matte { color: Color },
	/// A metallic, reflective material.
	/// 
	/// The `fuzz` parameter describes how imperfect the surface is.
	/// A value of 0 describes a perfectly reflective metal,
	/// while a value of 1 describes a rough/brushed surface.
	/// Values outside the `0..=1` range are clamped.
	Metal { color: Color, fuzz: f64 },
}

impl Material {
	/// Calculates the scattered (bouncing) ray, depending on the material.
	/// 
	/// Accepts the incoming `ray` onto the surface, and the `hit` from which the ray should scatter.
	/// 
	/// Returns either a scattered ray, or `None` if the ray was completely absorbed.
	pub fn scatter(&self, ray: Ray, hit: Hit) -> Option<Ray> {
		match self {
			Self::Absorbant => None,
			Self::Matte { color } => scatter_matte(hit, *color),
			Self::Metal { color, fuzz } => scatter_metal(ray, hit, *color, *fuzz),
		}
	}
}

/// Calculates the scattered ray off a matte material.
fn scatter_matte(hit: Hit, color: Color) -> Option<Ray> {
	let mut direction = hit.normal + Vec3::random_unit();
	if direction.is_near_zero() {
		direction = hit.normal
	}
	Some(Ray::newc(hit.point, direction, color))
}

/// Calculates the scattered ray off a metallic material.
fn scatter_metal(ray: Ray, hit: Hit, color: Color, fuzz: f64) -> Option<Ray> {
	let fuzz = fuzz.clamp(0.0, 1.0);
	let factor = 2.0 * ray.direction.dot(hit.normal);
	let perfect_direction = ray.direction - hit.normal.scale(factor);
	let direction = perfect_direction.unit() + Vec3::random_unit().scale(fuzz);
	// if direction vector lands below the surface, absorb
	if direction.dot(hit.normal) > 0.0 {
		Some(Ray::newc(hit.point, direction, color))
	} else {
		None
	}
}
