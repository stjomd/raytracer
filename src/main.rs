mod types;
mod objects;
mod camera;
mod scene;
mod output;

use std::io::stdout;

use camera::Camera;
use objects::Sphere;
use types::Point;
use scene::{scene, Scene};

fn main() {
  let camera = camera();
  let scene = scene();
  let image = camera.render(&scene);
  let _ = output::ppm::write(&image, &mut stdout());
}

fn camera() -> Camera {
  let mut camera = Camera::new(400, 225);
  camera.anti_aliasing(50);
  camera.bounces(2);
  camera
}

fn scene() -> Scene {
  let sphere1 = Sphere::new(Point::new(0, 0, -1), 0.5);
  let sphere2 = Sphere::new(Point::new(0, -100.5, -1), 100);
  scene!(sphere1, sphere2)
}
