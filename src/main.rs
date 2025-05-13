mod args;
mod types;
mod objects;
mod camera;
mod scene;
mod output;

use std::fs::File;
use std::io::{stdout, Write};

use args::Args;
use camera::Camera;
use objects::{Material, Sphere};
use types::{Color, Point};
use scene::{scene, Scene};

fn main() {
  let args = Args::parse();

  let mut writer: Box<dyn Write> = if let Some(ref path) = args.output {
    let file = File::create(path).unwrap();
    Box::new(file)
  } else {
    Box::new(stdout())
  };

  let camera = camera(&args);
  let scene = scene();
  let image = camera.render(&scene);

  output::ppm::write(&image, args.gamma, &mut writer).unwrap();
}

fn camera(args: &Args) -> Camera {
  let mut camera = Camera::new(args.width, args.height);
  camera.anti_aliasing(args.samples);
  camera.bounces(args.bounces);
  camera
}

fn scene() -> Scene {
  let sphere_bottom = Sphere::new(
    Point::new(0, -100.5, -1),
    100,
    Material::Matte { color: Color::new(0.8, 0.8, 0) }
  );
  let sphere_center = Sphere::new(
    Point::new(0, 0, -1.2),
    0.5,
    Material::Absorbant
  );
  let sphere_left = Sphere::new(
    Point::new(-1, 0, -1),
    0.5,
    Material::Metal { color: Color::new(0.8, 0.8, 0.8) }
  );
  let sphere_right = Sphere::new(
    Point::new(1, 0, -1),
    0.5,
    Material::Metal { color: Color::new(0.8, 0.6, 0.2) }
  );
  scene!(sphere_bottom, sphere_center, sphere_left, sphere_right)
}
