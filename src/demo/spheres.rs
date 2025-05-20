use raytracer::camera::CameraSetup;
use raytracer::objects::{Material, Sphere};
use raytracer::scene::Scene;

use raytracer::types::{Color, Point};

use super::Demo;

// Built upon scene during the first book

pub fn build() -> Demo {
	Demo {
		scene: scene(),
		setup: setup(),
	}
}

fn scene() -> Scene {
	let sphere_bottom = Sphere::new(
		Point::new(0, -100.5, -1),
		100,
		Material::Matte {
			color: Color::new(0.8, 0.8, 0),
		},
	);
	let sphere_center = Sphere::new(
		Point::new(0, 0, -1.2),
		0.5,
		Material::Matte {
			color: Color::new(0, 0.2, 0.1),
		},
	);
	let sphere_left = Sphere::new(
		Point::new(-1, 0, -1),
		0.5,
		Material::Dielectric { ridx: 1.5 },
	);
	let sphere_left_air = Sphere::new(
		Point::new(-1, 0, -1),
		0.4,
		Material::Dielectric { ridx: 1.0 / 1.5 },
	);
	let sphere_right = Sphere::new(
		Point::new(1, 0, -1),
		0.5,
		Material::Metal {
			color: Color::new(0.8, 0.6, 0.2),
			fuzz: 0.0,
		},
	);
	Scene::from([
		sphere_bottom,
		sphere_center,
		sphere_left,
		sphere_left_air,
		sphere_right,
	])
}

fn setup() -> CameraSetup {
	CameraSetup {
		v_fov: 90.0,
		..Default::default()
	}
}
