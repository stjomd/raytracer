mod hit;
mod sphere;
mod material;

pub use hit::{Hittable, Hit, Object, ToObject};
pub use sphere::Sphere;
pub use material::Material;
