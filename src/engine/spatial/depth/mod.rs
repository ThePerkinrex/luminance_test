use luminance::linear::{M33, M44};
use luminance::pipeline::BoundTexture;
use luminance::pixel::NormUnsigned;
use luminance::shader::program::Uniform;
use luminance::texture::Dim2;

use luminance_derive::{Semantics, UniformInterface, Vertex};

pub const VS_STR: &str = include_str!("shaders/vs.glsl");
pub const FS_STR: &str = include_str!("shaders/fs.glsl");

#[derive(UniformInterface)]
pub struct UniformInterface {
	#[uniform(unbound)]
	pub matrix: Uniform<M44>,
	#[uniform(unbound)]
	pub model: Uniform<M44>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Semantics)]
pub enum VertexSemantics {
	#[sem(name = "pos", repr = "[f32; 3]", wrapper = "VertexPosition")]
	Position,
}

#[derive(Vertex, Clone, Copy, Debug)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
	position: VertexPosition,
}
