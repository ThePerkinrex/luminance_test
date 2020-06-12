use luminance::blending::{Equation, Factor};
use luminance::context::GraphicsContext;
use luminance::pipeline::{Pipeline, ShadingGate};
use luminance::pixel::Depth32F;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::texture::{Dim2, Texture};

use std::cmp::Ordering;

use super::super::EntityRegistry;
use super::{DepthEntity, Entity, HudUniformInterface, VertexSemantics};

const VS_STR: &str = include_str!("shaders/vs.glsl");
const FS_STR: &str = include_str!("shaders/fs.glsl");

pub struct Renderer {
	program: Program<VertexSemantics, (), HudUniformInterface>,
	render_st: RenderState,
}

impl Renderer {
	pub fn new() -> Self {
		let program: Program<VertexSemantics, (), HudUniformInterface> =
			Program::from_strings(None, VS_STR, None, FS_STR)
				.expect("Error loading HUD shaders")
				.ignore_warnings();
		let render_st = RenderState::default().set_blending((
			Equation::Additive,
			Factor::SrcAlpha,
			Factor::SrcAlphaComplement,
		));
		Self { program, render_st }
	}

	pub fn render<C: GraphicsContext>(
		&self,
		registry: &EntityRegistry<Entity>,
		shd_gate: &mut ShadingGate<'_, C>,
		pipeline: &Pipeline,
		size: &[u32; 2],
		d: &DepthEntity,
		d_tex: &Texture<Dim2, Depth32F>,
	) {
		let mut ordered = registry.values();
		ordered.sort_by(|x, y| {
			x.get_depth()
				.partial_cmp(&y.get_depth())
				.unwrap_or(Ordering::Equal)
				.reverse()
		});
		shd_gate.shade(&self.program, |iface, mut rdr_gate| {
			rdr_gate.render(&self.render_st, |mut tess_gate| {
				iface.depth_tex.update(true);
				d.render(&pipeline, &iface, &mut tess_gate, &size, d_tex);
				for e in ordered {
					iface.depth_tex.update(false);
					e.render(&pipeline, &iface, &mut tess_gate, &size);
				}
			})
		});
	}
}
