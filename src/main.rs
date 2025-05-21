mod args;
mod demo;

use std::fs::File;
use std::io;

use args::Args;
use raytracer::camera::{Camera, CameraSetup};
use raytracer::input::RaytracerInput;
use raytracer::output;
use raytracer::scene::Scene;
use raytracer::types::ToVec3;

fn main() {
	let args = Args::parse();

	let json = File::open(&args.input).unwrap();
	let input = RaytracerInput::try_from(json).unwrap();

	// Check if we can write at all and hold onto the handle
	let mut writer: Box<dyn io::Write> = if let Some(ref path) = args.output {
		let file = File::create(path).unwrap();
		Box::new(file)
	} else {
		Box::new(io::stdout())
	};

	let (setup, scene) = prepare(&args, input);

	let camera = Camera::from(setup)
		.anti_aliasing(args.samples)
		.bounces(args.bounces);
	let image = camera.render(&scene);

	output::ppm::raw(&image, args.gamma, &mut writer).unwrap();
}

fn prepare(args: &Args, input: RaytracerInput) -> (CameraSetup, Scene) {
	let center = args.center.unwrap_or(input.camera.source);
	let target = args.target.unwrap_or(input.camera.target);
	let default_focus_distance = (center.to_vec3() - target.to_vec3()).norm();

	let setup = CameraSetup {
		width: args.width,
		height: args.height,
		v_fov: args.fov.unwrap_or(input.camera.fov),
		lookfrom: center,
		lookat: target,
		defocus_angle: args.aperture.unwrap_or(input.camera.aperture),
		focus_distance: args.focus.unwrap_or(default_focus_distance),
		..Default::default()
	};
	let scene = Scene::from_objs(input.scene);

	(setup, scene)
}
