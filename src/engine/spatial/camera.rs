use cgmath::{perspective, EuclideanSpace, Matrix4, Point3, Rad, Vector3};

const FOVY: Rad<f32> = Rad(std::f32::consts::PI / 2.);
const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 10.;

pub struct Camera {
	pub projection: Matrix4<f32>,
	pub view: Matrix4<f32>
}

impl Camera {
	pub fn new(size: [u32; 2]) -> Self {
		// TODO get data from args
		Self {
			projection: perspective(FOVY, size[0] as f32 / size[1] as f32, Z_NEAR, Z_FAR),
			view: Matrix4::<f32>::look_at(Point3::new(1., 2., 3.), Point3::origin(), Vector3::unit_y()),
		}
	}
}
