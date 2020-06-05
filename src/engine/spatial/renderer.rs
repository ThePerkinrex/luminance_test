use luminance::context::GraphicsContext;
use luminance::pipeline::{Pipeline, ShadingGate};
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::blending::{Equation, Factor};
use luminance::tess::{Tess, TessSliceIndex};

use std::path::Path;

use super::{VertexSemantics, SpatialUniformInterface, camera::Camera, obj::{Obj, Material, AsArray}};
use super::super::ASSETS_PATH;

const VS_STR: &str = include_str!("shaders/vs.glsl");
const FS_STR: &str = include_str!("shaders/fs.glsl");

pub struct Renderer {
	program: Program<VertexSemantics, (), SpatialUniformInterface>,
	render_st: RenderState,
	camera: Camera,
	mesh: Vec<(Tess, Material)>,
}

impl Renderer {
	pub fn new<C: GraphicsContext>(surface: &mut C, size: [u32; 2]) -> Self {
		let program: Program<VertexSemantics, (), SpatialUniformInterface> =
		Program::from_strings(None, VS_STR, None, FS_STR)
			.expect("Error loading spatial shaders")
			.ignore_warnings();
		let render_st = RenderState::default().set_blending((
			Equation::Additive,
			Factor::SrcAlpha,
			Factor::SrcAlphaComplement,
		));
		Self {
			program,
			render_st,
			camera: Camera::new(size),
			mesh: Obj::load(Path::new("test2.obj")).unwrap().to_tess(surface)
		}
	}

	pub fn render<C: GraphicsContext>(
		&self,
		shd_gate: &mut ShadingGate<'_, C>,
		pipeline: &Pipeline,
		size: &[u32; 2],
	) {
		shd_gate.shade(&self.program, |iface, mut rdr_gate| {
			iface.projection.update(self.camera.projection.into());
			iface.view.update(self.camera.view.into());
	  
			rdr_gate.render(&self.render_st, |mut tess_gate| {
				for (mesh, material) in &self.mesh {
					iface.obj_color_diffuse.update(material.color_diffuse.as_array().into());
					iface.obj_color_ambient.update(material.color_ambient.as_array().into());
					tess_gate.render(mesh.slice(..));

				}
			});
		  });
	}
}