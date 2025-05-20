use std::fmt::Display;
use std::ops;
use std::str::FromStr;

use serde::Deserialize;

/// An epsilon value used for near zero comparisons.
/// Two values are considered to be equal if their absolute
/// difference is smaller than this value.
const NEAR_ZERO_EPSILON: f64 = 1e-8;

/// A vector of three floating-point values.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Vec3(pub f64, pub f64, pub f64);

/// Denotes an object that can be converted to [`Vec3`].
pub trait ToVec3 {
	/// Converts this object to a [`Vec3`].
	fn to_vec3(&self) -> Vec3;
}

// Constructors
impl Vec3 {
	/// Creates a new vector from the specified parameters.
	/// Each of the parameters is converted to a floating-point type value (`f64`).
	pub fn new<X, Y, Z>(x: X, y: Y, z: Z) -> Self
	where
		X: Into<f64>,
		Y: Into<f64>,
		Z: Into<f64>,
	{
		Self(x.into(), y.into(), z.into())
	}
	/// Creates a new vector where each value is zero.
	pub const fn zero() -> Self {
		Self(0.0, 0.0, 0.0)
	}
	/// Creates a new vector where each value is the same as the specified one in the parameter.
	pub fn diagonal<A: Into<f64>>(xyz: A) -> Self {
		let val = xyz.into();
		Self::new(val, val, val)
	}
	// Creates a new vector where each value is random within a specified range.
	pub fn random<A: Into<f64>>(range: ops::Range<A>) -> Self {
		let (start, end): (f64, f64) = (range.start.into(), range.end.into());
		Self::new(
			rand::random_range(start..end),
			rand::random_range(start..end),
			rand::random_range(start..end),
		)
	}
	/// Creates a new random unit vector.
	/// This method randomly distributes the coordinates across the unit sphere.
	pub fn random_unit() -> Self {
		loop {
			let vec = Self::random(-1..1);
			if (1e-160..1.0).contains(&vec.norm_sq()) {
				return vec.unit();
			}
		}
	}
	/// Creates a new random (not necessarily unit) vector.
	/// This method randomly distributes the coordinates across the unit disk (z = 0).
	pub fn random_in_unit_disk() -> Self {
		loop {
			let vec = Self::new(
				rand::random_range(-1.0..1.0),
				rand::random_range(-1.0..1.0),
				0.0,
			);
			if vec.norm_sq() < 1.0 {
				return vec;
			}
		}
	}
}

// Getters
impl Vec3 {
	/// The x coordinate of this vector.
	pub fn x(&self) -> f64 {
		self.0
	}
	/// The y coordinate of this vector.
	pub fn y(&self) -> f64 {
		self.1
	}
	/// The z coordinate of this vector.
	pub fn z(&self) -> f64 {
		self.2
	}
}

// String conversions
impl Display for Vec3 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{} {} {}]", self.0, self.1, self.2)
	}
}
impl FromStr for Vec3 {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let opening_brackets = ['[', '(', '<', '{'];
		let closing_brackets = [']', ')', '>', '}'];
		let separators = [' ', ',', ';'];

		let trimmed = s
			.trim()
			.replace(opening_brackets, "")
			.replace(closing_brackets, "");
		let values: Vec<f64> = trimmed
			.split(separators)
			.map(|val| val.parse::<f64>().map_err(|e| format!("{} '{}'", e, val)))
			.collect::<Result<_, _>>()?;

		if values.len() != 3 {
			return Err(format!("expected 3 coordinates, got {}", values.len()));
		}
		Ok(Self(values[0], values[1], values[2]))
	}
}

// Indexes
impl ops::Index<usize> for Vec3 {
	type Output = f64;
	fn index(&self, index: usize) -> &Self::Output {
		match index {
			0 => &self.0,
			1 => &self.1,
			2 => &self.2,
			_ => panic!("index out of bounds {}", index),
		}
	}
}
impl ops::IndexMut<usize> for Vec3 {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		match index {
			0 => &mut self.0,
			1 => &mut self.1,
			2 => &mut self.2,
			_ => panic!("index out of bounds {}", index),
		}
	}
}

// Operators
impl ops::Neg for Vec3 {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Vec3(-self.0, -self.1, -self.2)
	}
}
impl<T> ops::Add<T> for Vec3
where
	T: Into<Vec3>,
{
	type Output = Self;
	fn add(self, rhs: T) -> Self::Output {
		let other = rhs.into();
		Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
	}
}
impl<T> ops::Sub<T> for Vec3
where
	T: Into<Vec3>,
{
	type Output = Self;
	fn sub(self, rhs: T) -> Self::Output {
		let other = rhs.into();
		Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
	}
}
impl<T> ops::Mul<T> for Vec3
where
	T: Into<Vec3>,
{
	type Output = Self;
	fn mul(self, rhs: T) -> Self::Output {
		let other = rhs.into();
		Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
	}
}
impl<T> ops::Div<T> for Vec3
where
	T: Into<Vec3>,
{
	type Output = Self;
	fn div(self, rhs: T) -> Self::Output {
		let other = rhs.into();
		Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
	}
}

