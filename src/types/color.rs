#![allow(unused)]

use std::fmt::Display;
use std::ops;

use super::Vec3;

/// A vector that represents a color with its red, green, and blue values.
#[derive(Debug, Clone, Copy)]
pub struct Color {
  vec: Vec3
}

// Constructors
impl Color {
  /// Creates a new color vector from specified values of red, green, and blue channel.
  /// This constructor does not enforce any constraints on each of the values.
  pub fn new<A, B, C>(r: A, g: B, b: C) -> Self
  where A: Into<f64>, B: Into<f64>, C: Into<f64> {
    Self { vec: Vec3::new(r, g, b) }
  }
}

// Getters
impl Color {
  /// The value of the red channel.
  pub fn red(&self) -> f64 {
    self.vec.0
  }
  /// The value of the green channel.
  pub fn green(&self) -> f64 {
    self.vec.1
  }
  /// The value of the blue channel.
  pub fn blue(&self) -> f64 {
    self.vec.2
  }
}

// Display
impl Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let (r, g, b) = self.vec.scale(255.9999).to_tuple(|x| x as u8);
    write!(f, "{} {} {}", r, g, b)
  }
}

// Transform between Color & Vec3
impl From<Vec3> for Color {
  fn from(value: Vec3) -> Self {
    Self { vec: value }
  }
}
impl From<Color> for Vec3 {
  fn from(value: Color) -> Self {
    value.vec
  }
}

// Dereference as Vec3
// This lets one use methods from Vec3 on Color, for example `Color::new(0.0, 0.5, 1.0).norm()`,
// as well as indexing `Color::new(0.0, 0.5, 1.0)[1]`.
impl ops::Deref for Color {
  type Target = Vec3;
  fn deref(&self) -> &Self::Target {
    &self.vec
  }
}
impl ops::DerefMut for Color {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.vec
  }
}
