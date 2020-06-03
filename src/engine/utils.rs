use luminance::context::GraphicsContext;
use luminance::pixel::{NormRGBA8UI};
use luminance::texture::{Dim2, GenMipmaps, Sampler, Texture};

use image;

use std::path::Path;

// read the texture into memory as a whole bloc (i.e. no streaming)
pub fn read_image(path: &Path) -> Option<image::RgbaImage> {
	image::open(path).map(|img| img.flipv().to_rgba()).ok()
}

pub fn load_from_disk<C: GraphicsContext>(
	surface: &mut C,
	img: image::RgbaImage,
) -> Texture<Dim2, NormRGBA8UI> {
	let (width, height) = img.dimensions();
	let texels = img.into_raw();

	// create the luminance texture; the third argument is the number of mipmaps we want (leave it
	// to 0 for now) and the latest is the sampler to use when sampling the texels in the
	// shader (we’ll just use the default one)
	let tex = Texture::new(surface, [width, height], 0, Sampler::default())
		.expect("luminance texture creation error");

	// the first argument disables mipmap generation (we don’t care so far)
	tex.upload_raw(GenMipmaps::No, &texels).unwrap();

	tex
}