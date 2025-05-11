mod types;
mod objects;
mod camera;

use camera::Camera;
use objects::{objects, Sphere};
use types::Point;

fn main() {
  let camera = Camera::new(400, 225);

  let sphere1 = Sphere::new(Point::new(0, 0, -1), 0.5);
  let sphere2 = Sphere::new(Point::new(0, -100.5, -1), 100);
  let objects = objects!(sphere1, sphere2);

  camera.render(&objects);
}
