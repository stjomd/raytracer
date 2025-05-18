mod helpers;

use std::path::PathBuf;

use clap::builder::styling::AnsiColor;
use clap::builder::Styles;
use helpers::{arg_desc, parse_point, UnquotedArgString};
use clap::{ArgAction, Parser};

use raytracer::camera::CameraSetup;
use raytracer::types::Point;

use crate::demo::AvailableDemo;

const ABOUT: &str = "Creates ray traced images.";

mod headings {
	pub const CAMERA: &str = "Camera";
	pub const INFO: &str = "Info";
	pub const INPUT: &str = "Input";
	pub const OUTPUT: &str = "Output";
	pub const RENDERING: &str = "Rendering";
}

#[derive(Parser)]
#[command(version, about = ABOUT, styles = help_style(), disable_help_flag = true, disable_version_flag = true)]
pub struct Args {

	/// The demo scene to be rendered
	#[arg(long, help_heading = headings::INPUT)]
	pub demo: Option<AvailableDemo>,

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
		default_value_t = Args::default().gamma,
		hide_default_value = true,
		help = arg_desc("Value used for gamma correction", None, Some(Args::default().gamma)),
		help_heading = headings::OUTPUT
	)]
	pub gamma: f64,

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
		default_value_t = Args::default().samples,
		hide_default_value = true,
		help = arg_desc("Samples per pixel (increase for SSAA)", None, Some(Args::default().samples)),
		help_heading = headings::RENDERING
	)]
	pub samples: u32,
	/// Max. amount of bounces per ray
	#[arg(
		short,
		long,
		default_value_t = Args::default().bounces,
		hide_default_value = true,
		help = arg_desc("Max. amount of bounces per ray", None, Some(Args::default().bounces)),
		help_heading = headings::RENDERING
	)]
	pub bounces: u32,

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
			demo: None,
			width: 0,
			height: 0,
			output: None,
			gamma: 2.2,
			center: Some(setup.lookfrom),
			target: Some(setup.lookat),
			aperture: Some(setup.defocus_angle),
			focus: Some(setup.lookfrom.distance(setup.lookat)),
			fov: Some(setup.v_fov),
			samples: 100,
			bounces: 10,
			help: None,
			version: None
		}
	}
}

/// Defines the color style of the help message.
fn help_style() -> Styles {
	Styles::styled()
		.header(AnsiColor::Green.on_default().bold().underline())
		.usage(AnsiColor::Green.on_default().bold().underline())
		.literal(AnsiColor::Cyan.on_default().bold())
		.placeholder(AnsiColor::Cyan.on_default())
}
