use luminance::blending::{Equation, Factor};
use luminance::context::GraphicsContext;
use luminance::pipeline::{Pipeline, ShadingGate};
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::{Tess, TessSliceIndex};

use cgmath::{Matrix, Matrix3, Matrix4, Rad, SquareMatrix};

use std::path::Path;

use super::super::FileLoader;
use super::{
	camera::Camera,
	obj::{AsArray, Material, Obj},
	SpatialUniformInterface, VertexSemantics,
};

const VS_STR: &str = include_str!("shaders/vs.glsl");
const FS_STR: &str = include_str!("shaders/fs.glsl");

pub struct Renderer {
	program: Program<VertexSemantics, (), SpatialUniformInterface>,
	render_st: RenderState,
	pub camera: Camera,
	mesh: Vec<(Tess, Material)>,
	rotation: Rad<f32>,
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
			mesh: Obj::load(file_loader, Path::new("test2.obj"))
				.unwrap()
				.to_tess(surface),
			rotation: Rad(0.0),
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
			let model = Matrix4::from_scale(1.); // * Matrix4::from_angle_x(self.rotation);
			iface.model.update(model.clone().into());
			iface.normal.update(model.clone().invert().unwrap().into());
			iface.view_pos.update(self.camera.pos.into());

			rdr_gate.render(&self.render_st, |mut tess_gate| {
				for (mesh, material) in &self.mesh {
					iface
						.obj_color_diffuse
						.update(material.color_diffuse.as_array().into());
					iface
						.obj_color_specular
						.update(material.color_specular.as_array().into());
					iface
						.obj_specular_coefficient
						.update(material.specular_coefficient as f32);
					tess_gate.render(mesh.slice(..));
				}
			});
		});

		self.rotation += Rad(0.01);
	}
}
