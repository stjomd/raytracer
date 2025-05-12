# raytracer 🔦 

Implementation of a ray tracer in Rust.
Hobby project.
Guidance: https://raytracing.github.io

## Usage

```
raytracer [OPTIONS] --width <WIDTH> --height <HEIGHT>
```

|         Argument | Description                                |
| ---------------: | ------------------------------------------ |
|  `-w, --width`\* | Width of the image in pixels. (mandatory)  |
| `-h, --height`\* | Height of the image in pixels. (mandatory) |
|   `-o, --output` | Path to the output file.                   |
|  `-s, --samples` | Amount of samples per pixel (SSAA).        |
|  `-b, --bounces` | Amount of max. bounces per ray.            |
|     `-H, --help` | Print help message.                        |
|  `-V, --version` | Print version.                             |

⚠️ The scene is hardcoded at the moment!

## Installation

If you haven't already, install the [Rust toolchain](https://www.rust-lang.org/tools/install).
You can either clone this repository or if you don't know how, download the source code by clicking on the green `<> Code` button above.

In the project directory you can then build the executable like this:

```
cargo build --release
```

which will create the executable in the `target/release/` subdirectory, which can be run with:

```
./target/release/raytracer -w <WIDTH> -h <HEIGHT> -o img.ppm [OPTIONS]
```

This should produce the image file `img.ppm` in the current directory.

Or even simpler, you can build and run the program in just one step:

```
cargo run -rq -- -w <WIDTH> -h <HEIGHT> -o img.ppm [OPTIONS]
```

Using the `-r/--release` flag is recommended, as it is substantially faster.

# References

_Peter Shirley, Trevor David Black, Steve Hollasch:_ [Ray Tracing in One Weekend Series](https://raytracing.github.io).
