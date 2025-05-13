use std::io::{Error, Write};

use crate::types::{Image, Interval, ToVec3};

/// Prints the image to stdout in .ppm format.
pub fn write<W: Write>(image: &Image, gamma: f64, writer: &mut W) -> Result<(), Error> {
	writeln!(writer, "P3\n{} {}\n255\n", image.width(), image.height())?;
	for i in 0..image.height() {
		for j in 0..image.width() {
			let rgb = image[(i, j)].to_vec3().exp(gamma);
			let intensity = Interval::new(0, 0.999);
			let (r, g, b) = rgb.to_tuple(|x| (256.0 * intensity.clamp(x)) as u8);
			writeln!(writer, "{} {} {}", r, g, b)?;
		}
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use crate::types::{Color, Image};

	use super::write;

	#[test]
	fn correct_ppm() {
		// This is a 2x2 image:
		let mut image = Image::init(2, 2);
		// The bottom right pixel is red:
		image[(1, 1)] = Color::new(1, 0, 0);

		// The output should match the .ppm format:
		let expected = "P3
2 2
255

0 0 0
0 0 0
0 0 0
255 0 0
";
		let mut buf: Vec<u8> = Vec::new();
		let write_result = write(&image, &mut buf);
		assert!(write_result.is_ok(), "writing should succeed, but didn't");
		let decode_result = String::from_utf8(buf);
		assert!(decode_result.is_ok(), "converting from utf-8 should succeed, but didn't");
		let actual = decode_result.unwrap();
		assert_eq!(actual, expected, ".ppm output should match, but didn't");
	}
}
