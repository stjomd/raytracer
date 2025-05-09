#![allow(unused)]

use std::fmt::Display;

use super::{Point, Vec3};

/// A representation of a ray.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
  pub origin: Point,
  pub direction: Vec3
}

// Constructors
impl Ray {
  pub fn new(origin: Point, direction: Vec3) -> Self {
    Ray { origin, direction }
  }
}

// Operations
impl Ray {
  pub fn at(&self, t: f64) -> Point {
    let point = *self.origin + self.direction.scale(t);
    point.into()
  }
}
