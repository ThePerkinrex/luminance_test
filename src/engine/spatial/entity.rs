use luminance::context::GraphicsContext;
use luminance::pipeline::{Pipeline, TessGate};
use luminance::shader::program::ProgramInterface;
use luminance::tess::{Tess, TessSliceIndex as _};

use cgmath::{EuclideanSpace, Matrix4, Point3, Rad, SquareMatrix};

use super::obj::{AsArray, Material, Obj};
use super::SpatialUniformInterface;

pub struct Entity {
	tess: Vec<(Tess, Material)>,
	pub pos: Point3<f32>,
	pub rot_x: Rad<f32>,
	pub rot_y: Rad<f32>,
	pub rot_z: Rad<f32>,
	pub scale: f32,
}

impl Entity {
	pub fn new<'p, C: GraphicsContext>(surface: &mut C, obj: Obj) -> Self {
		Self {
			tess: obj.to_tess(surface),
			pos: Point3::origin(),
			rot_x: Rad(0.),
			rot_y: Rad(0.),
			rot_z: Rad(0.),
			scale: 1.,
		}
	}

	fn model_matrix(&self) -> Matrix4<f32> {
		let s = Matrix4::from_scale(self.scale);
		let t = Matrix4::from_translation(Point3::<f32>::origin() - self.pos);
		let r = Matrix4::from_angle_x(self.rot_x)
			* Matrix4::from_angle_y(self.rot_y)
			* Matrix4::from_angle_z(self.rot_z);
		s * t * r
	}

	pub fn render<C: GraphicsContext>(
		&self,
		_pipeline: &Pipeline,
		iface: &ProgramInterface<'_, SpatialUniformInterface>,
		tess_gate: &mut TessGate<C>,
		_size: &[u32; 2],
	) {
		iface.model.update(self.model_matrix().into());
		iface
			.normal
			.update(self.model_matrix().invert().unwrap().into());
		for (mesh, material) in &self.tess {
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
	}
}
