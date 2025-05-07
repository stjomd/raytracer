#![allow(unused)]

use std::fmt::Display;
use std::ops;

use super::{Point, Vec3};

/// A representation of a ray.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
  pub origin: Point,
  pub direction: Vec3
}

// Constructors
impl Ray {
  fn new(origin: Point, direction: Vec3) -> Self {
    Ray { origin, direction: direction.unit() }
  }
}

// Operations
impl Ray {
  fn at(&self, t: f64) -> Point {
    let point: Vec3 = *self.origin + self.direction.scale(t);
    point.into()
  }
}
