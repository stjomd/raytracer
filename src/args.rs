use std::path::PathBuf;

use clap::builder::styling::AnsiColor;
use clap::builder::Styles;
use clap::error::ErrorKind;
use clap::{ArgAction, Error, Parser};

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
	/// Path to the output file (if empty, outputs to stdout)
	#[arg(short, long, help_heading = headings::OUTPUT)]
	pub output: Option<PathBuf>,
	/// Value used for gamma correction
	#[arg(short, long, default_value_t = 2.2, help_heading = headings::OUTPUT)]
	pub gamma: f64,

	/// Camera center [format: 'x,y,z']
	#[arg(
		short,
		long,
		default_value_t = CameraSetup::default().lookfrom,
		value_parser = parse_point,
		help = format!(
			"Camera center [format: 'x,y,z', default: '{}']",
			display_point(CameraSetup::default().lookfrom)
		),
		hide_default_value = true,
		help_heading = headings::CAMERA
	)]
	pub center: Point,
	/// Point the camera is looking at [format: 'x,y,z']
	#[arg(
		short,
		long,
		default_value_t = CameraSetup::default().lookat,
		value_parser = parse_point,
		help = format!(
			"Point the camera is looking at [format: 'x,y,z', default: '{}']",
			display_point(CameraSetup::default().lookat)
		),
		hide_default_value = true,
		help_heading = headings::CAMERA
	)]
	pub target: Point,
	/// Angular aperture size, in degrees (controls amount of blur)
	#[arg(short, long, default_value_t = CameraSetup::default().defocus_angle, help_heading = headings::CAMERA)]
	pub aperture: f64,
	/// Distance between camera center and object in focus
	#[arg(
		short,
		long,
		help = "Distance between camera center and object in focus [default: distance from center to target]",
		hide_default_value = true,
		help_heading = headings::CAMERA
	)]
	pub focus: Option<f64>,
	/// Vertical field of view, in degrees
	#[arg(long, default_value_t = CameraSetup::default().v_fov, help_heading = headings::CAMERA)]
	pub fov: f64,

	/// Samples per pixel (increase for supersampling anti-aliasing)
	#[arg(short, long, default_value_t = 100, help_heading = headings::RENDERING)]
	pub samples: u32,
	/// Max. amount of bounces per ray
	#[arg(short, long, default_value_t = 10, help_heading = headings::RENDERING)]
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

/// Defines the color style of the help message.
fn help_style() -> Styles {
	Styles::styled()
		.header(AnsiColor::Green.on_default().bold().underline())
		.usage(AnsiColor::Green.on_default().bold().underline())
		.literal(AnsiColor::Cyan.on_default().bold())
		.placeholder(AnsiColor::Cyan.on_default())
}

/// Parses a string argument into a [`Point`].
fn parse_point(arg: &str) -> Result<Point, Error> {
	let msg: &str = "format for point type is 'x,y,z', where 'x', 'y', and 'z' are numeric
example: '1.0,-2.0,3'\n
hint: try specifying the value like this: '--option=-1.5,2.0,3'";
	arg.parse::<Point>()
    .map_err(|e| Error::raw(ErrorKind::ValueValidation, format!("{}\n{}", e, msg)))
}
/// Converts a [`Point`] to a string representation.
fn display_point(point: Point) -> String {
	format!("[{},{},{}]", point.0, point.1, point.2)
}

#[cfg(test)]
mod tests {
  use raytracer::types::Point;

	use super::parse_point;

	#[test]
	fn should_parse_point_with_given_coordinates() {
		let point = parse_point("-1.0,-2,3.0");
		assert!(point.is_ok(), "point should be parsed, but error was returned");
		let point = point.unwrap();
		assert_eq!(point, Point(-1.0, -2.0, 3.0), "coordinates should be equal to arg");
	}

	#[test]
	fn if_point_arg_has_less_coordinates_then_error() {
		let point = parse_point("-1.0,2");
		assert!(point.is_err(), "arg has 2 coordinates, but point was parsed");
	}

	#[test]
	fn if_point_arg_has_more_coordinates_then_error() {
		let point = parse_point("-1.0,2,3.0,-4");
		assert!(point.is_err(), "arg has 4 coordinates, but point was parsed");
	}
}
