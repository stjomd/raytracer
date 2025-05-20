/// An interval or range between two floating point values.
#[derive(Debug, Clone, Copy)]
pub struct Interval {
	pub start: f64,
	pub end: f64,
}

impl Interval {
	/// Returns a new [`Interval`] with the specified start and end values.
	pub fn new<A, B>(start: A, end: B) -> Self
	where
		A: Into<f64>,
		B: Into<f64>,
	{
		Self {
			start: start.into(),
			end: end.into(),
		}
	}
	/// Returns a new [`Interval`] which starts at the specified point end ends at infinity.
	pub fn from<A>(start: A) -> Self
	where
		A: Into<f64>,
	{
		Self {
			start: start.into(),
			end: f64::INFINITY,
		}
	}
	// /// Returns a new empty [`Interval`], which does not contain any number.
	// pub fn empty() -> Self {
	// 	Self::new(f64::INFINITY, -f64::INFINITY)
	// }
	// /// Returns a new universe [`Interval`], which contains all numbers.
	// pub fn universe() -> Self {
	// 	Self::new(-f64::INFINITY, f64::INFINITY)
	// }
}

impl Interval {
	// /// Returns the size of this interval.
	// pub fn size(&self) -> f64 {
	// 	self.end - self.start
	// }
	// /// Indicates if a specified value is contained in this interval.
	// /// If the value is at the interval's ends, returns true.
	// pub fn contains<F: Into<f64>>(&self, value: F) -> bool {
	// 	let value: f64 = value.into();
	// 	self.start <= value && value <= self.end
	// }

	/// Indicates if a specified value is surrounded by this interval.
	/// If the value is at the interval's ends, returns false.
	pub fn surrounds<F: Into<f64>>(&self, value: F) -> bool {
		let value: f64 = value.into();
		self.start < value && value < self.end
	}
}
