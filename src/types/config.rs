#![allow(unused)]

use super::{Point, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Config {
  // image
  pub aspect_ratio: f64,
  pub img_size: (u64, u64),
  // camera
  pub focal_length: f64,
  pub camera_center: Point,
  pub viewport_size: (f64, f64),
  // viewport edge vectors
  pub vp_u: Vec3,
  pub vp_v: Vec3,
  // delta vectors between pixels
  pub px_d_u: Vec3,
  pub px_d_v: Vec3,
  // upper left point (viewport & pixel)
  pub vp_00: Point,
  pub px_00: Point,
}

impl Config {
  pub fn new(aspect_ratio: f64, width: u64) -> Self {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let (width, height) = Self::image_dimensions(aspect_ratio, 400);
    // Camera
    let focal_length = 1.0;
    let camera_center = Point::origin();
    let (vp_width, vp_height) = Self::viewport_dimensions(width, height);
    // Viewport edge vectors
    let vp_u = Vec3::new(vp_width as f64, 0, 0);
    let vp_v = Vec3::new(0, -(vp_height as f64), 0);
    // Delta vectors between pixels
    let px_d_u = vp_u / (width as f64);
    let px_d_v = vp_v / (height as f64);
    // Upper left viewport point & pixel
    let (vp_00, px_00) = Self::upper_left_points(camera_center, focal_length, vp_u, vp_v, px_d_u, px_d_v);
    Self {
      aspect_ratio,
      img_size: (width, height),
      focal_length,
      camera_center,
      viewport_size: (vp_width, vp_height),
      vp_u,
      vp_v,
      px_d_u,
      px_d_v,
      vp_00,
      px_00,
    }
  }

  fn image_dimensions(aspect_ratio: f64, width: u64) -> (u64, u64) {
    let height = f64::max(1.0, (width as f64) / aspect_ratio);
    (width, height as u64)
  }
  
  fn viewport_dimensions(image_width: u64, image_height: u64) -> (f64, f64) {
    let height = 2.0;
    let width = height * (image_width as f64) / (image_height as f64);
    (width, height)
  } 
  
  fn upper_left_points(camera_center: Point, focal_length: f64, vp_u: Vec3, vp_v: Vec3, px_d_u: Vec3, px_d_v: Vec3) -> (Point, Point) {
    let vp_00 = *camera_center - Vec3::new(0, 0, focal_length) - (vp_u/2.0) - (vp_v/2.0);
    let px_00 = vp_00 + (px_d_u + px_d_v)/2.0;
    (vp_00.into(), px_00.into())
  }
}
