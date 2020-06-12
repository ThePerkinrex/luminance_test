use luminance::context::GraphicsContext;
use luminance::pipeline::{Pipeline, TessGate};
use luminance::pixel::{Depth32F, Floating, NormRGBA8UI, NormUnsigned, Pixel};
use luminance::shader::program::ProgramInterface;
use luminance::tess::{Mode as TessMode, Tess, TessBuilder, TessSliceIndex as _};
use luminance::texture::{Dim2, Texture};

use std::collections::HashMap;
use std::path::Path;

use super::{HudUniformInterface, Vertex, VertexPosition, VertexUV};

use super::super::text::{tex_from_string, Font};
use super::super::texture::TextureData;
use super::super::utils::*;
use super::super::TEXTURES_PATH;
// use super::super::renderer::{Renderable, HasDepth};

pub struct Entity {
	vao: Tess,
	tex_size: [u32; 2],
	scale: f32,
	pos: [i32; 2],
	depth: f32,
	uv_states: Option<HashMap<String, Vec<VertexUV>>>, // ID: [VertexUV]
}

impl Entity {
	#[allow(dead_code)]
	pub fn new<'p, C: GraphicsContext>(
		surface: &mut C,
		vertices: &'p [Vertex],
		indices: &'p [u8],
		tex_size: [u32; 2],
	) -> Self {
		let tess = TessBuilder::new(surface)
			.add_vertices(vertices)
			.set_indices(indices)
			.set_mode(TessMode::Triangle)
			.build()
			.unwrap();
		// println!("{},{}", width, height);
		return Self {
			vao: tess,
			tex_size,
			scale: 1.0,
			pos: [0, 0],
			depth: 0.0,
			uv_states: None,
		};
	}

	pub fn get_depth(&self) -> f32 {
		self.depth
	}

	pub fn render<C: GraphicsContext>(
		&self,
		pipeline: &Pipeline,
		iface: &ProgramInterface<'_, HudUniformInterface>,
		tess_gate: &mut TessGate<C>,
		size: &[u32; 2],
		tex: &Texture<Dim2, Depth32F>,
	) {
		let bound_tex = pipeline.bind_texture(tex);

		iface.tex_floating.update(&bound_tex);
		iface.size.update(size.clone().into());
		iface.pos.update(self.pos.into());
		iface.depth.update(self.depth.into());
		iface.scale.update(self.scale.into());
		iface.tex_size.update(self.tex_size.into());

		tess_gate.render(self.vao.slice(..));
	}
}
