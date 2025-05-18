mod spheres;
mod spheromania;

use raytracer::camera::CameraSetup;
use raytracer::scene::Scene;

#[allow(unused)]
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum AvailableDemo {
	/// A hollow glass sphere, a matte sphere, and a metal sphere next to each other,
	/// with a matte bottom sphere below.
	Spheres,
	/// Three big spheres of different materials among many smaller spheres.
	Spheromania,
}
impl AvailableDemo {
	pub fn build(&self) -> Demo {
		match self {
			AvailableDemo::Spheres => spheres::build(),
			AvailableDemo::Spheromania => spheromania::build(),
		}
	}
}

pub struct Demo {
	scene: Scene,
	setup: CameraSetup,
}

impl Demo {
	pub fn scene(&self) -> &Scene {
		&self.scene
	}
	pub fn setup(&self) -> CameraSetup {
		self.setup
	}
}
