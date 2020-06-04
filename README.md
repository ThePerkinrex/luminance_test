# Luminance-test

This a test I've been making using [`luminance-rs`](https://crates.io/crates/luminance), a wrapper for [`gfx-rs`](https://crates.io/crates/gfx), a bindless (no C libraries) library for graphics (OpenGL / OpenGL ES2+ (WebGL) / Direct3D 11 / Metal / Vulkan). It also uses [`freetype-rs`](https://crates.io/crates/freetype-rs) for text rendering.

# Using this / testing this
You'll need the latest rust version, as well as the development libraries needed by [`freetype-rs`](https://crates.io/crates/freetype-rs), which are the `FreeType2` development libraries.

For debian install the `libfreetype-dev` apt package, for other distros, check your package manager for those libraries.

For MacOS, [`brew install freetype`](https://formulae.brew.sh/formula/freetype#default) should work, but make sure you've got [homebrew](brew.sh) installed.

For Windows, go [here](https://github.com/PistonDevelopers/freetype-sys#for-windows-users) for install instructions, and make sure you change the path to your binaries in the `Cargo.toml` file.

# License 

MIT License

Copyright (c) 2020 ThePerkinrex

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
