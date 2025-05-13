#![allow(unused)]

use std::fmt::Display;

use super::vector::ToVec3;
use super::{Color, Point, Vec3};

/// A representation of a ray.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
  /// The starting point of a ray.
  pub origin: Point,
  /// The direction where the ray is pointing.
  pub direction: Vec3,
  /// A measure of how much luminance this ray keeps.
  pub attenuation: Color,
}

// Constructors
impl Ray {
  /// Creates a ray with full attenuation (factor of 1).
  pub fn new(origin: Point, direction: Vec3) -> Self {
    Ray { origin, direction, attenuation: Color::new(1, 1, 1) }
  }
  /// Creates a ray with a specified color/attenuation.
  pub fn newc(origin: Point, direction: Vec3, color: Color) -> Self {
    Ray { origin, direction, attenuation: color }
  }
}

// Operations
impl Ray {
  /// Calculates the point where a ray is located at a specific parameter `t`.
  /// This parameter can be (albeit misguidedly) understood as 'time'.
  pub fn at(&self, t: f64) -> Point {
    let point = self.origin.to_vec3() + self.direction.scale(t);
    point.into()
  }
}
