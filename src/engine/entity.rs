use luminance::context::GraphicsContext;
use luminance::pipeline::{Pipeline, TessGate};
use luminance::pixel::NormRGBA8UI;
use luminance::shader::program::ProgramInterface;
use luminance::tess::{Mode as TessMode, Tess, TessBuilder, TessSliceIndex as _};
use luminance::texture::{Dim2, Texture};
use luminance_derive::{Semantics, Vertex};

use serde::Deserialize;

use ron::de::from_reader;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use super::utils::*;
use super::ShaderInterface;
use super::ASSETS_PATH;

#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
	#[sem(name = "position", repr = "[i32; 2]", wrapper = "VertexPosition")]
	Position,
	#[sem(name = "uv", repr = "[u32; 2]", wrapper = "VertexUV")]
	UV,
}

#[derive(Vertex, Clone, Debug)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
	position: VertexPosition,
	uv: VertexUV,
}

#[allow(dead_code)]
impl Vertex {
	pub fn update_uv(&mut self, new_uv: VertexUV) {
		self.uv = new_uv
	}

	pub fn get_uv(&self) -> &VertexUV {
		&self.uv
	}

	pub fn get_pos(&self) -> &VertexPosition {
		&self.position
	}
}

pub trait Renderable {
	fn render<C: GraphicsContext>(
		&self,
		pipeline: &Pipeline,
		iface: &ProgramInterface<'_, ShaderInterface>,
		tess_gate: &mut TessGate<C>,
		size: &[u32; 2],
	);
}

pub struct SimpleEntity {
	vao: Tess,
	tex: Texture<Dim2, NormRGBA8UI>,
	tex_size: [u32; 2],
	scale: f32,
	pos: [i32; 2],
	depth: f32,
}

impl SimpleEntity {
	pub fn new<'p, C: GraphicsContext>(
		surface: &mut C,
		vertices: &'p [Vertex],
		indices: &'p [u8],
		path: &Path,
	) -> Option<SimpleEntity> {
		if let Some(img) = read_image(path) {
			return Some(Self::new_from_img(surface, vertices, indices, img))
		}
		None
	}

	pub fn new_from_img<'p, C: GraphicsContext>(
		surface: &mut C,
		vertices: &'p [Vertex],
		indices: &'p [u8],
		img: image::RgbaImage,
	) -> SimpleEntity {
		let tex = load_from_disk(surface, img);
		return Self::new_from_tex(surface, vertices, indices, tex)
	}

	pub fn new_from_tex<'p, C: GraphicsContext>(
		surface: &mut C,
		vertices: &'p [Vertex],
		indices: &'p [u8],
		tex: Texture<Dim2, NormRGBA8UI>,
	) -> SimpleEntity {
		let tess = TessBuilder::new(surface)
				.add_vertices(vertices)
				.set_indices(indices)
				.set_mode(TessMode::Triangle)
				.build()
				.unwrap();
			// println!("{},{}", width, height);
			let size = tex.size();
			return SimpleEntity {
				vao: tess,
				tex: tex,
				tex_size: size,
				scale: 1.0,
				pos: [0, 0],
				depth: 0.0,
			};
	}

	#[allow(dead_code)]
	pub fn set_pos(&mut self, new_pos: [i32; 2]) {
		self.pos = new_pos
	}

	#[allow(dead_code)]
	pub fn set_scale(&mut self, new_scale: f32) {
		self.scale = new_scale
	}

	#[allow(dead_code)]
	pub fn set_depth(&mut self, new_depth: f32) {
		self.depth = new_depth
	}

	#[allow(dead_code)]
	pub fn update_uv(&mut self, new_uv: &[VertexUV]) {
		let mut v_slice = self
			.vao
			.as_slice_mut::<Vertex>()
			.expect("Error getting mutablee slice");
		for i in 0..v_slice.len() {
			v_slice[i].update_uv(new_uv[i])
		}
	}

	#[allow(dead_code)]
	pub fn print_uv(&mut self) {
		let v_slice = self
			.vao
			.as_slice_mut::<Vertex>()
			.expect("Error getting mutablee slice");
		for i in 0..v_slice.len() {
			println!(
				"{} {}",
				v_slice[i].get_uv()[0] as f32 / self.tex_size[0] as f32,
				v_slice[i].get_uv()[1] as f32 / self.tex_size[1] as f32
			);
			let position = v_slice[i].get_pos();
			let pos = [0, 0];
			let scale = 0.5;
			let size = [1000, 1000];
			println!(
				"- {} {}: {} {}",
				position[0],
				position[1],
				((position[0] as f32 * scale + pos[0] as f32) / (size[0]) as f32) * 2. - 1.,
				((position[1] as f32 * scale + pos[1] as f32) / (size[1]) as f32) * 2. - 1.
			);
		}
	}
}

