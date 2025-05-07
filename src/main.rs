mod types;

use types::Color;

fn main() {
  let width = 256;
  let height = 256;

  print!("P3\n{} {}\n255\n", width, height);
  for j in 0..height {
    eprint!("\rLines remaining: {}", height - j);
    for i in 0..width {
      let rgb = Color::new(
        (i as f64) / ((width - 1) as f64),
        (j as f64) / ((height - 1) as f64),
        0.0
      );
      print!("{}\n", rgb);
    }
  }
  eprint!("\rDone.                                  \n");
}
