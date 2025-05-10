#![allow(unused)]

use std::ops::Range;

use crate::types::{Point, Ray, Vec3};

/// Represents an object hittable/intersectable by a ray.
pub trait Hittable {
  /// Calculates the intersection point of the specified ray with this object.
  /// Additionally, validates if the parameter `t` lies within the specified (plausible) range.
  /// If `t` lies outside the range, returns [`None`]; otherwise a [`Hit`] object.
  fn hit<F: Into<f64>>(&self, ray: Ray, t_range: Range<F>) -> Option<Hit>;
}

/// Represents an intersection between a ray and an object in the scene.
pub struct Hit {
  /// The time parameter along the the ray vector axis.
  pub t: f64,
  /// The intersection point.
  pub point: Point,
  /// The normal vector at the intersection point.
  pub normal: Vec3
}
