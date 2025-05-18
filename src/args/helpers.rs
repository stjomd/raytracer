use clap::error::{Error, ErrorKind};
use raytracer::types::Point;

/// Represents a type that can be represented as a string in the CLI.
pub trait ToArgString {
	/// Converts the type to a string representation for the CLI.
	fn to_arg_str(&self) -> String;
}
impl ToArgString for u32 {
	fn to_arg_str(&self) -> String {
		self.to_string()
	}
}
impl ToArgString for f64 {
	fn to_arg_str(&self) -> String {
		format!("{:.1}", self)
	}
}
impl ToArgString for &str {
	fn to_arg_str(&self) -> String {
		format!("'{}'", self)
	}
}
impl ToArgString for Point {
	fn to_arg_str(&self) -> String {
		format!("'{},{},{}'", self.0, self.1, self.2)
	}
}

/// A type that represents an unquoted string in the CLI.
pub struct UnquotedArgString(pub &'static str);
impl ToArgString for UnquotedArgString {
	fn to_arg_str(&self) -> String {
		self.0.to_string()
	}
}

/// Builds a description message for an option.
pub fn arg_desc<T: ToArgString>(desc: &str, format: Option<&str>, default: Option<T>) -> String {
	let formatted_hints = [
		format.map(|str| format!("format: '{}'", str)),
		default.map(|str| format!("default: {}", str.to_arg_str())),
	];
	let present_hints = formatted_hints.iter()
		.filter_map(|x| x.clone())
		.collect::<Vec<_>>();
	format!("{} [{}]", desc, present_hints.join(", "))
}

/// Parses a string argument into a [`Point`].
pub fn parse_point(arg: &str) -> Result<Point, Error> {
	let msg: &str = "format for point type is 'x,y,z', where 'x', 'y', and 'z' are numeric
example: '1.0,-2.0,3'\n
hint: try specifying the value like this: '--option=-1.5,2.0,3'";
	arg.parse::<Point>()
    .map_err(|e| Error::raw(ErrorKind::ValueValidation, format!("{}\n{}", e, msg)))
}

#[cfg(test)]
mod tests {
	use raytracer::types::Point;

	use crate::args::helpers::UnquotedArgString;

	use super::{arg_desc, parse_point};

	#[test]
	fn should_parse_point_with_given_coordinates() {
		let point = parse_point("-1.0,-2,3.0");
		assert!(point.is_ok(), "point should be parsed, but error was returned");
		let point = point.unwrap();
		assert_eq!(point, Point(-1.0, -2.0, 3.0), "coordinates should be equal to arg");
	}

	#[test]
	fn if_point_arg_has_less_coordinates_then_error() {
		let point = parse_point("-1.0,2");
		assert!(point.is_err(), "arg has 2 coordinates, but point was parsed");
	}

	#[test]
	fn if_point_arg_has_more_coordinates_then_error() {
		let point = parse_point("-1.0,2,3.0,-4");
		assert!(point.is_err(), "arg has 4 coordinates, but point was parsed");
	}

	/// Note for future me: this is just useless for this case, just write the functions directly
	mod paramtest {
		macro_rules! arg_desc_appendix {
			( 
				$($name:ident {
					params: {
						let format: $ftype:ty = $fval:expr;
						let default: $dtype:ty = $dval:expr;
					},
					expect: $expected:expr$(,)?
				}),+
			) => {
				$(#[test]
				fn $name() {
					let format: $ftype = $fval;
					let default: $dtype = $dval;					
					let actual = arg_desc("arg_desc_appendix", format, default);
					let expected = format!("arg_desc_appendix {}", $expected);
					assert_eq!(actual, expected);
				})+
			};
		}
		pub(super) use arg_desc_appendix;
	}

	paramtest::arg_desc_appendix! {
		if_format_and_default_then_description_has_both {
			params: {
				let format: Option<&str> = Some("float");
				let default: Option<f64> = Some(2.5);
			},
			expect: "[format: 'float', default: 2.5]"
		},
		if_format_but_no_default_then_description_only_has_format {
			params: {
				let format: Option<&str> = Some("a,b,c");
				let default: Option<&str> = None;
			},
			expect: "[format: 'a,b,c']"
		},
		if_default_is_unquoted_str_then_description_has_no_quotes_for_default {
			params: {
				let format: Option<&str> = Some("Write");
				let default: Option<UnquotedArgString> = Some(UnquotedArgString("stdout"));
			},
			expect: "[format: 'Write', default: stdout]"
		}
	}

}
