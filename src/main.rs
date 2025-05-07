mod vec3;

use vec3::Vec3;

type Color = Vec3;

fn main() {
  let width = 256;
  let height = 256;

  print!("P3\n{} {}\n255\n", width, height);
  for j in 0..height {
    eprint!("\rLines remaining: {}", height - j);
    for i in 0..width {
      let rgb: Color = Vec3(
        (i as f64) / ((width - 1) as f64),
        (j as f64) / ((height - 1) as f64),
        0.0
      );
      let (ir, ig, ib) = rgb.scale(255.9999).to_tuple(|x| x as u8);
      print!("{} {} {}\n", ir, ig, ib);
    }
  }
  eprint!("\rDone.                                  \n");
}
