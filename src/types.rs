#![allow(unused_imports)]

mod vector;
mod ray;
mod config;

pub use vector::{Vec3, ToVec3, Color, Point};
pub use ray::Ray;
pub use config::Config;
