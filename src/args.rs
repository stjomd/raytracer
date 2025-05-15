use std::path::PathBuf;

use clap::error::ErrorKind;
use clap::{ArgAction, Error, Parser};

use crate::camera::CameraSetup;
use crate::types::Point;

const ABOUT: &str = "Creates ray traced images.";

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
	#[arg(short, long, default_value_t = CameraSetup::default().v_fov)]
	pub fov: f64,
	/// The camera center [format: 'x,y,z']
	#[arg(
		short,
		long,
		default_value_t = CameraSetup::default().lookfrom,
		value_parser = parse_point,
		help = format!("The camera center [format: 'x,y,z'] [default: '{}']", display_point(CameraSetup::default().lookfrom)),
		hide_default_value = true
	)]
	pub center: Point,

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

fn parse_point(arg: &str) -> Result<Point, Error> {
	let msg: &str = "\nformat for point type is 'x,y,z', where 'x', 'y', and 'z' are numeric\nexample: '1.0,-2.0,3'";
	let values: Vec<f64> = arg.split(",")
    .map(|val| val.parse::<f64>())
    .map(|res| res.map_err(|_| Error::raw(ErrorKind::ValueValidation, msg)))
		.collect::<Result<_, _>>()?;
	if values.len() != 3 {
		return Err(Error::raw(ErrorKind::ValueValidation, msg));
	}
	Ok(Point(values[0], values[1], values[2]))
}
fn display_point(point: Point) -> String {
	format!("{},{},{}", point.0, point.1, point.2)
}

#[cfg(test)]
mod tests {
  use crate::types::Point;

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
