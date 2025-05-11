#![allow(unused_imports)]

mod vector;
mod ray;
mod config;
mod interval;

pub use vector::{Vec3, ToVec3, Color, Point};
pub use ray::Ray;
pub use config::Config;
pub use interval::Interval;
