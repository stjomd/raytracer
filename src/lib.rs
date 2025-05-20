mod core;

// Public API

pub mod camera {
	pub use super::core::camera::{Camera, CameraSetup};
}

pub mod objects {
	pub use super::core::objects::{Material, Object, Sphere, ToObject};
}

pub mod output {
	pub use super::core::output::*;
}

pub mod types {
	pub use super::core::types::{Color, Point, ToVec3, Vec3};
}

pub mod scene {
	pub use super::core::scene::*;
}
