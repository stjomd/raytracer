#![allow(unused)]

use std::fmt::Display;
use std::ops;
use std::str::FromStr;

use super::vec3::ToVec3;
use super::Vec3;

/// A representation of a point in 3D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(pub f64, pub f64, pub f64);

// Constructors
impl Point {
	/// Creates a new point with the corresponding coordinates.
	pub fn new<X, Y, Z>(x: X, y: Y, z: Z) -> Self
	where X: Into<f64>, Y: Into<f64>, Z: Into<f64> {
		Self(x.into(), y.into(), z.into())
	}
	/// Creates a point at origin, that is, where each coordinate is zero.
	pub const fn origin() -> Self {
		Self(0.0, 0.0, 0.0)
	}
}

// Getters
impl Point {
	/// The coordinate on the X axis.
	pub fn x(&self) -> f64 {
		self.0
	}
	/// The coordinate on the Y axis.
	pub fn y(&self) -> f64 {
		self.1
	}
	/// The coordinate on the Z axis.
	pub fn z(&self) -> f64 {
		self.2
	}
}

// Display
impl Display for Point {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{} {} {}]", self.0, self.1, self.2)
	}
}
impl FromStr for Point {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let trimmed = s.trim().replace("[", "").replace("]", "");
		let values: Vec<f64> = trimmed.split(&[' ', ','])
			.map(|val| {
				val.parse::<f64>().map_err(|e| format!("{} '{}'", e, val))
			})
			.collect::<Result<_, _>>()?;
		if values.len() != 3 {
			return Err(format!("expected 3 coordinates, got {}", values.len()));
		}
		Ok(Self(values[0], values[1], values[2]))
	}
}

// Transform between Point & Vec3
impl ToVec3 for Point {
	fn to_vec3(&self) -> Vec3 {
		Vec3(self.0, self.1, self.2)
	}
}
impl From<Vec3> for Point {
	fn from(value: Vec3) -> Self {
		Self(value.0, value.1, value.2)
	}
}
impl From<Point> for Vec3 {
	fn from(value: Point) -> Self {
		Vec3(value.0, value.1, value.2)
	}
}
