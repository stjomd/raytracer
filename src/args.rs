use std::path::PathBuf;

use clap::{ArgAction, Parser};

use crate::camera::CameraSetup;

const ABOUT: &str = "Creates ray traced images.";
const DEFAULT_CAMERA_SETUP: CameraSetup = CameraSetup::const_default();

#[derive(Parser)]
#[command(version, about = ABOUT, disable_help_flag = true)]
pub struct Args {

	/// The width in pixels
	#[arg(short, long)]
	pub width: usize,
	/// The height in pixels
	#[arg(short, long)]
	pub height: usize,

	/// The path to the output file (if empty, outputs to stdout)
	#[arg(short, long)]
	pub output: Option<PathBuf>,

	/// Samples per pixel (increase for supersampling anti-aliasing)
	#[arg(short, long, default_value_t = 25)]
	pub samples: u32,
	/// Max. amount of times a ray can bounce until a color is determined
	#[arg(short, long, default_value_t = 2)]
	pub bounces: u32,

	/// Vertical field of view, in degrees
	#[arg(short, long, default_value_t = DEFAULT_CAMERA_SETUP.v_fov)]
	pub fov: f64,

	/// The value used for gamma correction
	#[arg(short, long, default_value_t = 2.2)]
	pub gamma: f64,

	/// Print help message
	#[arg(short = 'H', long, action = ArgAction::Help)]
	pub help: Option<bool>,

}

impl Args {
	/// Parses CLI arguments.
	pub fn parse() -> Self {
		<Self as Parser>::parse()
	}
}
