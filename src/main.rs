mod args;
mod demo;

use std::fs::File;
use std::io;

use args::Args;
use demo::AvailableDemo;
use raytracer::camera::{Camera, CameraSetup};
use raytracer::output;
use raytracer::types::ToVec3;

fn main() {
	let args = Args::parse();

	// Check if we can write at all and hold onto the handle
	let mut writer: Box<dyn io::Write> = if let Some(ref path) = args.output {
		let file = File::create(path).unwrap();
		Box::new(file)
	} else {
		Box::new(io::stdout())
	};

	let demo = args.demo.unwrap_or(AvailableDemo::Spheres).build();
	let demo_setup = demo.setup();

	let center = args.center.unwrap_or(demo_setup.lookfrom);
	let target = args.target.unwrap_or(demo_setup.lookat);
	let default_focus_distance = (center.to_vec3() - target.to_vec3()).norm();

	let setup = CameraSetup {
		width: args.width,
		height: args.height,
		v_fov: args.fov.unwrap_or(demo_setup.v_fov),
		lookfrom: center,
		lookat: target,
		defocus_angle: args.aperture.unwrap_or(demo_setup.defocus_angle),
		focus_distance: args.focus.unwrap_or(default_focus_distance),
		..demo_setup
	};
	let camera = Camera::from(setup)
		.anti_aliasing(args.samples)
		.bounces(args.bounces);
	let image = camera.render(demo.scene());

	output::ppm::raw(&image, args.gamma, &mut writer).unwrap();
}
