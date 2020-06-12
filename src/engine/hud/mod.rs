use luminance::pipeline::BoundTexture;
use luminance::pixel::{Floating, NormUnsigned};
use luminance::shader::program::Uniform;
use luminance::texture::Dim2;

use luminance_derive::{Semantics, UniformInterface, Vertex};

mod depth_entity;
mod entity;
mod renderer;

pub use depth_entity::Entity as DepthEntity;
pub use entity::{Entity, EntityKind};
pub use renderer::Renderer;

#[derive(UniformInterface)]
pub struct HudUniformInterface {
	pos: Uniform<[i32; 2]>,
	depth: Uniform<f32>,
	scale: Uniform<f32>,
	size: Uniform<[u32; 2]>,
	#[uniform(unbound)]
	tex: Uniform<&'static BoundTexture<'static, Dim2, NormUnsigned>>,
	#[uniform(unbound)]
	tex_floating: Uniform<&'static BoundTexture<'static, Dim2, Floating>>,
	tex_size: Uniform<[u32; 2]>,
	depth_tex: Uniform<bool>,
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

	pub fn update_pos(&mut self, new_pos: VertexPosition) {
		self.position = new_pos
	}

	pub fn get_uv(&self) -> &VertexUV {
		&self.uv
	}

	pub fn get_pos(&self) -> &VertexPosition {
		&self.position
	}
}
