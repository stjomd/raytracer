mod helpers;

use std::path::PathBuf;

use helpers::{arg_desc, help_style, parse_point, UnquotedArgString};
use clap::{ArgAction, Parser};

use raytracer::camera::CameraSetup;
use raytracer::types::Point;

const ABOUT: &str = "Creates ray traced images.";

mod headings {
	pub const CAMERA: &str = "Camera";
	pub const INFO: &str = "Info";
	pub const OUTPUT: &str = "Output";
	pub const RENDERING: &str = "Rendering";
}

#[derive(Parser)]
#[command(version, about = ABOUT, styles = help_style(), disable_help_flag = true, disable_version_flag = true)]
pub struct Args {

	/// Width of the image in pixels
	#[arg(short, long, help_heading = headings::OUTPUT)]
	pub width: usize,
	/// Height of the image in pixels
	#[arg(short, long, help_heading = headings::OUTPUT)]
	pub height: usize,
	/// Path to the output file
	#[arg(
		short,
		long,
		help = arg_desc("Path to the output file", None, Some(UnquotedArgString("stdout"))),
		help_heading = headings::OUTPUT
	)]
	pub output: Option<PathBuf>,
	/// Value used for gamma correction
	#[arg(
		short,
		long,
		help = arg_desc("Value used for gamma correction", None, Args::default().gamma),
		help_heading = headings::OUTPUT
	)]
	pub gamma: Option<f64>,

	/// Camera center
	#[arg(
		short,
		long,
		value_parser = parse_point,
		help = arg_desc("Camera center", Some("x,y,z"), Args::default().center),
		help_heading = headings::CAMERA
	)]
	pub center: Option<Point>,
	/// Point the camera is looking at
	#[arg(
		short,
		long,
		value_parser = parse_point,
		help = arg_desc("Point the camera is looking at", Some("x,y,z"), Args::default().target),
		help_heading = headings::CAMERA
	)]
	pub target: Option<Point>,
	/// Angular aperture size, in degrees
	#[arg(
		short,
		long,
		help = arg_desc("Angular aperture size, in degrees (blur amount)", None, Args::default().aperture),
		help_heading = headings::CAMERA
	)]
	pub aperture: Option<f64>,
	/// Distance between camera center and object in focus
	#[arg(
		short,
		long,
		help = arg_desc(
			"Distance between camera center and object in focus",
			None,
			Some(UnquotedArgString("distance from center to target"))
		),
		help_heading = headings::CAMERA
	)]
	pub focus: Option<f64>,
	/// Vertical field of view, in degrees
	#[arg(
		long,
		help = arg_desc("Vertical field of view, in degrees", None, Args::default().fov),
		help_heading = headings::CAMERA
	)]
	pub fov: Option<f64>,

	/// Samples per pixel
	#[arg(
		short,
		long,
		help = arg_desc("Samples per pixel (increase for SSAA)", None, Args::default().samples),
		help_heading = headings::RENDERING
	)]
	pub samples: Option<u32>,
	/// Max. amount of bounces per ray
	#[arg(
		short,
		long,
		help = arg_desc("Max. amount of bounces per ray", None, Args::default().bounces),
		help_heading = headings::RENDERING
	)]
	pub bounces: Option<u32>,

	/// Print help message and exit
	#[arg(short = 'H', long, action = ArgAction::Help, help_heading = headings::INFO)]
	pub help: Option<bool>,
	/// Print version and exit
	#[arg(short = 'V', long, action = ArgAction::Version, help_heading = headings::INFO)]
	pub version: Option<bool>,

}

impl Args {
	/// Parses CLI arguments.
	pub fn parse() -> Self {
		<Self as Parser>::parse()
	}
}

impl Default for Args {
	fn default() -> Self {
		let setup = CameraSetup::default();
		Self {
			width: 0,
			height: 0,
			output: None,
			gamma: Some(2.2),
			center: Some(setup.lookfrom),
			target: Some(setup.lookat),
			aperture: Some(setup.defocus_angle),
			focus: Some(setup.lookfrom.distance(setup.lookat)),
			fov: Some(setup.v_fov),
			samples: Some(100),
			bounces: Some(10),
			help: None,
			version: None
		}
	}
}
