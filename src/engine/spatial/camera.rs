use cgmath::{
	perspective, EuclideanSpace, Euler, InnerSpace, Matrix4, Point3, Quaternion, Rad, Rotation,
	Rotation3, Vector3,
};

const FOVY: Rad<f32> = Rad(std::f32::consts::PI / 2.);
const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 100.;

pub struct Camera {
	pub projection: Matrix4<f32>,
	pub view: Matrix4<f32>,
	pub pos: Point3<f32>,
	pub dir: Vector3<f32>,
	pub rot: Quaternion<f32>,
}

impl Camera {
	pub fn new(size: [u32; 2]) -> Self {
		// TODO get data from args
		let eye_pos = Point3::new(2., 2., 3.);
		let look_at = Point3::origin();
		let look_dir = look_at - eye_pos;

		Self {
			projection: perspective(FOVY, size[0] as f32 / size[1] as f32, Z_NEAR, Z_FAR),
			view: Matrix4::<f32>::look_at(eye_pos, look_at, Vector3::unit_y()),
			pos: eye_pos,
			dir: look_dir,
			rot: Quaternion::look_at(look_dir, Vector3::unit_y()),
		}
	}

	pub fn update_dir(&mut self) {
		self.dir = self.rot.rotate_vector(Vector3::unit_z());
		dbg!(self.rot);
	}

	// pub fn change_pos_dir(&mut self, position: Point3<f32>, look_dir: Vector3<f32>) {
	// 	self.view = Matrix4::look_at_dir(position, look_dir, Vector3::unit_y());
	// 	self.pos = position;
	// 	self.dir = look_dir;
	// }

	// pub fn change_pos_at(&mut self, position: Point3<f32>, look_at: Point3<f32>) {
	// 	self.view = Matrix4::look_at(position, look_at, Vector3::unit_y());
	// 	self.pos = position;
	// 	self.dir = look_at - position;
	// }

	pub fn update(&mut self) {
		self.view = Matrix4::look_at_dir(self.pos, self.dir, Vector3::unit_y());
	}

	pub fn look_at(&mut self, at: Point3<f32>) {
		self.dir = at - self.pos;
		self.update()
	}

	// pub fn look_at_dir(&mut self, dir: Vector3<f32>) {
	// 	self.dir = dir;
	// 	self.update()
	// }

	pub fn update_surface_size(&mut self, size: [u32; 2]) {
		self.projection = perspective(FOVY, size[0] as f32 / size[1] as f32, Z_NEAR, Z_FAR);
	}
}
