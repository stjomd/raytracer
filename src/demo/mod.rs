use raytracer::camera::CameraSetup;
use raytracer::objects::{Material, Sphere, ToObject};
use raytracer::scene::Scene;
use raytracer::types::{Color, Point, ToVec3, Vec3};

#[allow(unused)]
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum DemoScene {
	/// A hollow glass sphere, a matte sphere, and a metal sphere next to each other,
	/// with a matte bottom sphere below.
	Spheres,
	/// Three big spheres of different materials among many smaller spheres.
	Spheromania,
}
impl DemoScene {
	pub fn scene(&self) -> Scene {
		match self {
			Self::Spheres => spheres(),
			Self::Spheromania => spheromania(),
		}
	}
	pub fn camera_setup(&self) -> CameraSetup {
		match self {
			Self::Spheres => CameraSetup {
				v_fov: 90.0,
				..Default::default()
			},
			Self::Spheromania => CameraSetup {
				lookfrom: Point::new(13, 2, 3),
				lookat: Point::new(0, 0, 0),
				v_fov: 20.0,
				..Default::default()
			}
		}
	}
}

// Built upon scene during the book
fn spheres() -> Scene {
	let sphere_bottom = Sphere::new(
		Point::new(0, -100.5, -1),
		100,
		Material::Matte { color: Color::new(0.8, 0.8, 0) }
	);
	let sphere_center = Sphere::new(
		Point::new(0, 0, -1.2),
		0.5,
		Material::Matte { color: Color::new(0, 0.2, 0.1) }
	);
	let sphere_left = Sphere::new(
		Point::new(-1, 0, -1),
		0.5,
		Material::Dielectric { ridx: 1.5 }
	);
	let sphere_left_air = Sphere::new(
		Point::new(-1, 0, -1),
		0.4,
		Material::Dielectric { ridx: 1.0 / 1.5 }
	);
	let sphere_right = Sphere::new(
		Point::new(1, 0, -1),
		0.5,
		Material::Metal { color: Color::new(0.8, 0.6, 0.2), fuzz: 0.0 }
	);
	Scene::from([sphere_bottom.obj(), sphere_center.obj(), sphere_left.obj(), sphere_left_air.obj(), sphere_right.obj()])
}

// Final scene from the first book.
fn spheromania() -> Scene {
	let random = |a: f64, b: f64| rand::random_range(a .. b);
	let random_unit = || random(0.0, 1.0);

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

				scene.add(sphere.obj());
			}
		}
	}

	let big1 = Sphere::new(
		Point::new(0, 1, 0),
		1.0,
		Material::Dielectric { ridx: 1.5 }
	);
	scene.add(big1.obj());

	let big2 = Sphere::new(
		Point::new(-4, 1, 0),
		1.0,
		Material::Matte { color: Color(0.4, 0.2, 0.1) }
	);
	scene.add(big2.obj());

	let big3 = Sphere::new(
		Point::new(4, 1, 0),
		1.0,
		Material::Metal { color: Color(0.7, 0.6, 0.5), fuzz: 0.0 }
	);
	scene.add(big3.obj());

	scene
}
