use raytracer::camera::{Camera, CameraSetup};
use raytracer::scene::Scene;
use raytracer::types::Color;

#[test]
fn if_empty_scene_then_render_produces_image_with_bg() {
	// This ray shoots out from the camera center into the view direction:
	let setup = CameraSetup { width: 50, height: 50, ..Default::default() };
	let camera = Camera::from(setup);
	// This scene has no objects:
	let scene = Scene::new();

	// TODO: adjust when scene supports custom backgrounds
	let image = camera.render(&scene);
	let mut violating_px_count = 0;
	for i in 0..image.height() {
		for j in 0..image.width() {
			if image[(i, j)] == Color::black() {
				violating_px_count += 1;
			}
		}
	}
	assert_eq!(violating_px_count, 0, "all pixels should be non-black, but {} pixels were", violating_px_count);
}
