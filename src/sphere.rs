use std::ops::RangeInclusive;

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
  pub fn new<F: Into<f64>>(center: Point, radius: F) -> Self {
    Self { center, radius: f64::max(0.0, radius.into()) }
  }
}

impl Hittable for Sphere {
  fn hit<F>(&self, ray: crate::types::Ray, t_range: RangeInclusive<F>) -> Option<Hit>
  where F: Into<f64> + Clone {
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
    let t_min: f64 = t_range.start().clone().into();
    let t_max: f64 = t_range.end().clone().into();
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
    Some(Hit { t, point, normal, is_front_face })
  }
}

#[cfg(test)]
mod tests {
  use crate::hit::Hittable;
  use crate::types::{Point, Ray, Vec3};
  use super::Sphere;

  #[test]
  fn if_ray_hits_sphere_then_some_and_correct_intersect() {
    // This sphere is positioned at origin and has radius 1:
    let sphere = Sphere::new(Point::origin(), 1);
    // This ray starts 'on the left' from the sphere, and points horizontally (x-axis) towards it:
    let ray = Ray::new(Point::new(-10, 0, 0), Vec3::new(1, 0, 0));

    // The ray should intersect the sphere at (-1, 0, 0):
    let hit = sphere.hit(ray, -15..=15);
    assert!(hit.is_some(), "ray should hit the sphere, but returned None");
    let hit = hit.unwrap();
    assert_eq!(hit.point, Point::new(-1, 0, 0), "ray should intersect sphere at (-1, 0, 0)");
  }

  #[test]
  fn if_ray_doesnt_hit_sphere_then_none() {
    // This sphere is positioned at origin and has radius 1:
    let sphere = Sphere::new(Point::origin(), 1);
    // This ray starts 'on the left' from the sphere, and points vertically (y-axis) and misses it:
    let ray = Ray::new(Point::new(-10, 0, 0), Vec3::new(0, 1, 0));

    // The ray should not intersect the sphere:
    let hit = sphere.hit(ray, -15..=15);
    assert!(hit.is_none(), "ray should miss the sphere, but returned Some")
  }

  #[test]
  fn if_ray_hits_sphere_and_t_outside_range_then_none() {
    // This sphere is positioned at origin and has radius 1:
    let sphere = Sphere::new(Point::origin(), 1);
    // This ray starts 'on the left' from the sphere, and points horizontally (x-axis) towards it:
    let ray = Ray::new(Point::new(-10, 0, 0), Vec3::new(1, 0, 0));

    // The ray should intersect the sphere at (-1, 0, 0), meaning `t` should equal 9 (-10 + t*1 = -1)
    let hit = sphere.hit(ray, 0..=1);
    assert!(hit.is_none(), "parameter t lies outside the specified range, but returned Some")
  }

  #[test]
  #[ignore = "currently no distinction between in front of/behind camera"]
  fn if_ray_hits_sphere_from_outside_then_some_and_front_face() {
    // This sphere is positioned at origin and has radius 1:
    let sphere = Sphere::new(Point::origin(), 1);
    // This ray starts 'on the left' from the sphere, and points horizontally (x-axis) towards it:
    let ray = Ray::new(Point::new(-10, 0, 0), Vec3::new(1, 0, 0));

    // The ray should intersect the sphere from outside:
    let hit = sphere.hit(ray, -15..=15);
    assert!(hit.is_some(), "ray should hit the sphere, but returned None");
    let hit = hit.unwrap();
    println!("{:?}", hit);
    assert!(hit.is_front_face, "hit should be on the front face, but was not");
  }

  #[test]
  #[ignore = "currently no distinction between in front of/behind camera"]
  fn if_ray_hits_sphere_from_inside_then_some_and_not_front_face() {
    // This sphere is positioned at origin and has radius 10:
    let sphere = Sphere::new(Point::origin(), 10);
    // This ray starts from inside sphere:
    let ray = Ray::new(Point::new(5, 0, 0), Vec3::new(1, 0, 0));

    // The ray should intersect the sphere from inside:
    let hit = sphere.hit(ray, -15..=15);
    assert!(hit.is_some(), "ray should hit the sphere, but returned None");
    let hit = hit.unwrap();
    assert!(!hit.is_front_face, "hit should be on the back face, but was front face");
  }
}
