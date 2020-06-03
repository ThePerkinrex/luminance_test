use luminance::context::GraphicsContext;
use luminance::pipeline::{BoundTexture, TessGate};
use luminance::pixel::NormUnsigned;
use luminance::shader::program::Uniform;
use luminance::texture::Dim2;
use luminance_derive::UniformInterface;

use lazy_static::lazy_static;

use std::path::Path;

pub mod entity;
mod utils;
pub mod text;

lazy_static! {
	pub static ref ASSETS_PATH: &'static Path = Path::new("assets");
}

#[derive(UniformInterface)]
pub struct ShaderInterface {
	pos: Uniform<[i32; 2]>,
	depth: Uniform<f32>,
	scale: Uniform<f32>,
	size: Uniform<[u32; 2]>,
	tex: Uniform<&'static BoundTexture<'static, Dim2, NormUnsigned>>,
	tex_size: Uniform<[u32; 2]>,
}

pub trait Game {
	fn init<C: GraphicsContext>(surface: C);
	fn update<C: GraphicsContext>(surface: C);
	fn render<C: GraphicsContext>(tess_gate: luminance::pipeline::TessGate<'_, C>);
}

pub struct GameEngine<C: GraphicsContext, G: Game> {
	surface: C,
	game: G,
}
