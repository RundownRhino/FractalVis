# FractalVis
A fractal visualiser written in C# and Rust. Uses WPF for the GUI, and calls a Rust function via CFFI to render the fractal (on the CPU, for now). Supports zooming in.

Was messily ported from .NET 4.0 to 4.7.2, which is why there are still mentions of 4.0 in some configs, and few new C# features used.

# Build Instructions
1. Do `cargo build --release` in `fractals` to build the Rust dll.
2. Build the C# solution, which should automatically link to the dll in `fractals/target/release`.

# Usage
Left mouse button drag to select a region to zoom on. Right mouse button drag to select a region to zoom out on (the zoom will be adjusted so that the current viewport is projected into the rectangle selected). Middle mouse button to reset to default zoom. In some cases, the fractal doesn't render on program start; middle mouse button fixes that.