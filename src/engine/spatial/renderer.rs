use luminance::blending::{Equation, Factor};
use luminance::context::GraphicsContext;
use luminance::pipeline::{Pipeline, ShadingGate};
use luminance::render_state::RenderState;
use luminance::shader::program::Program;

use cgmath::Rad;

use std::path::Path;

use super::super::FileLoader;
use super::{camera::Camera, entity::Entity, obj::Obj, SpatialUniformInterface, VertexSemantics};

const VS_STR: &str = include_str!("shaders/vs.glsl");
const FS_STR: &str = include_str!("shaders/fs.glsl");

pub struct Renderer {
	program: Program<VertexSemantics, (), SpatialUniformInterface>,
	render_st: RenderState,
	pub camera: Camera,
	mesh: Entity, //Vec<(Tess, Material)>,
}

impl Renderer {
	pub fn new<C: GraphicsContext>(
		file_loader: &mut FileLoader,
		surface: &mut C,
		size: [u32; 2],
	) -> Self {
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
			mesh: Entity::new(
				surface,
				Obj::load(file_loader, Path::new("test2.obj")).unwrap(),
			),
		}
	}

	pub fn render<C: GraphicsContext>(
		&mut self,
		shd_gate: &mut ShadingGate<'_, C>,
		pipeline: &Pipeline,
		size: &[u32; 2],
	) {
		self.camera.update_surface_size(size.clone());
		shd_gate.shade(&self.program, |iface, mut rdr_gate| {
			iface.projection.update(self.camera.projection.into());
			iface.view.update(self.camera.view.into());
			iface.view_pos.update(self.camera.pos.into());

			rdr_gate.render(&self.render_st, |mut tess_gate| {
				self.mesh.render(pipeline, &iface, &mut tess_gate, size)
			});
		});
		self.mesh.rot_x += Rad(0.01).into();
		self.mesh.rot_y += Rad(0.01).into();
		// self.mesh.pos += Vector3::new(0.,0.01,0.);
		self.mesh.scale += 0.01;
	}
}