// Operators with scalars
impl ops::Mul<f64> for Vec3 {
	type Output = Self;
	fn mul(self, rhs: f64) -> Self::Output {
		Vec3(rhs * self.0, rhs * self.1, rhs * self.2)
	}
}
impl ops::Div<f64> for Vec3 {
	type Output = Self;
	fn div(self, rhs: f64) -> Self::Output {
		Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
	}
}

// Assignment operators
impl ops::AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
		self.1 += rhs.1;
		self.2 += rhs.2;
	}
}
impl ops::SubAssign for Vec3 {
	fn sub_assign(&mut self, rhs: Self) {
		self.0 -= rhs.0;
		self.1 -= rhs.1;
		self.2 -= rhs.2;
	}
}
impl ops::MulAssign for Vec3 {
	fn mul_assign(&mut self, rhs: Self) {
		self.0 *= rhs.0;
		self.1 *= rhs.1;
		self.2 *= rhs.2;
	}
}
impl ops::DivAssign for Vec3 {
	fn div_assign(&mut self, rhs: Self) {
		self.0 /= rhs.0;
		self.1 /= rhs.1;
		self.2 /= rhs.2;
	}
}

// Properties
impl Vec3 {
	/// Calculates the squared norm `||v||^2` of this vector `v = (x, y, z)`, that is
	/// the value `x^2 + y^2 + z^2`.
	pub fn norm_sq(&self) -> f64 {
		self.0 * self.0 + self.1 * self.1 + self.2 * self.2
	}
	/// Calculates the norm (distance from origin) `||v||` of this vector `v`.
	pub fn norm(&self) -> f64 {
		self.norm_sq().sqrt()
	}
}

// Operations
impl Vec3 {
	/// Returns a new vector `a * v` that is obtained by scaling this vector `v` by a factor of `a`.
	pub fn scale<T: Into<f64>>(self, f: T) -> Self {
		self * f.into()
	}
	/// Returns a new vector `u` that is obtained by raising every coordinate of this vector `v`
	/// to a specified power.
	pub fn exp<T: Into<f64>>(self, pwr: T) -> Self {
		let powr = pwr.into();
		Vec3(self.0.powf(powr), self.1.powf(powr), self.2.powf(powr))
	}
	/// Calculates the dot product `v * u` of this vector `v` and another vector `u`.
	pub fn dot(self, rhs: Self) -> f64 {
		self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
	}
	/// Calculates the cross product `v x u` of this vector `v` and another vector `u`.
	pub fn cross(self, rhs: Self) -> Self {
		Vec3(
			self.1 * rhs.2 - self.2 * rhs.1,
			self.2 * rhs.0 - self.0 * rhs.2,
			self.0 * rhs.1 - self.1 * rhs.0,
		)
	}
	/// Returns a new unit vector (vector of norm 1) pointing in the same direction as this vector.
	pub fn unit(self) -> Self {
		self / self.norm()
	}
}

// Miscellaneous
impl Vec3 {
	/// Converts this vector to a tuple of three values.
	/// The conversion is specified by the passed in function.
	///
	/// ```
	/// let bytes: (u8, u8, u8) = Vec3::new(1, 2.0, 3.0).to_tuple(|x| x as u8);
	/// // (1, 2, 3)
	///
	/// let squared = Vec3::new(1, 2, 3).to_tuple(|x| x*x);
	/// // (1.0, 4.0, 9.0)
	/// ```
	pub fn to_tuple<T, F>(self, f: F) -> (T, T, T)
	where
		F: Fn(f64) -> T,
	{
		(f(self.0), f(self.1), f(self.2))
	}
	/// Checks if this vector is almost zero, that is each of the values is almost zero.
	///
	/// A value is considered to be almost zero if its absolute value is smaller
	/// than [`NEAR_ZERO_EPSILON`].
	pub fn is_near_zero(&self) -> bool {
		f64::abs(self.0) < NEAR_ZERO_EPSILON
			&& f64::abs(self.1) < NEAR_ZERO_EPSILON
			&& f64::abs(self.2) < NEAR_ZERO_EPSILON
	}
}

#[cfg(test)]
mod tests {
	use super::Vec3;

