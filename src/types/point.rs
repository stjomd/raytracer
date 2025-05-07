#![allow(unused)]

use std::fmt::Display;
use std::ops;

use super::Vec3;

/// A representation of a point in 3D space.
#[derive(Debug, Clone, Copy)]
pub struct Point {
  vec: Vec3
}

// Constructors
impl Point {
  /// Creates a new point with the corresponding coordinates.
  pub fn new<A, B, C>(x: A, y: B, z: C) -> Self
  where A: Into<f64>, B: Into<f64>, C: Into<f64> {
    Self { vec: Vec3::new(x, y, z) }
  }
}

// Getters
impl Point {
  /// The coordinate on the X axis.
  pub fn x(&self) -> f64 {
    self.vec.0
  }
  /// The coordinate on the Y axis.
  pub fn y(&self) -> f64 {
    self.vec.1
  }
  /// The coordinate on the Z axis.
  pub fn z(&self) -> f64 {
    self.vec.2
  }
}

// Display
impl Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.vec.fmt(f)
  }
}

// Transform between Point & Vec3
impl From<Vec3> for Point {
  fn from(value: Vec3) -> Self {
    Self { vec: value }
  }
}
impl From<Point> for Vec3 {
  fn from(value: Point) -> Self {
    value.vec
  }
}

// Dereference as Vec3
impl ops::Deref for Point {
  type Target = Vec3;
  fn deref(&self) -> &Self::Target {
    &self.vec
  }
}
impl ops::DerefMut for Point {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.vec
  }
}
