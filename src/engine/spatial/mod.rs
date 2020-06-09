use luminance::linear::{M33, M44};
use luminance::pipeline::BoundTexture;
use luminance::pixel::NormUnsigned;
use luminance::shader::program::Uniform;
use luminance::texture::Dim2;

use luminance_derive::{Semantics, UniformInterface, Vertex};

pub mod camera;
mod entity;
pub mod obj;
mod renderer;

//pub use entity::Entity;
pub use renderer::Renderer;

#[derive(UniformInterface)]
pub struct SpatialUniformInterface {
	#[uniform(unbound)]
	projection: Uniform<M44>,
	#[uniform(unbound)]
	view: Uniform<M44>,
	#[uniform(unbound)]
	model: Uniform<M44>,
	#[uniform(unbound, name = "normal_m")]
	normal: Uniform<M44>,
	obj_color_diffuse: Uniform<[f32; 3]>,
	obj_color_specular: Uniform<[f32; 3]>,
	obj_specular_coefficient: Uniform<f32>,
	view_pos: Uniform<[f32; 3]>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Semantics)]
pub enum VertexSemantics {
	#[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
	Position,
	#[sem(name = "normal", repr = "[f32; 3]", wrapper = "VertexNormal")]
	Normal,
}

#[derive(Vertex, Clone, Copy, Debug)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
	position: VertexPosition,
	normal: VertexNormal,
}

type VertexIndex = u32;
