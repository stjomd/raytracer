use std::io::{Error, Write};

use crate::core::types::{Image, ToVec3};

/// Writes the image in .ppm format.
pub fn write<W: Write>(image: &Image, gamma: f64, writer: &mut W) -> Result<(), Error> {
	let correction = 1.0 / gamma;
	writeln!(writer, "P3\n{} {}\n255\n", image.width(), image.height())?;
	for line in image {
		for pixel in line {
			let rgb = pixel.to_vec3().exp(correction);
			let (r, g, b) = rgb.to_tuple(|x| (256.0 * x.clamp(0.0, 0.999)) as u8);
			writeln!(writer, "{} {} {}", r, g, b)?;
		}
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use crate::core::types::{Color, Image};

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
		let write_result = write(&image, 2.2, &mut buf);
		assert!(write_result.is_ok(), "writing should succeed, but didn't");
		let decode_result = String::from_utf8(buf);
		assert!(decode_result.is_ok(), "converting from utf-8 should succeed, but didn't");
		let actual = decode_result.unwrap();
		assert_eq!(actual, expected, ".ppm output should match, but didn't");
	}
}
