#![allow(unused)]

mod point;
mod color;

use std::fmt::Display;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub use point::{Point, Axis};
pub use color::Color;

// Constructors
impl Vec3 {
  pub fn new<A, B, C>(a: A, b: B, c: C) -> Self
  where A: Into<f64>, B: Into<f64>, C: Into<f64> {
    Vec3(a.into(), b.into(), c.into())
  }
  pub fn zero() -> Self {
    Vec3(0.0, 0.0, 0.0)
  }
}

// Display
impl Display for Vec3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {} {}", self.0, self.1, self.2)
  }
}

// Indexes
impl ops::Index<usize> for Vec3 {
  type Output = f64;
  fn index(&self, index: usize) -> &Self::Output {
    match index {
      0 => &self.0,
      1 => &self.1,
      2 => &self.2,
      _ => panic!("index out of bounds {}", index)
    }
  }
}
impl ops::IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    match index {
      0 => &mut self.0,
      1 => &mut self.1,
      2 => &mut self.2,
      _ => panic!("index out of bounds {}", index)
    }
  }
}

// Operators
impl ops::Neg for Vec3 {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Vec3(-self.0, -self.1, -self.2)
  }
}
impl ops::Add for Vec3 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
  }
}
impl ops::Sub for Vec3 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
  }
}
impl ops::Mul for Vec3 {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
  }
}
impl ops::Div for Vec3 {
  type Output = Self;
  fn div(self, rhs: Self) -> Self::Output {
    Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
  }
}

// Assignment operators
impl ops::AddAssign for Vec3 {
  fn add_assign(&mut self, rhs: Self) {
    self.0 += rhs.0;
    self.1 += rhs.1;
    self.2 += rhs.2;
  }
}
impl ops::SubAssign for Vec3 {
  fn sub_assign(&mut self, rhs: Self) {
    self.0 -= rhs.0;
    self.1 -= rhs.1;
    self.2 -= rhs.2;
  }
}
impl ops::MulAssign for Vec3 {
  fn mul_assign(&mut self, rhs: Self) {
    self.0 *= rhs.0;
    self.1 *= rhs.1;
    self.2 *= rhs.2;
  }
}
impl ops::DivAssign for Vec3 {
  fn div_assign(&mut self, rhs: Self) {
    self.0 /= rhs.0;
    self.1 /= rhs.1;
    self.2 /= rhs.2;
  }
}

// Properties
impl Vec3 {
  pub fn norm_sq(&self) -> f64 {
    self.0*self.0 + self.1*self.1 + self.2*self.2
  }
  pub fn norm(&self) -> f64 {
    self.norm_sq().sqrt()
  }
}

// Operations
impl Vec3 {
  pub fn scale<T: Into<f64>>(self, f: T) -> Self {
    let factor = f.into();
    Vec3(factor*self.0, factor*self.1, factor*self.2)
  }
  pub fn dot(self, rhs: Self) -> f64 {
    self.0*rhs.0 + self.1*rhs.1 + self.2*rhs.2
  }
  pub fn cross(self, rhs: Self) -> Self {
    Vec3(
      self.1*rhs.2 - self.2*rhs.1,
      self.2*rhs.0 - self.0*rhs.2,
      self.0*rhs.1 - self.1*rhs.0
    )
  }
  pub fn unit(self) -> Self {
    self.scale(1.0 / self.norm())
  }
}
