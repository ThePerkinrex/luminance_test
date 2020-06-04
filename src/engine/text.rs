use luminance::context::GraphicsContext;
use luminance::pixel::NormRGBA8UI;
use luminance::texture::{Dim2, GenMipmaps, Sampler, Texture};

use freetype as ft;

use lazy_static::lazy_static;

use std::cmp::{min, PartialEq};
use std::path::Path;

use super::ASSETS_PATH;

lazy_static! {
	static ref FONT: &'static Path = &Path::new("Roboto-Regular.ttf");
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontWeight {
	Black,
	Bold,
	Regular,
	Medium,
	Light,
	Thin,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontStyle {
	Regular,
	Italic
}

#[derive(Debug, Clone)]
pub struct Font {
	name: String,
	weight: FontWeight,
	style: FontStyle,
	size: u32,
}

impl Font {
	pub fn new<T: ToString>(name: T, weight: FontWeight, style: FontStyle, size: u32) -> Self {
		Self {
			name: name.to_string(),
			weight,
			style,
			size,
		}
	}

	pub fn name(&self) -> String {
		let weight = format!("{:?}", self.weight);
		let style = if self.style != FontStyle::Regular {format!("{:?}", self.style)} else {String::new()};
		format!("{}-{}{}.ttf", self.name.to_string(), weight, style)
	}
}

// https://www.freetype.org/freetype2/docs/tutorial/metrics.png
#[derive(Debug, Clone, Copy)]
pub struct GlyphMetrics {
	start_x: i64, // bearingX
	start_y: i64, // bearingY
	width: u64,
	height: u64,
	advance: i64,
}

#[allow(dead_code)] // Draw the char on stdout
fn draw_map(map: Vec<Vec<u8>>, metrics: GlyphMetrics) {
	let mut ri = 0;
	print!("+");
	for _ in 0..map[0].len() {
		print!("---");
	}
	println!("+");
	for row in map.clone() {
		let s = if ri == metrics.start_y { '+' } else { '|' };
		print!("{}", s);
		for byte in row {
			if byte == 255 {
				print!(" # ");
			} else if byte > 100 {
				print!(" * ");
			} else {
				print!("{}", if ri == metrics.start_y { "---" } else { "   " });
			}
		}
		println!("{}", s);
		ri += 1;
	}
	print!("+");
	for _ in 0..map[0].len() {
		print!("---");
	}
	println!("+");
}

fn raw_glyph(c: char, font: Font) -> Option<(GlyphMetrics, Vec<u8>)> {
	if let Ok(library) = ft::Library::init() {
		if let Ok(face) = library.new_face(ASSETS_PATH.join("fonts").join(font.name()), 0) {
			// println!("FONT LOADED");
			face.set_char_size(font.size as isize * 64, 0, font.size * 3, 0).expect("Error setting char size");
			face.load_char(c as usize, ft::face::LoadFlag::RENDER)
				.unwrap();
			// println!("face LOADED");

			let glyph = face.glyph();
			let bitmap = glyph.bitmap();
			let full_buffer = bitmap.buffer();

			let g_m = glyph.metrics();
			let metrics = GlyphMetrics {
				start_x: g_m.horiBearingX >> 6,
				start_y: g_m.horiBearingY >> 6,
				advance: g_m.horiAdvance >> 6,
				width: (g_m.width >> 6) as u64,
				height: (g_m.height >> 6) as u64,
			};

			let max_width = metrics.advance;
			let max_height = metrics.height;

			let mut map = Vec::new();

			for _ in 0..if max_height == 0 {
				// Make a 1 row map if max_height is 0
				1
			} else {
				max_height as usize
			} {
				let mut v = Vec::new();
				for _ in 0..max_width {
					v.push(0_u8);
				}
				map.push(v);
			}

			// Fill the map with the bitmap data
			for i in 0..metrics.height {
				for j in 0..metrics.width {
					//println!("LOADING VAL");
					let val = full_buffer[i as usize * metrics.width as usize + j as usize];
					//println!("LOADING INTO MAP: {} {}", i, metrics.start_x + j as i64);
					map[i as usize][min(
						metrics.start_x as usize + j as usize,
						max_width as usize - 1,
					)] = val;
				}
			}
			// println!("LOADED MAP");
			// Change the greyscale bitmap into an rgba bitmap
			let mut texels: Vec<u8> = Vec::new();
			texels.resize(map.len() * map[0].len() * 4, 0_u8);
			let height = map.len();
			let width = map[0].len();
			for i in 0..height {
				for j in 0..width {
					let val = map[i][j];
					texels[i * width * 4 + j * 4 + 0] = 255; // R: This could be changed for the desired color
					texels[i * width * 4 + j * 4 + 1] = 255; // G: This could be changed for the desired color
					texels[i * width * 4 + j * 4 + 2] = 255; // B: This could be changed for the desired color
					texels[i * width * 4 + j * 4 + 3] = val; // A: This could be multiplied times the alpha multiplier
				}
			}
			// draw_map(map.clone(), metrics);
			// println!("'{}': {:?} x:{} y:{} x_max:{} y_max:{}; {} {}", c, metrics, x, y, x_max, y_max, texels.len(), max_height * max_width as u64 * 4);

			return Some((metrics, texels));
		} else {
			eprintln!("Error creating face for char '{}'", c)
		}
	} else {
		eprintln!("Error initilaising FreeType2")
	}
	None
}

pub fn tex_from_string<'p, C: GraphicsContext>(
	surface: &mut C,
	s: String,
	font: Font,
) -> Option<(Texture<Dim2, NormRGBA8UI>, [[u32; 2]; 4])> {
	let mut max_top_height = 0_u32;
	let mut max_bottom_height = 0_u32;
	let mut max_width = 0_u32;
	let mut glyphs = Vec::new();
	for c in s.chars() {
		if let Some((metrics, glyph)) = raw_glyph(c, font.clone()) {
			if (max_top_height as i64) < metrics.start_y {
				max_top_height = metrics.start_y.abs() as u32
			}
			if (max_bottom_height as i64) < (metrics.height as i64 - metrics.start_y) {
				max_bottom_height = (metrics.height as i64 - metrics.start_y).abs() as u32
			}
			max_width += metrics.advance as u32;
			glyphs.push((metrics, glyph));
		} else {
			eprintln!("Error creating glyph for '{}'", c);
			return None;
		}
	}
	let res_tex: Result<Texture<Dim2, NormRGBA8UI>, _> = Texture::new(
		surface,
		[
			max_width as u32,
			(max_top_height + max_bottom_height) as u32,
		],
		0,
		Sampler::default(),
	);
	if let Ok(tex) = res_tex {
		let mut offset = 0;
		for (metrics, texels) in glyphs {
			let size = [metrics.advance as u32, metrics.height as u32];
			if let Err(e) = tex.upload_part_raw(
				GenMipmaps::No,
				[offset, (max_top_height as i64 - metrics.start_y) as u32],
				size,
				&texels,
			) {
				eprintln!("Error uploading part: {}", e);
				return None;
			}
			let mut padding_top = Vec::new();
			padding_top.resize(
				(size[0] * 4 * ((((max_top_height) as i64) - metrics.start_y) as u32)) as usize,
				0_u8,
			);
			if let Err(e) = tex.upload_part_raw(
				GenMipmaps::No,
				[offset, 0],
				[
					size[0],
					(((max_top_height) as i64) - metrics.start_y) as u32,
				],
				&padding_top,
			) {
				eprintln!("Error uploading part top padding: {}", e);
				return None;
			}
			let mut padding_bottom = Vec::new();
			padding_bottom.resize(
				(size[0]
					* 4 * (max_bottom_height - (metrics.height as i64 - metrics.start_y) as u32))
					as usize,
				0_u8,
			);
			if let Err(e) = tex.upload_part_raw(
				GenMipmaps::No,
				[
					offset,
					((metrics.height as i64 - metrics.start_y) as u32 + max_top_height),
				],
				[
					size[0],
					max_bottom_height - (metrics.height as i64 - metrics.start_y) as u32,
				],
				&padding_bottom,
			) {
				eprintln!("Error uploading part bottom padding: {}", e);
				return None;
			}
			offset += metrics.advance as u32;
		}
		let [width, height] = tex.size();
		// 3 2
		// 0 1 (UV indices)
		return Some((tex, [
			[0, height as u32],
			[width as u32, height as u32],
			[width as u32, 0],
			[0, 0],
		]));
	} else {
		eprintln!("Error initialising texture for string \"{}\"", s)
	}
	None
}
