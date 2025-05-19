use raytracer::camera::CameraSetup;
use raytracer::objects::{Material, Sphere};
use raytracer::scene::Scene;
use raytracer::types::{Color, Point};

use super::Demo;

// Banner in the Github repository

pub fn build() -> Demo {
	Demo { scene: scene(), setup: setup() }
}

fn scene() -> Scene {
	let btm = Sphere::new(
		Point::new(0, -99.5, -19),
		100,
		Material::Matte { color: Color(0.5, 0.5, 0.5) }
	);
	let sph = Sphere::new(
		Point::origin(),
		1.0,
		Material::Dielectric { ridx: 1.5 }
	);

	let sphere_l = Sphere::new(
		Point::new(-1.01, 0.12, -2.3),
		1.0,
		Material::Matte { color: Color(0.24, 0.16, 0.37) }
	);
	let sphere_r = Sphere::new(
		Point::new(1.01, 0.12, -2.3),
		1.0,
		Material::Metal { color: Color(0.16, 0.37, 0.3), fuzz: 0.00 }
	);
	
	let sphere_l2 = Sphere::new(
		Point::new(-1.6, -0.8, 0.3),
		0.6,
		Material::Metal { color: Color(0.37, 0.32, 0.16), fuzz: 0.0 }
	);
	let sphere_r2 = Sphere::new(
		Point::new(1.6, -0.8, 0.3),
		0.6,
		Material::Metal { color: Color(0.16, 0.16, 0.37), fuzz: 0.95 },
	);

	let sphere_front = Sphere::new(
		Point::new(0, -1.05, 1.6),
		0.6,
		Material::Matte { color: Color(0.42, 0.19, 0.19) }
	);
	Scene::from([
		btm, sph, sphere_l, sphere_r, sphere_l2, sphere_r2, sphere_front
	])
}

fn setup() -> CameraSetup {
	CameraSetup {
		v_fov: 27.0,
		lookat: Point::new(0, -0.35, 0),
		lookfrom: Point::new(0, 0.35, 10),
		..Default::default()
	}
}
