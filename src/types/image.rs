use std::ops;

use super::Color;

// MARK: - Image

type ImageIdx = (usize, usize);

/// A type that represents an image.
#[derive(Debug, Clone)]
pub struct Image {
	lines: Vec<Color>,
	height: usize,
	width: usize,
}

impl Image {
	/// Creates a new black image with the specified height and width.
	pub fn init(height: usize, width: usize) -> Self {
		let lines = vec![Color::black(); height * width];
		Self { lines, height, width }
	}
	/// Returns the height of this image, in pixels.
	pub fn height(&self) -> usize {
		self.height
	}
	/// Returns the width of this image, in pixels.
	pub fn width(&self) -> usize {
		self.width
	}
	/// Checks if the specified index is valid for this image.
	/// Panics if either the row or column index is out of bounds.
	fn check_index(&self, index: &ImageIdx) {
		if index.0 >= self.height {
			panic!("index out of bounds: the height is {} but the index is {:?}", self.height, index)
		}
		if index.1 >= self.width {
			panic!("index out of bounds: the width is {} but the index is {:?}", self.width, index)
		}
	}
}

impl ops::Index<ImageIdx> for Image {
	type Output = Color;
	/// Performs the indexing operation.
	/// Indexation for [`Image`] is row-major: meaning the first index in the tuple
	/// is the index of the row, and the second index corresponds to the column.
	/// 
	/// ```
	/// // An image that is 10 pixels wide and 5 pixels tall:
	/// let mut image = Image::init(5, 10);
	/// // Set the pixel in the 5th row, 9th column to black:
	/// let color = image[(4, 9)];
	/// // The following panics:
	/// let color = image[(9, 4)]; // panic!
	/// ```
	///
	/// # Panics
	/// Panics if either the height or the width is out of bounds.
	fn index(&self, index: ImageIdx) -> &Self::Output {
		self.check_index(&index);
		&self.lines[index.0 * self.width + index.1]
	}
}
impl ops::IndexMut<ImageIdx> for Image {
	/// Performs the mutable indexing operation.
	/// Indexation for [`Image`] is row-major: meaning the first index in the tuple
	/// is the index of the row, and the second index corresponds to the column.
	/// 
	/// ```
	/// // An image that is 10 pixels wide and 5 pixels tall:
	/// let mut image = Image::init(5, 10);
	/// // Set the pixel in the 5th row, 9th column to black:
	/// image[(4, 9)] = Color::black();
	/// // The following panics:
	/// image[(9, 4)] = Color::black(); // panic!
	/// ```
	///
	/// # Panics
	/// Panics if either the height or the width is out of bounds.
	fn index_mut(&mut self, index: ImageIdx) -> &mut Self::Output {
		self.check_index(&index);
		&mut self.lines[index.0 * self.width + index.1]
	}
}
