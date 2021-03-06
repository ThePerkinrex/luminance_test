use luminance::context::GraphicsContext;
use luminance::pipeline::{Pipeline, TessGate};
use luminance::pixel::{Floating, NormRGBA8UI, NormUnsigned, Pixel};
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

pub enum EntityKind {
	Regular,
	Text,
}

pub struct Entity {
	vao: Tess,
	tex: Texture<Dim2, NormRGBA8UI>,
	tex_size: [u32; 2],
	scale: f32,
	pos: [i32; 2],
	depth: f32,
	uv_states: Option<HashMap<String, Vec<VertexUV>>>, // ID: [VertexUV]
}

impl Entity {
	#[allow(dead_code)]
	pub fn new<'p, C: GraphicsContext>(
		file_loader: &mut FileLoader,
		surface: &mut C,
		vertices: &'p [Vertex],
		indices: &'p [u8],
		path: &Path,
	) -> Option<Self> {
		if let Some(img) = read_image(file_loader, path) {
			return Some(Self::new_from_img(surface, vertices, indices, img));
		}
		None
	}

	#[allow(dead_code)]
	pub fn new_from_img<'p, C: GraphicsContext>(
		surface: &mut C,
		vertices: &'p [Vertex],
		indices: &'p [u8],
		img: image::RgbaImage,
	) -> Self {
		let tex = load_from_disk(surface, img);
		return Self::new_from_tex(surface, vertices, indices, tex);
	}

	#[allow(dead_code)]
	pub fn new_with_texture_data<'p, C: GraphicsContext>(
		file_loader: &mut FileLoader,
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
		let mut ret = Self::new(
			file_loader,
			surface,
			&vertices,
			indices,
			TEXTURES_PATH.join(&tex_data.file).as_ref(),
		)
		.expect(&format!(
			"Error creating simple entity for texture (IMAGE NOT FOUND): {:?}",
			TEXTURES_PATH.join(&tex_data.file)
		));

		ret.uv_states = Some(uv_states);
		ret
	}

	#[allow(dead_code)]
	pub fn load<'p, C: GraphicsContext>(
		file_loader: &mut FileLoader,
		surface: &mut C,
		vertices_pos: &'p [VertexPosition],
		indices: &'p [u8],
		file: &Path,
	) -> Option<Self> {
		match TextureData::load(file_loader, file) {
			Some(x) => Some(Self::new_with_texture_data(
				file_loader,
				surface,
				vertices_pos,
				indices,
				x,
			)),
			None => None,
		}
	}

	pub fn new_entity_from_string<'p, C: GraphicsContext>(
		file_loader: &mut FileLoader,
		surface: &mut C,
		s: String,
		font: &Font,
	) -> Option<Self> {
		if let Some((tex, uvs)) = tex_from_string(file_loader, surface, s.clone(), font) {
			let [width, height] = tex.size();
			return Some(Self::new_from_tex(
				surface,
				&[
					Vertex::new(VertexPosition::new([0, 0]), VertexUV::new(uvs[0])),
					Vertex::new(
						VertexPosition::new([width as i32, 0]),
						VertexUV::new(uvs[1]),
					),
					Vertex::new(
						VertexPosition::new([width as i32, height as i32]),
						VertexUV::new(uvs[2]),
					),
					Vertex::new(
						VertexPosition::new([0, height as i32]),
						VertexUV::new(uvs[3]),
					),
				],
				&[0, 1, 2, 0, 2, 3],
				tex,
			));
		} else {
			eprintln!("Error creting texture for string \"{}\"", s)
		}
		None
	}

	#[allow(dead_code)]
	pub fn new_from_tex<'p, C: GraphicsContext>(
		surface: &mut C,
		vertices: &'p [Vertex],
		indices: &'p [u8],
		tex: Texture<Dim2, NormRGBA8UI>,
	) -> Self {
		let tess = TessBuilder::new(surface)
			.add_vertices(vertices)
			.set_indices(indices)
			.set_mode(TessMode::Triangle)
			.build()
			.unwrap();
		// println!("{},{}", width, height);
		let size = tex.size();
		return Self {
			vao: tess,
			tex: tex,
			tex_size: size,
			scale: 1.0,
			pos: [0, 0],
			depth: 0.0,
			uv_states: None,
		};
	}

	pub fn set_pos(&mut self, new_pos: [i32; 2]) {
		self.pos = new_pos
	}

	pub fn set_scale(&mut self, new_scale: f32) {
		self.scale = new_scale
	}

	pub fn set_depth(&mut self, new_depth: f32) {
		self.depth = new_depth
	}

	pub fn get_depth(&self) -> f32 {
		self.depth
	}

	pub fn state_ids(&self) -> Vec<&String> {
		if let Some(uv_states) = &self.uv_states {
			uv_states.keys().collect()
		} else {
			Vec::new()
		}
	}

	pub fn set_state<S: ToString>(&mut self, id: S) -> Result<(), ()> {
		if let Some(uv_states) = self.uv_states.clone() {
			if let Some(res) = uv_states.get(&id.to_string()) {
				self.update_uv(&res);
				return Ok(());
			}
		}
		return Err(());
	}

	pub fn update_uv(&mut self, new_uv: &[VertexUV]) {
		let mut v_slice = self
			.vao
			.as_slice_mut::<Vertex>()
			.expect("Error getting mutablee slice");
		for i in 0..v_slice.len() {
			v_slice[i].update_uv(new_uv[i])
		}
	}

	pub fn update_text<T: ToString, C: GraphicsContext>(
		&mut self,
		file_loader: &mut FileLoader,
		surface: &mut C,
		text: &T,
		font: &Font,
	) -> Result<(), ()> {
		//if let EntityKind::Text = self.kind {
		if let Some((tex, uvs)) = tex_from_string(file_loader, surface, text.to_string(), font) {
			let [width, height] = tex.size();
			self.tex_size = tex.size();
			self.tex = tex;
			self.update(&[
				(VertexPosition::new([0, 0]), VertexUV::new(uvs[0])),
				(
					VertexPosition::new([width as i32, 0]),
					VertexUV::new(uvs[1]),
				),
				(
					VertexPosition::new([width as i32, height as i32]),
					VertexUV::new(uvs[2]),
				),
				(
					VertexPosition::new([0, height as i32]),
					VertexUV::new(uvs[3]),
				),
			]);
			return Ok(());
		} else {
			eprintln!("Error creating texture for string \"{}\"", text.to_string())
		}
		//}
		Err(())
	}

	pub fn update_pos(&mut self, new_pos: &[VertexPosition]) {
		let mut v_slice = self
			.vao
			.as_slice_mut::<Vertex>()
			.expect("Error getting mutablee slice");
		for i in 0..v_slice.len() {
			v_slice[i].update_pos(new_pos[i])
		}
	}

	pub fn update(&mut self, new_v: &[(VertexPosition, VertexUV)]) {
		let mut v_slice = self
			.vao
			.as_slice_mut::<Vertex>()
			.expect("Error getting mutablee slice");
		for i in 0..v_slice.len() {
			v_slice[i].update_pos(new_v[i].0);
			v_slice[i].update_uv(new_v[i].1);
		}
	}

	pub fn update_tex(&mut self, tex: Texture<Dim2, NormRGBA8UI>) {
		self.tex = tex;
		self.tex_size = self.tex.size();
	}

	pub fn render<C: GraphicsContext>(
		&self,
		pipeline: &Pipeline,
		iface: &ProgramInterface<'_, HudUniformInterface>,
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
