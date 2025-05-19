use raytracer::camera::CameraSetup;
use raytracer::objects::{Material, Sphere, ToObject};

use raytracer::scene::Scene;
use raytracer::types::{Color, Point, ToVec3, Vec3};

use super::Demo;

// Final scene of the first book

pub fn build() -> Demo {
	Demo { scene: scene(), setup: setup() }
}

fn random(a: f64, b: f64) -> f64 {
	rand::random_range(a .. b)
}
fn random_unit() -> f64 {
	random(0.0, 1.0)
}

fn scene() -> Scene {
	let ground = Sphere::new(
		Point::new(0, -1000, 0),
		1000,
		Material::Matte { color: Color::new(0.5, 0.5, 0.5) }
	).obj();

	let mut scene = Scene::from([ground]);
	for a in -11..11 {
		for b in -11..11 {
			let (a, b) = (a as f64, b as f64);
			let offset = random_unit();
			let center = Point::new(a + 0.9*random_unit(), 0.2, b + 0.9*random_unit());

			if (center.to_vec3() - Vec3::new(4, 0.2, 0)).norm() > 0.9 {
				let sphere;
				if offset < 0.7 {
					let color = Color(random_unit(), random_unit(), random_unit());
					let material = Material::Matte { color };
					sphere = Sphere::new(center, 0.2, material);
				} else if offset < 0.9 {
					let color = Color(random(0.5, 1.0), random(0.5, 1.0), random(0.5, 1.0));
					let fuzz = random(0.0, 0.5);
					let material = Material::Metal { color, fuzz };
					sphere = Sphere::new(center, 0.2, material);
				} else {
					let material = Material::Dielectric { ridx: 1.5 };
					sphere = Sphere::new(center, 0.2, material);
				}

				scene.add(sphere);
			}
		}
	}

	let big1 = Sphere::new(
		Point::new(0, 1, 0),
		1.0,
		Material::Dielectric { ridx: 1.5 }
	);
	scene.add(big1);

	let big2 = Sphere::new(
		Point::new(-4, 1, 0),
		1.0,
		Material::Matte { color: Color(0.4, 0.2, 0.1) }
	);
	scene.add(big2);

	let big3 = Sphere::new(
		Point::new(4, 1, 0),
		1.0,
		Material::Metal { color: Color(0.7, 0.6, 0.5), fuzz: 0.0 }
	);
	scene.add(big3);

	scene
}

fn setup() -> CameraSetup {
	CameraSetup {
		lookfrom: Point::new(13, 2, 3),
		lookat: Point::new(0, 0, 0),
		v_fov: 20.0,
		..Default::default()
	}
}
