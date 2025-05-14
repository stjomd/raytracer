#![allow(unused)]

use std::fmt::Display;
use std::ops;

use crate::types::Interval;

use super::vec3::ToVec3;
use super::Vec3;

/// A vector that represents a color with its red, green, and blue values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub f64, pub f64, pub f64);

// Constructors
impl Color {
  /// Creates a new color vector from specified values of red, green, and blue channel.
  /// This constructor does not enforce any constraints on each of the values.
  pub fn new<R, G, B>(r: R, g: G, b: B) -> Self
  where R: Into<f64>, G: Into<f64>, B: Into<f64> {
    Self(r.into(), g.into(), b.into())
  }
  /// Creates a black color value, where each color channel has value zero.
  pub fn black() -> Self {
    Self(0.0, 0.0, 0.0)
  }
}

// Getters
impl Color {
  /// The value of the red channel.
  pub fn r(&self) -> f64 {
    self.0
  }
  /// The value of the green channel.
  pub fn g(&self) -> f64 {
    self.1
  }
  /// The value of the blue channel.
  pub fn b(&self) -> f64 {
    self.2
  }
}

// Transform between Color & Vec3
impl ToVec3 for Color {
  fn to_vec3(&self) -> Vec3 {
    Vec3(self.0, self.1, self.2)
  }
}
impl From<Vec3> for Color {
  fn from(value: Vec3) -> Self {
    Self(value.0, value.1, value.2)
  }
}
impl From<Color> for Vec3 {
  fn from(value: Color) -> Vec3 {
    Vec3(value.0, value.1, value.2)
  }
}

// Assignment operators
impl ops::AddAssign for Color {
  fn add_assign(&mut self, rhs: Self) {
    self.0 += rhs.0;
    self.1 += rhs.1;
    self.2 += rhs.2;
  }
}
