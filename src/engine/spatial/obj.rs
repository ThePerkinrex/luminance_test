use luminance::context::GraphicsContext;
use luminance::tess::{Mode as TessMode, Tess, TessBuilder};

use wavefront_obj::{mtl, obj};

use std::collections::HashMap;
use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use super::super::{FileLoader, MODELS_PATH};
use super::{Vertex, VertexIndex, VertexNormal, VertexPosition};

pub type Material = mtl::Material;

pub trait AsArray<T> {
	fn as_array(&self) -> T;
}

impl AsArray<[f32; 3]> for mtl::Color {
	fn as_array(&self) -> [f32; 3] {
		[self.r as f32, self.g as f32, self.b as f32]
	}
}

pub fn new_material<T: ToString>(
	name: T,
	specular_coefficient: f64,
	color_ambient: mtl::Color,
	color_diffuse: mtl::Color,
	color_specular: mtl::Color,
	illumination: mtl::Illumination,
) -> Material {
	Material {
		name: name.to_string(),
		specular_coefficient,
		color_ambient,
		color_diffuse,
		color_specular,
		color_emissive: None,
		optical_density: None,
		alpha: 1.0,
		illumination,
		uv_map: None,
	}
}

pub fn color_material<T: ToString>(name: T, color: mtl::Color) -> Material {
	new_material(
		name,
		300.0,
		mtl::Color {
			r: 1.0,
			g: 1.0,
			b: 1.0,
		},
		color,
		mtl::Color {
			r: 0.5,
			g: 0.5,
			b: 0.5,
		},
		mtl::Illumination::AmbientDiffuseSpecular,
	)
}

#[derive(Debug)]
pub struct Geometry {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<VertexIndex>,
	pub material: Material,
}

#[derive(Debug)]
pub struct Obj {
	pub geometries: Vec<Geometry>,
}

impl Obj {
	pub fn to_tess<C>(self, ctx: &mut C) -> Vec<(Tess, Material)>
	where
		C: GraphicsContext,
	{
		let mut res = Vec::new();
		for geo in self.geometries {
			if let Ok(t) = TessBuilder::new(ctx)
				.set_mode(TessMode::Triangle)
				.add_vertices(geo.vertices)
				.set_indices(geo.indices)
				.build()
			{
				res.push((t, geo.material));
			}
		}

		return res;
	}

	pub fn load<P>(file_loader: &mut FileLoader, path: P) -> Result<Self, String>
	where
		P: AsRef<Path>,
	{
		let file_content = {
			let mut file = file_loader
				.load(MODELS_PATH.join(&path))
				.or_else(|| {
					panic!("Can't open file: {:?}", MODELS_PATH.join(path));
				})
				.unwrap();
			let mut content = String::new();
			file.read_to_string(&mut content).unwrap();
			content
		};
		let obj_set = obj::parse(file_content).map_err(|e| format!("cannot parse: {:?}", e))?;

		let mtl = if let Some(mtl_lib) = obj_set.material_library {
			let mut file = file_loader
				.load(MODELS_PATH.join(&mtl_lib))
				.or_else(|| {
					panic!("Can't open file: {:?}", MODELS_PATH.join(mtl_lib));
				})
				.unwrap();
			let mut content = String::new();
			file.read_to_string(&mut content).unwrap();
			Some(mtl::parse(content).map_err(|e| format!("cannot parse: {:?}", e))?)
		} else {
			None
		};

		println!("{:?}", mtl);
		let objects = obj_set.objects;

		assert_eq!(objects.len(), 1, "expecting a single object");

		let object = objects.into_iter().next().unwrap();

		// verify!(object.geometry.len() == 1).ok_or("expecting a single geometry".to_owned())?;
		let mut geos = Vec::new();
		for geometry in object.geometry {
			let g_material = if let Some(mtl_lib) = &mtl {
				if let Some(mtl_name) = geometry.material_name {
					let mut res = None;
					for material in &mtl_lib.materials {
						if material.name == mtl_name {
							res = Some(material.clone())
						}
					}
					res
				} else {
					None
				}
			} else {
				None
			}
			.unwrap_or(Material {
				name: "Default".into(),
				specular_coefficient: 225.0,
				color_ambient: mtl::Color {
					r: 1.0,
					g: 1.0,
					b: 1.0,
				},
				color_diffuse: mtl::Color {
					r: 0.001174,
					g: 0.0,
					b: 0.8,
				},
				color_specular: mtl::Color {
					r: 0.5,
					g: 0.5,
					b: 0.5,
				},
				color_emissive: Some(mtl::Color {
					r: 0.0,
					g: 0.0,
					b: 0.0,
				}),
				optical_density: Some(1.45),
				alpha: 1.0,
				illumination: mtl::Illumination::AmbientDiffuseSpecular,
				uv_map: None,
			});

			println!("loading {}", object.name);
			println!("{} vertices", object.vertices.len());
			println!("{} shapes", geometry.shapes.len());

			// build up vertices; for this to work, we remove duplicated vertices by putting them in a
			// map associating the vertex with its ID
			let mut vertex_cache: HashMap<obj::VTNIndex, VertexIndex> = HashMap::new();
			let mut vertices: Vec<Vertex> = Vec::new();
			let mut indices: Vec<VertexIndex> = Vec::new();

			for shape in geometry.shapes {
				if let obj::Primitive::Triangle(a, b, c) = shape.primitive {
					for key in &[a, b, c] {
						if let Some(vertex_index) = vertex_cache.get(key) {
							indices.push(*vertex_index);
						} else {
							let p = object.vertices[key.0];
							let n = object.normals
								[key.2.ok_or("missing normal for a vertex".to_owned())?];
							let position =
								VertexPosition::new([p.x as f32, p.y as f32, p.z as f32]);
							let normal = VertexNormal::new([n.x as f32, n.y as f32, n.z as f32]);
							let vertex = Vertex { position, normal };
							let vertex_index = vertices.len() as VertexIndex;

							vertex_cache.insert(*key, vertex_index);
							vertices.push(vertex);
							indices.push(vertex_index);
						}
					}
				} else {
					return Err("unsupported non-triangle shape".to_owned());
				}
			}

			geos.push(Geometry {
				vertices,
				indices,
				material: g_material,
			})
		}
		Ok(Obj { geometries: geos })
	}
}
