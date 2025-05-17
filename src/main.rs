mod args;
mod demo;

use std::fs::File;
use std::io::{stdout, Write};

use args::Args;
use demo::DemoScene;
use raytracer::camera::{Camera, CameraSetup};
use raytracer::output;

fn main() {
	let args = Args::parse();

	// Check if file can be created, and close
	if let Some(ref path) = args.output {
		File::create(path).unwrap();
	}

	let demo = DemoScene::Spheromania;

	// let focus_default = (args.center).distance(args.target);
	// let _setup = CameraSetup {
	// 	width: args.width,
	// 	height: args.height,
	// 	v_fov: args.fov,
	// 	lookfrom: args.center,
	// 	lookat: args.target,
	// 	defocus_angle: args.aperture,
	// 	focus_distance: args.focus.unwrap_or(focus_default),
	// 	..Default::default()
	// };
	let setup = CameraSetup { width: 1200, height: 500, defocus_angle: 0.6, focus_distance: 10.0, ..demo.camera_setup() };
	let camera = Camera::from(setup)
    .anti_aliasing(args.samples)
    .bounces(args.bounces);

	let image = camera.render(&demo.scene());

	let mut writer: Box<dyn Write> = if let Some(ref path) = args.output {
		let file = File::create(path).unwrap();
		Box::new(file)
	} else {
		Box::new(stdout())
	};
	output::ppm::write(&image, args.gamma, &mut writer).unwrap();
}
