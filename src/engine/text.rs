use luminance::context::GraphicsContext;
use luminance::pixel::NormRGBA8UI;
use luminance::texture::{Dim2, GenMipmaps, Sampler, Texture};

use rusttype::{point, Font as RTFont, Scale};
use std::io::Read;

use lazy_static::lazy_static;

use std::cmp::PartialEq;
use std::path::Path;

use super::FileLoader;
use super::RgbaColor;
use super::FONTS_PATH;

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
    Italic,
}

#[derive(Debug, Clone)]
pub struct Font {
    name: String,
    weight: FontWeight,
    style: FontStyle,
    size: f32,
    color: RgbaColor,
}

impl Font {
    pub fn new<T: ToString>(name: T, weight: FontWeight, style: FontStyle, size: f32) -> Self {
        Self {
            name: name.to_string(),
            weight,
            style,
            size,
            color: RgbaColor::new(255, 255, 255, 255),
        }
    }

    #[allow(dead_code)]
    pub fn set_color(&mut self, color: RgbaColor) {
        self.color = color
    }

    pub fn name(&self) -> String {
        let weight = format!("{:?}", self.weight);
        let style = if self.style != FontStyle::Regular {
            format!("{:?}", self.style)
        } else {
            String::new()
        };
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
fn draw_map(map: Vec<Vec<u8>>) {
    print!("+");
    for _ in 0..map[0].len() {
        print!("-");
    }
    println!("+");
    for row in map.clone() {
        //let s = if ri == metrics.start_y { '+' } else { '|' };
        print!("|");
        for byte in row {
            if byte == 255 {
                print!("#");
            } else if byte > 100 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("|");
    }
    print!("+");
    for _ in 0..map[0].len() {
        print!("-");
    }
    println!("+");
}

pub fn tex_from_string<T: ToString, C: GraphicsContext>(
    file_loader: &mut FileLoader,
    surface: &mut C,
    name: T,
    font: &Font,
) -> Option<(Texture<Dim2, NormRGBA8UI>, [[u32; 2]; 4])> {
    let mut font_data = Vec::new();
    if let Some(mut f) = file_loader.load(FONTS_PATH.join(font.name())) {
        f.read_to_end(&mut font_data).expect("Error loading data");

        let rt_font = RTFont::try_from_bytes(font_data.as_slice())
            .expect("error constructing a Font from bytes");

        // Desired font pixel height
        let height: f32 = font.size; // to get 80 chars across (fits most terminals); adjust as desired
        let pixel_height = height.ceil() as usize;

        let scale = Scale {
            x: height,
            y: height,
        };

        // The origin of a line of text is at the baseline (roughly where
        // non-descending letters sit). We don't want to clip the text, so we shift
        // it down with an offset when laying it out. v_metrics.ascent is the
        // distance between the baseline and the highest edge of any glyph in
        // the font. That's enough to guarantee that there's no clipping.
        let v_metrics = rt_font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);

        // Glyphs to draw for "RustType". Feel free to try other strings.
        let glyphs: Vec<_> = rt_font.layout(&name.to_string(), scale, offset).collect();

        // Find the most visually pleasing width to display
        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as usize;

        // println!("width: {}, height: {}", width, pixel_height);

        // Rasterise directly into ASCII art.
        //let mut pixel_data = vec![b' '; width * pixel_height];
        let mut map: Vec<Vec<u8>> = Vec::new();
        let mut empty_row = Vec::new();
        empty_row.resize(width, 0);
        map.resize(pixel_height, empty_row);
        //let mapping = b"@%#x+=:-. "; // The approximation of greyscale
        //let mapping_scale = (mapping.len() - 1) as f32;
        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    // v should be in the range 0.0 to 1.0
                    //let i = ((v * -1. + 1.) * mapping_scale + 0.5) as usize;
                    // so something's wrong if you get $ in the output.
                    //let c = mapping.get(i).cloned().unwrap_or(b'$');
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    // There's still a possibility that the glyph clips the boundaries of the bitmap
                    if x >= 0 && x < width as i32 && y >= 0 && y < pixel_height as i32 {
                        let x = x as usize;
                        let y = y as usize;
                        //pixel_data[(x + y * width)] = c;
                        map[y][x] = (v * 255.) as u8
                    }
                })
            }
        }

        // draw_map(map.clone());

        // Print it out
        // let stdout = ::std::io::stdout();
        // let mut handle = stdout.lock();
        // for j in 0..pixel_height {
        // 	handle
        // 		.write_all(&pixel_data[j * width..(j + 1) * width])
        // 		.unwrap();
        // 	handle.write_all(b"\n").unwrap();
        // }

        let mut texels: Vec<u8> = Vec::new();
        texels.resize(map.len() * map[0].len() * 4, 0_u8);
        let height = map.len();
        let width = map[0].len();
        for i in 0..height {
            for j in 0..width {
                let val = map[i][j];
                texels[i * width * 4 + j * 4 + 0] = font.color.r; // R: This could be changed for the desired color
                texels[i * width * 4 + j * 4 + 1] = font.color.g; // G: This could be changed for the desired color
                texels[i * width * 4 + j * 4 + 2] = font.color.b; // B: This could be changed for the desired color
                texels[i * width * 4 + j * 4 + 3] = (font.color.a as f64 / 255. * val as f64) as u8;
                // A: This could be multiplied times the alpha multiplier
            }
        }

        let res_tex: Texture<Dim2, NormRGBA8UI> = Texture::new(
            surface,
            [width as u32, pixel_height as u32],
            0,
            Sampler::default(),
        )
        .expect("Error creating texture");

        res_tex
            .upload_raw(GenMipmaps::No, &texels)
            .expect("Error uploading texture");
        // 3 2
        // 0 1 (UV indices)
        return Some((
            res_tex,
            [
                [0, height as u32],
                [width as u32, height as u32],
                [width as u32, 0],
                [0, 0],
            ],
        ));
    }
    None
}
