# Luminance-test

This a test I've been making using [`luminance-rs`](https://crates.io/crates/luminance), a wrapper for [`gfx-rs`](https://crates.io/crates/gfx), a bindless (no C libraries) library for graphics (OpenGL / OpenGL ES2+ (WebGL) / Direct3D 11 / Metal / Vulkan). It also uses [`rusttype`](https://crates.io/crates/rusttype) for text rendering.

For audio it uses [`ambisonic`](https://crates.io/crates/ambisonic), a wrapper for [`rodio`](https://crates.io/crates/rodio) with 3d audio capabilities (including the doppler effect).

It also uses some utility libraries, such as [`image`](https://crates.io/crates/image) for loading images, [`ron`](https://crates.io/crates/ron) & [`serde`](https://crates.io/crates/serde) for loading RON files (Rusty Object Notation, like JSON but specially designed for Rust), [`lazy_static`](https://crates.io/crates/lazy_static) for creating "constants" for things that can't strictly be constants, [`hound`](https://crates.io/crates/hound) for loading `.wav` files, [`wavefront_obj`](https://crates.io/crates/wavefront_obj) for loading `.obj` & `.mtl` files, and [`cgmath`](https://crates.io/crates/cgmath) for vector & matrix math, as well as utility functions for 3d rendering.

# Using this / testing this
You'll need the latest rust version, and just doing a `cargo run` to compile and start it up. Use `cargo run --release` for a faster startup.
Everything should be built by cargo.

# Example
Dynamic rendering
![dynamic_rendering.gif](dynamic_rendering.gif)