	/// Checks whether two `f64` values are approximately equal within [`super::NEAR_ZERO_EPSILON`].
	fn f64_approx_eq(a: f64, b: f64) -> bool {
		f64::abs(a - b) < super::NEAR_ZERO_EPSILON
	}

	#[test]
	fn random_unit_has_length_one() {
		let vec = Vec3::random_unit();
		let length = vec.norm();
		assert!(
			f64_approx_eq(1.0, length),
			"length of random unit vector should be 1, but was {}",
			length
		)
	}

	#[test]
	fn random_in_unit_disk_has_length_less_than_one() {
		let vec = Vec3::random_in_unit_disk();
		let length = vec.norm();
		assert!(
			length < 1.0,
			"length of random unit vector should be < 1.0, but was {}",
			length
		)
	}

	#[test]
	fn norm_is_correct_length() {
		let vec = Vec3::new(2, 10, 11);
		let length = vec.norm();
		assert!(
			f64_approx_eq(15.0, length),
			"length should be 15.0, but was {}",
			length
		)
	}

	#[test]
	fn scale_multiplies_all_coordinates() {
		let vec = Vec3::new(0, 5, -1);
		let scaled = vec.scale(10);
		assert!(
			f64_approx_eq(0.0, scaled.x()),
			"x coordinate should be 0*10 = 0, but was {}",
			scaled.x()
		);
		assert!(
			f64_approx_eq(50.0, scaled.y()),
			"y coordinate should be 5*10 = 50, but was {}",
			scaled.y()
		);
		assert!(
			f64_approx_eq(-10.0, scaled.z()),
			"z coordinate should be -1*10 = -1, but was {}",
			scaled.z()
		);
	}

	#[test]
	fn exp_raises_all_coordinates() {
		let vec = Vec3::new(0.0, 1.0, -3.0);
		let expd = vec.exp(2.0);
		assert!(
			f64_approx_eq(0.0, expd.x()),
			"x coordinate should be 0^2 = 0, but was {}",
			expd.x()
		);
		assert!(
			f64_approx_eq(1.0, expd.y()),
			"y coordinate should be 1^2 = 1, but was {}",
			expd.y()
		);
		assert!(
			f64_approx_eq(9.0, expd.z()),
			"z coordinate should be (-3)^2 = 9, but was {}",
			expd.z()
		);
	}

	#[test]
	fn dot_product_of_orthogonal_vecs_is_zero() {
		let vec1 = Vec3::new(1, 0, 0);
		let vec2 = Vec3::new(0, 1, 0);
		let dot = vec1.dot(vec2);
		assert!(
			f64_approx_eq(0.0, dot),
			"dot product of orthogonal vectors should be 0, but was {}",
			dot
		);
	}

	#[test]
	fn cross_product_is_orthogonal() {
		let vec1 = Vec3::new(-1.5, 2.4, 3.8);
		let vec2 = Vec3::new(0.0, -0.5, 5.5);
		let cross = vec1.cross(vec2);
		// Two vectors are orthogonal if their dot product is zero:
		assert!(
			f64_approx_eq(0.0, cross.dot(vec1)),
			"cross product is not orthogonal to vec1"
		);
		assert!(
			f64_approx_eq(0.0, cross.dot(vec2)),
			"cross product is not orthogonal to vec2"
		);
	}

	#[test]
	fn cross_product_is_anticommutative() {
		let vec1 = Vec3::new(-1.5, 2.4, 3.8);
		let vec2 = Vec3::new(0.0, -0.5, 5.5);
		let cross1 = vec1.cross(vec2);
		let cross2 = vec2.cross(vec1);
		assert_eq!(cross1, -cross2, "A.cross(B) should equal -B.cross(A)");
	}

	#[test]
	fn if_orthogonal_vecs_then_cross_product_length_is_product_of_vecs_lengths() {
		// These vectors are orthogonal and have both length 5:
		let vec1 = Vec3::new(3, 4, 0);
		let vec2 = Vec3::new(4, -3, 0);
		// Thus the cross product should have length 25:
		let cross = vec1.cross(vec2);
		assert!(f64_approx_eq(25.0, cross.norm()), "was {}", cross.norm())
	}

	#[test]
	fn unit_returns_vector_of_length_one() {
		let vec = Vec3::new(2.3, -5, 0.0);
		let length_of_unit = vec.unit().norm();
		assert_eq!(
			1.0, length_of_unit,
			"length of unit vector should be 1, but was {}",
			length_of_unit
		)
	}

	#[test]
	fn unit_of_zero_vector_has_length_nan() {
		let vec = Vec3::zero();
		let length_of_unit = vec.unit().norm();
		assert!(
			length_of_unit.is_nan(),
			"length of unit vector should be NaN, but was {}",
			length_of_unit
		)
	}
}
