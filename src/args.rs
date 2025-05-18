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
		let cam_defl = CameraSetup::default();
		Self {
			width: 0,
			height: 0,
			output: None,
			gamma: Some(2.2),
			center: Some(cam_defl.lookfrom),
			target: Some(cam_defl.lookat),
			aperture: Some(0.0),
			focus: Some(cam_defl.lookfrom.distance(cam_defl.lookat)),
			fov: Some(45.0),
			samples: Some(100),
			bounces: Some(10),
			help: None,
			version: None
		}
	}
}

struct UnquotedArgString(&'static str);
impl ToArgString for UnquotedArgString {
	fn to_arg_str(&self) -> String {
		self.0.to_string()
	}
}

trait ToArgString {
	fn to_arg_str(&self) -> String;
}
impl ToArgString for u32 {
	fn to_arg_str(&self) -> String {
		self.to_string()
	}
}
impl ToArgString for f64 {
	fn to_arg_str(&self) -> String {
		format!("{:.1}", self)
	}
}
impl ToArgString for &str {
	fn to_arg_str(&self) -> String {
		format!("'{}'", self)
	}
}
impl ToArgString for Point {
	fn to_arg_str(&self) -> String {
		format!("'{},{},{}'", self.0, self.1, self.2)
	}
}

/// Builds a description message for an option.
fn arg_desc<T: ToArgString>(desc: &str, format: Option<&str>, default: Option<T>) -> String {
	let formatted_hints = [
		format.map(|str| format!("format: '{}'", str)),
		default.map(|str| format!("default: {}", str.to_arg_str())),
	];
	let present_hints = formatted_hints.iter()
		.filter_map(|x| x.clone())
		.collect::<Vec<_>>();
	format!("{} [{}]", desc, present_hints.join(", "))
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

#[cfg(test)]
mod tests {
  use raytracer::types::Point;

	use super::{arg_desc, parse_point};

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

	#[test]
	fn if_format_and_default_then_description_has_both() {
		let format = Some("a,b,c");
		let default = Some(2.5);

		let desc = arg_desc("my param", format, default);
		assert_eq!(desc, "my param [format: 'a,b,c', default: 2.5]");
	}

	#[test]
	fn if_format_but_no_default_then_description_only_has_format() {
		let format = Some("a,b,c");
		let default: Option<&str> = None;

		let desc = arg_desc("my param", format, default);
		assert_eq!(desc, "my param [format: 'a,b,c']");
	}
}
