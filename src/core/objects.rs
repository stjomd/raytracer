mod hit;
mod material;
mod sphere;

pub use hit::{Hit, Hittable, Object, ToObject};
pub use material::Material;
pub use sphere::Sphere;
