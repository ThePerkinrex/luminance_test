use luminance::pipeline::{BoundTexture};
use luminance::pixel::NormUnsigned;
use luminance::shader::program::Uniform;
use luminance::texture::Dim2;

use luminance_derive::{UniformInterface, Vertex, Semantics};

mod entity;
mod renderer;

pub use entity::Entity;
pub use renderer::Renderer;


#[derive(UniformInterface)]
pub struct HudUniformInterface {
	pos: Uniform<[i32; 2]>,
	depth: Uniform<f32>,
	scale: Uniform<f32>,
	size: Uniform<[u32; 2]>,
	tex: Uniform<&'static BoundTexture<'static, Dim2, NormUnsigned>>,
	tex_size: Uniform<[u32; 2]>,
}

#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
	#[sem(name = "position", repr = "[i32; 2]", wrapper = "VertexPosition")]
	Position,
	#[sem(name = "uv", repr = "[u32; 2]", wrapper = "VertexUV")]
	UV,
}

#[derive(Vertex, Clone, Debug)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
	position: VertexPosition,
	uv: VertexUV,
}

#[allow(dead_code)]
impl Vertex {
	pub fn update_uv(&mut self, new_uv: VertexUV) {
		self.uv = new_uv
	}

	pub fn get_uv(&self) -> &VertexUV {
		&self.uv
	}

	pub fn get_pos(&self) -> &VertexPosition {
		&self.position
	}
}


