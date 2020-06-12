use luminance::context::GraphicsContext;
use luminance::pixel::{Depth32F, NormRGBA8UI};
use luminance::texture::{Dim2, GenMipmaps, Sampler, Texture};

use image;

use zip::ZipArchive;

use lazy_static::lazy_static;

use cgmath::Rad;

use std::fs::File;
use std::io::{Cursor, Read, Seek};
use std::path::{Path, PathBuf};

// read the texture into memory as a whole bloc (i.e. no streaming)
pub fn read_image(file_loader: &mut FileLoader, path: &Path) -> Option<image::RgbaImage> {
	image::load(
		file_loader.load(path).expect("Error loading file"),
		image::ImageFormat::from_path(path).expect("Error loading format"),
	)
	.map(|img| img.flipv().to_rgba())
	.ok()
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

#[derive(Clone, Copy)]
pub struct RgbaColor {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl RgbaColor {
	pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self { r, g, b, a }
	}

	#[allow(dead_code)]
	pub fn array(&self) -> [u8; 4] {
		[self.r, self.g, self.b, self.a]
	}
}

impl std::fmt::Debug for RgbaColor {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "RGBA({}, {}, {}, {})", self.r, self.g, self.b, self.a)
	}
}

impl std::fmt::Display for RgbaColor {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "{:?}", self)
	}
}

#[cfg(feature = "pack")]
static ZIP: Option<&[u8]> = Some(include_bytes!("../../assets.zip"));
#[cfg(not(any(feature = "pack")))]
static ZIP: Option<&[u8]> = None;

lazy_static! {
	pub static ref ASSETS_PATH: &'static Path = Path::new("assets");
}

pub struct FileLoader<'a> {
	ar: Option<ZipArchive<Cursor<&'a [u8]>>>,
}

impl<'a> FileLoader<'a> {
	pub fn new() -> Self {
		if cfg!(feature = "pack") {
			let buf_reader = Cursor::new(ZIP.unwrap());
			let ar = ZipArchive::new(buf_reader).unwrap();
			return Self { ar: Some(ar) };
		} else {
			return Self {
				ar: if let None = ZIP {
					None
				} else {
					unreachable!("ERROR, LOADED ZIP, NOT")
				},
			};
		}
	}

	pub fn load<P: AsRef<Path>>(&mut self, p: P) -> Option<Cursor<Vec<u8>>> {
		println!(
			"Loading {}",
			&Self::path_buf_as_zip_path(ASSETS_PATH.join(&p))
		);
		if cfg!(feature = "pack") {
			let ar = self.ar.as_mut().unwrap();
			if let Ok(mut f) = ar.by_name(&Self::path_buf_as_zip_path(ASSETS_PATH.join(p))) {
				let mut buf = Vec::new();
				f.read_to_end(&mut buf).expect("Error loading file");
				Some(Cursor::new(buf))
			} else {
				None
			}
		} else {
			if let Ok(mut f) = File::open(ASSETS_PATH.join(p)) {
				let mut buf = Vec::new();
				f.read_to_end(&mut buf).expect("Error loading file");
				Some(Cursor::new(buf))
			} else {
				None
			}
		}
	}

	fn path_buf_as_zip_path(p: PathBuf) -> String {
		let mut r = Vec::<String>::new();
		for s in p.into_iter() {
			r.push(s.to_str().unwrap().to_string());
		}
		r.join("/")
	}
}

pub fn depth_texture_to_color<C: GraphicsContext>(
	surface: &mut C,
	t: &Texture<Dim2, Depth32F>,
) -> Texture<Dim2, NormRGBA8UI> {
	let (width, height) = {
		let [w, h] = t.size();
		(w as usize, h as usize)
	};
	let mut texels: Vec<u8> = Vec::new();
	texels.resize(width * height * 4, 0_u8);
	let raw = t.get_raw_texels();
	for i in 0..height {
		for j in 0..width {
			let val = (raw[i * width + j]).round() as u8;
			print!("{:.2} ", val);
			texels[i * width * 4 + j * 4 + 0] = val;
			texels[i * width * 4 + j * 4 + 1] = val;
			texels[i * width * 4 + j * 4 + 2] = val;
			texels[i * width * 4 + j * 4 + 3] = 255;
			// A: This could be multiplied times the alpha multiplier
		}
		println!("");
	}

	let res_tex: Texture<Dim2, NormRGBA8UI> = Texture::new(
		surface,
		[width as u32, height as u32],
		0,
		Sampler::default(),
	)
	.expect("Error creating texture");

	res_tex
		.upload_raw(GenMipmaps::No, &texels)
		.expect("Error uploading texture");
	res_tex
}
