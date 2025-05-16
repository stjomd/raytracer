mod args;
mod types;
mod objects;
mod camera;
mod scene;
mod output;

use std::fs::File;
use std::io::{stdout, Write};

use args::Args;
use camera::{Camera, CameraSetup};
use objects::{Material, Sphere};
use types::{Color, Point};
use scene::{scene, Scene};

fn main() {
	let args = Args::parse();

	// Check if file can be created, and close
	if let Some(ref path) = args.output {
		File::create(path).unwrap();
	}

	let focus_default = (args.center).distance(args.target);
	let setup = CameraSetup {
		width: args.width,
		height: args.height,
		v_fov: args.fov,
		lookfrom: args.center,
		lookat: args.target,
		defocus_angle: args.aperture,
		focus_distance: args.focus.unwrap_or(focus_default),
		..Default::default()
	};
	let camera = Camera::from(setup)
    .anti_aliasing(args.samples)
    .bounces(args.bounces);

	let scene = scene();
	let image = camera.render(&scene);

	let mut writer: Box<dyn Write> = if let Some(ref path) = args.output {
		let file = File::create(path).unwrap();
		Box::new(file)
	} else {
		Box::new(stdout())
	};
	output::ppm::write(&image, args.gamma, &mut writer).unwrap();
}

fn scene() -> Scene {
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
	scene!(sphere_bottom, sphere_center, sphere_left, sphere_left_air, sphere_right)
}
