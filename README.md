# raytracer 🔦

Implementation of a ray tracer in Rust.
Hobby project.
Guidance: https://raytracing.github.io

## Usage

```
raytracer [OPTIONS] --width <WIDTH> --height <HEIGHT>
```

|         Argument | Description                                |
| ---------------: | :----------------------------------------- |
|  `-w, --width`\* | Width of the image in pixels. (mandatory)  |
| `-h, --height`\* | Height of the image in pixels. (mandatory) |
|   `-o, --output` | Path to the output file.                   |
|  `-s, --samples` | Amount of samples per pixel (SSAA).        |
|  `-b, --bounces` | Amount of max. bounces per ray.            |
|    `-g, --gamma` | The gamma value (for gamma correction).    |
|     `-H, --help` | Print help message.                        |
|  `-V, --version` | Print version.                             |

> [!WARNING]
> The scene is hardcoded at the moment!

## Build & Run

If you haven't already, install the [Rust toolchain](https://www.rust-lang.org/tools/install).
You can either clone this repository or if you don't know how, download the source code by clicking on the green `<> Code` button above.

> [!NOTE]
> The following commands utilize the `release` profile, as the resulting executable is substantially faster (at the expense of compile time).
> For debug information, use the `dev` profile by omitting the `-r/--release` flag.

You can build and run the program with the following command:

```
cargo run -rq -- -w <WIDTH> -h <HEIGHT> [OPTIONS]
```

On the first execution this will compile the binary, which might take a little time.
Successive executions will be faster, provided no source code has been changed.

> [!TIP]
> Another performance optimization you can do is compiling the binary strictly for your specific CPU by setting the `target-cpu` compiler flag:
>
> ```
> RUSTFLAGS="-C target-cpu=native" cargo run -rq -- -w <WIDTH> -h <HEIGHT> [OPTIONS]
> ```

### Executable

If you've run the commands above, the executable will be located in the `target/release` subdirectory.
Otherwise, you can compile the binary with the following command:

```
cargo build --release
```

## References

_Peter Shirley, Trevor David Black, Steve Hollasch:_ [Ray Tracing in One Weekend Series](https://raytracing.github.io).
