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
	let demo_setup = demo.camera_setup();

	let setup = CameraSetup {
		width: args.width,
		height: args.height,
		v_fov: args.fov.unwrap_or(demo_setup.v_fov),
		lookfrom: args.center.unwrap_or(demo_setup.lookfrom),
		lookat: args.target.unwrap_or(demo_setup.lookat),
		defocus_angle: args.aperture.unwrap_or(demo_setup.defocus_angle),
		focus_distance: args.focus.unwrap_or(demo_setup.focus_distance),
		..demo_setup
	};
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
