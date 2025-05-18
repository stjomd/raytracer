use crate::core::objects::Hit;
use crate::core::types::{Color, Ray, Vec3};

/// A type that describes a material of a surface.
//
// This is used to mimic dynamic dispatch to simplify handling of different materials
// (so that we do not have to use `Box<dyn Material>` and deal with its consequences).
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
	/// A transparent, dielectric material.
	/// 
	/// The `ridx` parameter is the refractive index of the material.
	/// For glass, use a value of 1.5-1.7; for diamonds 2.4.
	Dielectric { ridx: f64 },
}
// Keep the list in sync (used in tests)
#[allow(dead_code)]
const ALL_MATERIALS: &[Material] = &[
	Material::Absorbant,
	Material::Matte { color: Color::black() },
	Material::Metal { color: Color::black(), fuzz: 0.0 },
	Material::Dielectric { ridx: 1.0 }
];

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
			Self::Dielectric { ridx } => scatter_dielectric(ray, hit, *ridx),
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
	let direction = reflect_dir(ray.direction, hit.normal) + Vec3::random_unit().scale(fuzz);
	// if direction vector lands below the surface, absorb
	if direction.dot(hit.normal) > 0.0 {
		Some(Ray::newc(hit.point, direction, color))
	} else {
		None
	}
}

/// Calculates the scattered ray off a dielectric material.
fn scatter_dielectric(ray: Ray, hit: Hit, ridx: f64) -> Option<Ray> {
	let ri = if hit.is_front_face {
		1.0 / ridx
	} else {
		ridx
	};

	let unit_dir = ray.direction.unit();
	let cos_theta = f64::min(1.0, -unit_dir.dot(hit.normal));
	let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
	let can_refract = ri * sin_theta <= 1.0;

	if can_refract || reflectance(cos_theta, 1.0, ridx) > rand::random_range(0.0 .. 1.0) {
		let direction = refract_dir(unit_dir, hit.normal, ri);
		Some(Ray::new(hit.point, direction))
	} else {
		let direction = reflect_dir(ray.direction, hit.normal);
		Some(Ray::new(hit.point, direction))
	}
}

/// Calculates the specular reflection coefficient using Schlick's approximation.
/// 
/// The `cos` parameter denotes the cosine of the angle between the incoming ray direction and the normal;
/// `ridx1` is the refraction index of the surrounding medium;
/// and `ridx1` is the refraction index of the material.
fn reflectance(cos: f64, ridx1: f64, ridx2: f64) -> f64 {
	let r0_sqrt = (ridx1 - ridx2) / (ridx1 + ridx2);
	let r0 = r0_sqrt * r0_sqrt;
	r0 + (1.0 - r0)*(1.0 - cos).powi(5)
}

/// Calculates the reflection direction.
/// 
/// The `incoming` parameter denotes the incoming direction onto the surface;
/// and `normal` is the normal vector at the hit point.
fn reflect_dir(incoming: Vec3, normal: Vec3) -> Vec3 {
	let factor = 2.0 * incoming.dot(normal);
	incoming - normal.scale(factor)
}

/// Calculates the refraction direction.
/// 
/// The `incoming` parameter denotes the incoming direction onto the surface;
/// `normal` is the normal vector at the hit point;
/// and `ridx_ratio` is the ratio of the medium's refractive index to the material's refractive index.
fn refract_dir(incoming: Vec3, normal: Vec3, ridx_ratio: f64) -> Vec3 {
	let direction = incoming.unit();
	let cos_theta = f64::min(1.0, (-direction).dot(normal));
	let r_perp = (direction + normal.scale(cos_theta)).scale(ridx_ratio);
	let r_parl = normal * -(1.0 - r_perp.norm_sq()).abs().sqrt();
	r_perp + r_parl
}

#[cfg(test)]
mod tests {
	use crate::core::objects::material::ALL_MATERIALS;
	use crate::core::objects::Hit;
	use crate::core::types::{Point, Ray, Vec3};

	use super::{reflect_dir, refract_dir};

	#[test]
	fn bouncing_ray_always_originates_at_hit_point() {
		// This is the incoming ray:
		let ray_in = Ray::new(Point::origin(), Vec3::new(1, 0, 0));
		// This is the hit point and normal:
		let point = Point::new(5, 0, 0);
		let normal = Vec3::new(-1, 0, 0);

		// For every material, if the ray is scattered, the bouncing one should originate at the hit point:
		let mut violations = vec![];
		for mat in ALL_MATERIALS {
			let hit = Hit { t: 5.0, point, normal, is_front_face: true, material: *mat };
			let Some(ray_out) = mat.scatter(ray_in, hit) else {
				continue;
			};
			if ray_out.origin != point {
				violations.push((mat, ray_out.origin));
			}
		}

		// No violations should occur:
		let details = violations.iter()
			.map(|(mat, pt)| format!("  - {:?}: originated at {:?};\n", mat, pt))
			.collect::<String>();
		assert!(
			violations.is_empty(),
			"scattered rays should originate off the hit point, but didn't for the following:\n{}",
			details
		);
	}

	#[test]
	fn reflected_ray_has_same_angle() {
		// This incoming ray hits the surface at an angle:
		let ray = Ray::new(Point::origin(), Vec3::new(1, -2, 0));
		// The normal points straight upwards (y-axis):
		let normal = Vec3::new(0, 1, 0);

		// The reflected ray should point at the same angle, but upwards:
		let expected = Vec3::new(1, 2, 0).unit();
		let actual = reflect_dir(ray.direction, normal).unit();
		assert_eq!(actual, expected)
	}

	#[test]
	fn refracted_ray_does_not_reverse_direction() {
		// This incoming ray hits the surface at an angle:
		let ray = Ray::new(Point::origin(), Vec3::new(1, -2, 0));
		// The normal points straight upwards (y-axis):
		let normal = Vec3::new(0, 1, 0);

		// The refracted ray should not 'bounce', but 'continue on', albeit at a different angle
		let ray_refracted: Vec3 = refract_dir(ray.direction, normal, 1.0 / 1.5);
		let is_same_direction = ray_refracted.dot(ray.direction) >= 0.0;
		assert!(is_same_direction, "refracted ray should continue on, but direction was reversed")
	}
}
