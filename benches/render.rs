use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracer::camera::{Camera, CameraSetup};
use raytracer::objects::{Material, Sphere, ToObject};
use raytracer::scene::Scene;
use raytracer::types::{Color, Point, Vec3};

pub fn criterion_benchmark(c: &mut Criterion) {
	let camera = camera();
	let scene = scene();
	c.bench_function(
		"spheres",
		|b| b.iter(|| {
			camera.render(black_box(&scene));
		})
	);
}

fn camera() -> Camera {
	let lookfrom = Point(0.0, 0.0, 0.0);
	let lookat = Point(0.0, 0.0, -1.0);
	let setup = CameraSetup {
		width: 50,
		height: 50,
		v_fov: 90.0,
		lookfrom,
		lookat,
		view_up: Vec3(0.0, 1.0, 0.0),
		defocus_angle: 0.0,
		focus_distance: lookfrom.distance(lookat)
	};
	Camera::from(setup)
    .anti_aliasing(100)
    .bounces(50)
}

fn scene() -> Scene {
	let center_outer = Sphere::new(
		Point::new(0, 0, -1),
		0.5,
		Material::Dielectric { ridx: 1.5 }
	);
	let center_inner = Sphere::new(
		Point::new(0, 0, -1),
		0.4,
		Material::Dielectric { ridx: 1.0 / 1.5 }
	);
	let left = Sphere::new(
		Point::new(-1.0, 0, -1),
		0.5,
		Material::Metal { color: Color(1.0, 0.0, 0.0), fuzz: 0.0 }
	);
	let right = Sphere::new(
		Point::new(1.0, 0, -1),
		0.5,
		Material::Metal { color: Color(0.0, 1.0, 0.0), fuzz: 0.0 }
	);
	let bg1 = Sphere::new(
		Point::new(0, -5, -5),
		5,
		Material::Matte { color: Color(0.8, 0.6, 0.2) }
	);
	let bg2 = Sphere::new(
		Point::new(0, 5, -5),
		5,
		Material::Metal { color: Color(0.1, 0.3, 0.1), fuzz: 0.0 }
	);
	Scene::from([center_outer.obj(), center_inner.obj(), left.obj(), right.obj(), bg1.obj(), bg2.obj()])
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
