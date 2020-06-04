# Luminance-test

This a test I've been making using [`luminance-rs`](https://crates.io/crates/luminance), a wrapper for [`gfx-rs`](https://crates.io/crates/gfx), a bindless (no C libraries) library for graphics (OpenGL / OpenGL ES2+ (WebGL) / Direct3D 11 / Metal / Vulkan). It also uses [`freetype-rs`](https://crates.io/crates/freetype-rs) for text rendering.

# Using this / testing this
You'll need the latest rust version, as well as the development libraries needed by [`freetype-rs`](https://crates.io/crates/freetype-rs), which are the `FreeType2` development libraries.

For debian install the `libfreetype-dev` apt package, for other distros, check your package manager for those libraries.

For MacOS, [`brew install freetype`](https://formulae.brew.sh/formula/freetype#default) should work, but make sure you've got [homebrew](brew.sh) installed.

For Windows, go [here](https://github.com/PistonDevelopers/freetype-sys#for-windows-users) for install instructions, and make sure you change the path to your binaries in the `Cargo.toml` file.