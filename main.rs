fn main() {
  let width = 256;
  let height = 256;

  print!("P3\n{} {}\n255\n", width, height);
  for j in 0..height {
    eprint!("\rLines remaining: {}", height - j);
    for i in 0..width {
      let g = (i as f64) / ((width - 1) as f64);
      let b = (j as f64) / ((height - 1) as f64);
      let r = 0.0;

      let ir = (256.0 * r) as i32;
      let ig = (256.0 * g) as i32;
      let ib = (256.0 * b) as i32;

      print!("{} {} {}\n", ir, ig, ib);
    }
  }
  eprint!("\rDone.                                  \n");
}