impl Renderable for SimpleEntity {
	fn render<C: GraphicsContext>(
		&self,
		pipeline: &Pipeline,
		iface: &ProgramInterface<'_, ShaderInterface>,
		tess_gate: &mut TessGate<C>,
		size: &[u32; 2],
	) {
		let bound_tex = pipeline.bind_texture(&self.tex);

		iface.tex.update(&bound_tex);
		iface.size.update(size.clone().into());
		iface.pos.update(self.pos.into());
		iface.depth.update(self.depth.into());
		iface.scale.update(self.scale.into());
		iface.tex_size.update(self.tex_size.into());

		tess_gate.render(self.vao.slice(..));
	}
}

#[derive(Debug, Deserialize)]
pub struct TextureData {
	file: String,
	default_uv: String,
	uv: HashMap<String, Vec<(u32, u32)>>,
}

pub struct DynamicStateTexEntity {
	simple_entity: SimpleEntity,
	uv_states: HashMap<String, Vec<VertexUV>>, // ID: [VertexUV]
}

impl DynamicStateTexEntity {
	pub fn new<'p, C: GraphicsContext>(
		surface: &mut C,
		vertices_pos: &'p [VertexPosition],
		indices: &'p [u8],
		tex_data: TextureData,
	) -> Self {
		let mut vertices = Vec::new();
		let def_uv = tex_data.uv.get(&tex_data.default_uv).expect(&format!(
			"Default UV is not valid for texture: {}",
			tex_data.file
		));
		for i in 0..vertices_pos.len() {
			let uv = VertexUV::new([def_uv[i].0, def_uv[i].1]);
			vertices.push(Vertex::new(vertices_pos[i], uv));
		}
		let mut uv_states = HashMap::new();
		for key in tex_data.uv.keys() {
			let mut uvs = Vec::new();
			for v in tex_data.uv.get(key).unwrap() {
				uvs.push(VertexUV::new([v.0, v.1]))
			}
			uv_states.insert(key.clone(), uvs);
		}
		Self {
			simple_entity: SimpleEntity::new(
				surface,
				&vertices,
				indices,
				ASSETS_PATH.join(&tex_data.file).as_ref(),
			)
			.expect(&format!(
				"Error creating simple entity for texture (IMAGE NOT FOUND): {:?}",
				ASSETS_PATH.join(&tex_data.file)
			)),
			uv_states,
		}
	}

	pub fn load<'p, C: GraphicsContext>(
		surface: &mut C,
		vertices_pos: &'p [VertexPosition],
		indices: &'p [u8],
		file: &Path,
	) -> Self {
		let ron_path = ASSETS_PATH.join(file);
		println!("Opening {:?}", ron_path);
		let f = File::open(ron_path).expect("Error opening RON Texture file. Path might be wrong?");

		let tex_data: TextureData = match from_reader(f) {
			Ok(x) => x,
			Err(e) => panic!("Can't load texture data: {}", e),
		};
		Self::new(surface, vertices_pos, indices, tex_data)
	}

	#[allow(dead_code)]
	pub fn set_pos(&mut self, new_pos: [i32; 2]) {
		self.simple_entity.set_pos(new_pos)
	}

	#[allow(dead_code)]
	pub fn set_scale(&mut self, new_scale: f32) {
		self.simple_entity.set_scale(new_scale)
	}

	#[allow(dead_code)]
	pub fn set_depth(&mut self, new_depth: f32) {
		self.simple_entity.set_depth(new_depth)
	}

	#[allow(dead_code)]
	pub fn state_ids(&self) -> Vec<&String> {
		self.uv_states.keys().collect()
	}

	#[allow(dead_code)]
	pub fn set_state<T: ToString>(&mut self, id: T) -> Result<(), ()> {
		if let Some(res) = self.uv_states.get(&id.to_string()) {
			self.simple_entity.update_uv(&res);
			return Ok(());
		}
		return Err(());
	}
}

impl Renderable for DynamicStateTexEntity {
	fn render<C: GraphicsContext>(
		&self,
		pipeline: &Pipeline,
		iface: &ProgramInterface<'_, ShaderInterface>,
		tess_gate: &mut TessGate<C>,
		size: &[u32; 2],
	) {
		self.simple_entity.render(pipeline, iface, tess_gate, size)
	}
}
