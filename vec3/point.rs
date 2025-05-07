use std::fmt::Display;
use std::ops;

use super::Vec3;

pub type Point = Vec3;
pub enum Axis {
  X, Y, Z
}

// Indexes
impl ops::Index<Axis> for Point {
  type Output = f64;
  fn index(&self, index: Axis) -> &Self::Output {
    match index {
      Axis::X => &self.0,
      Axis::Y => &self.1,
      Axis::Z => &self.2,
    }
  }
}
impl ops::IndexMut<Axis> for Point {
  fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
    match index {
      Axis::X => &mut self.0,
      Axis::Y => &mut self.1,
      Axis::Z => &mut self.2,
    }
  }
}
