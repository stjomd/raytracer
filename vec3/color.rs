use super::Vec3;

pub type Color = Vec3;

impl Color {
  pub fn rgb(&self) -> (i32, i32, i32) {
    (
      self.0 as i32,
      self.1 as i32,
      self.2 as i32
    )
  }
}
