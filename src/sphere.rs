use std::ops::Range;

use crate::types::{Point, ToVec3};
use crate::hit::{Hit, Hittable};

/// A 3D sphere.
pub struct Sphere {
  center: Point,
  radius: f64
}

impl Sphere {
  /// Creates a new 3D sphere with the specified center point and radius.
  /// If `radius` is negative, a radius of 0 is assumed.
  pub fn new(center: Point, radius: f64) -> Self {
    Self { center, radius: f64::max(0.0, radius) }
  }
}

impl Hittable for Sphere {
  fn hit<F: Into<f64>>(&self, ray: crate::types::Ray, t_range: Range<F>) -> Option<Hit> {
    // Solve quadratic equation
    let cq = self.center.to_vec3() - ray.origin;
    let a = ray.direction.norm_sq();
    let h = ray.direction.dot(cq);
    let c = cq.norm_sq() - self.radius*self.radius;
  
    let discr = h*h - a*c;
    if discr < 0.0 {
      return None
    }

    let discr_sqrt = discr.sqrt();
    let t1 = (h - discr_sqrt) / a;
    let t2 = (h + discr_sqrt) / a;

    // Choose a plausible root
    let t_min: f64 = t_range.start.into();
    let t_max: f64 = t_range.end.into();
    let t = if t_min <= t1 && t1 <= t_max {
      t1
    } else if t_min <= t2 && t2 <= t_max {
      t2
    } else {
      return None;
    };

    let point = ray.at(t);
    let outward_normal = (point.to_vec3() - self.center) / self.radius;

    let (normal, is_front_face) = Hit::determine_front_face(ray, outward_normal);
    return Some(Hit { t, point, normal, is_front_face })
  }
}
